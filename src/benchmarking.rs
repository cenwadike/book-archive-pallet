//! Benchmarking setup for pallet-archiver
#![cfg(feature = "runtime-benchmarks")]

use crate::pallet::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_support::dispatch::EncodeLike;
use frame_system::RawOrigin;
use sp_core::Blake2Hasher;
use sp_core::Hasher;
use sp_runtime::testing::H256;

benchmarks! {
    where_clause {
        where H256: EncodeLike<<T as frame_system::Config>::Hash>
    }

    archive_book {
        let caller: T::AccountId = whitelisted_caller();
        let title: Vec<u8> = "title".into();
        let author: Vec<u8> = "author".into();
        let url: Vec<u8> = "url".into();
    }: _(RawOrigin::Signed(caller),title.clone(), author.clone(), url.clone())
    verify {
        let data = format!("{:?}{:?}", title, author);
        let hash = Blake2Hasher::hash(data.as_bytes());

        let stored_book_summary = Pallet::<T>::book_summary(hash).unwrap();
        assert_eq!(stored_book_summary.url, url);
    }

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
