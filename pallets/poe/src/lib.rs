
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Import Rust Macros from frame_support library
#[frame_support::pallet]
pub mod pallet
{
    use frame_support::pallet_prelude::*;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_system::Account;
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    //Define Runtime Configuration Trait
    #[pallet::config]
    pub trait Config: frame_system::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    //Declaration of the Pallet type
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    //Runtime Storage
    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>;

    //Runtime Events
    #[pallet::event]
    //#[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>
    {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        TransferCreated(T::AccountId, Vec<u8>),
    }

    //Error
    #[pallet::error]
    pub enum Error<T>
    {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
    }

    //Hooks
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    //Extrinsics
    #[pallet::call]
    impl<T: Config> Pallet<T>
    {
        #[pallet::weight(10_000)]
        pub fn create_claim(origin: OriginFor<T>, proof: Vec<u8>,) -> DispatchResultWithPostInfo
        {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not already been claimed.
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (sender.clone(), current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>,) -> DispatchResultWithPostInfo
        {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed.
            // ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim.
            let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage.
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased.
            Self::deposit_event(Event::ClaimRevoked(sender, proof));

            Ok(().into())
        }

        // Create transfer claim of proof function which contains Account ID and the content of the proof
        #[pallet::weight(10_000)]
        pub fn transfer_claim(origin: OriginFor<T>, receiver: T::AccountId, proof: Vec<u8>,) -> DispatchResultWithPostInfo
        {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            let sender = ensure_signed(origin)?;

            // Get Owner and verify that the specified proof has been claimed.
            let (owner, _) = Proofs::<T>::get(&proof).ok_or(Error::<T>::NoSuchProof)?;

            // Verify that sender of the current call is the claim owner.
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Store the proof with the sender and block number.
            Proofs::<T>::insert(&proof, (receiver.clone(), current_block));

            // Emit an event that the claim was created.
            Self::deposit_event(Event::TransferCreated(receiver, proof));

            Ok(().into())
        }
    }
}