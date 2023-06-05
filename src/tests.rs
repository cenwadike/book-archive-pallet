use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn archive_book_works() {
    new_test_env().execute_with(|| {
        let title: Vec<u8> = "title".into();
        let author: Vec<u8> = "author".into();
        let url: Vec<u8> = "url".into();

        // Dispatch a signed extrinsic to store book in archive
        assert_ok!(TemplateModule::archive_book(
            RuntimeOrigin::signed(1),
            title.clone(),
            author.clone(),
            url.clone(),
        ));
    });
}
