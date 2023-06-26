//! # Book Archive
//!
//! ## Overview
//!
//! This pallet allows users to create an archive record for a book.
//! Only one record can be created for a specific book
//!
//! ## Interface
//!
//! ### Config
//!
//! ### Dispatchable functions
//!
//! * `archive_book(orgin, title, author, url, archiver, timestamp)` - Archive a specified book
//! * `pay_royalty ( hash(title + author) )` - Pay royalty to archiver
//!
//! ### RPC query endpoints
//!
//! * `book_summary( hash(title + author) )` - Retrieve book summary from the archive
//!

#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::inherent::Vec;
    use frame_support::pallet_prelude::*;
    use frame_support::sp_runtime::traits::Hash;
    use frame_support::traits::{Currency, ReservableCurrency};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::format;
    use sp_runtime::traits::AccountIdConversion;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Trait for handling fungible tokens
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
        /// Pallet ID.
        #[pallet::constant]
        type PalletId: Get<PalletId>;
    }

    // Pallet derived account id
    #[pallet::storage]
    pub(super) type PalletAccountId<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    /// Book summary
    #[derive(Clone, Encode, Decode, Default, Eq, PartialEq, RuntimeDebug, TypeInfo)]
    pub struct BookSummary<AccountId, BlockNumber> {
        pub title: Vec<u8>,     // title of book
        pub author: Vec<u8>,    // author of book
        pub url: Vec<u8>,       // web url to off-chain storage
        pub royalty_paid: u32,  // amount of token paid as royalty
        archiver: AccountId,    // account id of archiver
        timestamp: BlockNumber, // time when book was archived
    }

    /// Archive storage
    ///
    /// Maps a book hash to book summary
    /// Book hash is Blake2 hash of book title and author
    #[pallet::storage]
    #[pallet::getter(fn book_summary)]
    pub(super) type ArchiveStore<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        BookSummary<T::AccountId, T::BlockNumber>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when a book is archived
        BookArchived { who: T::AccountId },
        /// Event emitted when royalty is paid to book archiver
        RoyaltyReserved {
            sender: T::AccountId,
            receiver: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Book already exist in archive
        BookAlreadyExistInArchive,
        /// Book does not exist in archive
        BookDoesNotExistInArchive,
    }

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100_000_000)]
        #[pallet::call_index(1)]
        pub fn archive_book(
            origin: OriginFor<T>,
            title: Vec<u8>,
            author: Vec<u8>,
            url: Vec<u8>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            let signer = ensure_signed(origin)?;

            let title = title.to_ascii_lowercase();
            let author = author.to_ascii_lowercase();

            // Create book pre-signature
            let pre_image = format!("{:?}{:?}", title, author,);

            // Get book hash
            let book_hash = T::Hashing::hash(&pre_image.as_bytes());

            // Verify that title and author have not already been stored
            ensure!(
                !ArchiveStore::<T>::contains_key(&book_hash),
                Error::<T>::BookAlreadyExistInArchive
            );

            // Get the block number from the FRAME System pallet.
            let current_block = <frame_system::Pallet<T>>::block_number();

            // Create specified book summary
            let book_summary = BookSummary {
                title,
                author,
                url,
                royalty_paid: 0,
                archiver: signer.clone(),
                timestamp: current_block,
            };

            // Store book summary in archive
            ArchiveStore::<T>::insert(&book_hash, book_summary);

            // Emit an event that the book was archived.
            Self::deposit_event(Event::BookArchived { who: signer });

            Ok(())
        }

        #[pallet::weight(100_000_000)]
        #[pallet::call_index(2)]
        pub fn pay_royalty(
            origin: OriginFor<T>,
            title: Vec<u8>,
            author: Vec<u8>,
            deposit: u32,
        ) -> DispatchResult {
            // Extrinsic must be signed by payee
            let signer = ensure_signed(origin)?;

            // royalty must be greater than zero
            assert!(deposit > 0, "Error: Deposit must be greater than zero");

            let title = title.to_ascii_lowercase();
            let author = author.to_ascii_lowercase();

            // Compute book hash key
            let pre_image = format!("{:?}{:?}", title, author);
            let book_hash = T::Hashing::hash(&pre_image.as_bytes());

            // Verify that book exist in archive
            ensure!(
                ArchiveStore::<T>::contains_key(&book_hash),
                Error::<T>::BookDoesNotExistInArchive
            );

            // Get book summary from archive
            let book_summary = ArchiveStore::<T>::get(&book_hash).unwrap();

            // Update royalty for book
            let book_summary = BookSummary {
                title: book_summary.title,
                author: book_summary.author,
                url: book_summary.url,
                royalty_paid: book_summary.royalty_paid.checked_add(deposit).unwrap(),
                archiver: book_summary.archiver,
                timestamp: book_summary.timestamp,
            };

            // Store book summary in archive
            ArchiveStore::<T>::insert(&book_hash, book_summary.clone());

            // reserve deposit for archiver
            // initialize pallet account id if account id is not initialized
            match PalletAccountId::<T>::get() {
                Some(_) => {
                    // reserve deposit
                    T::Currency::reserve(&signer, deposit.into())
                        .expect("Error: Failed to reserve royalty for archiver");
                }
                None => {
                    //
                    Self::pallet_account_id();
                    // reserve deposit
                    T::Currency::reserve(&signer, deposit.into())
                        .expect("Error: Failed to reserve royalty for archiver");
                }
            }

            // Emit an event that the book was archived.
            Self::deposit_event(Event::RoyaltyReserved {
                sender: signer,
                receiver: book_summary.archiver,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        // generate pallet account id
        pub fn pallet_account_id() -> T::AccountId {
            let pallet_account_id: T::AccountId = T::PalletId::get().into_account_truncating();
            PalletAccountId::<T>::put(pallet_account_id.clone());
            pallet_account_id
        }
    }
}
