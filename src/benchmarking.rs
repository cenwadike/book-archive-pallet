#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::Pallet as ArchiverPallet;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn archive_book() {
        let title: Vec<u8> = "title".into();
        let author: Vec<u8> = "author".into();
        let url: Vec<u8> = "url".into();
        let archiver: T::AccountId = whitelisted_caller();
        #[extrinsic_call]
        archive_book(
            RawOrigin::Signed(archiver),
            title.clone(),
            author.clone(),
            url.clone(),
        );
    }

    impl_benchmark_test_suite!(
        ArchiverPallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}
