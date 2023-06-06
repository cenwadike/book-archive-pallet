use crate::mock::*;
use frame_support::traits::schedule::MaybeHashed;
use frame_support::Blake2_256;
use frame_support::{assert_ok, traits::Hash};

fn archive_book(title: &str, author: &str, url: &str, archiver: u64) -> Hash {
    let title = title.to_lowercase().as_bytes().to_vec();
    let author = author.to_lowercase().as_bytes().to_vec();
    let url = url.to_lowercase().as_bytes().to_vec();

    assert_ok!(TemplateModule::archive_book(
        RuntimeOrigin::signed(archiver),
        title.clone(),
        author.clone(),
        url,
    ));

    // Create book pre-signature
    let pre_image = format!("{:?}{:?}", title, author,);
    // Return book hash
    // let hash = Hash::from_slice(&pre_image.as_bytes());
    let hash = Hash::from_slice(&pre_image.as_str().as_bytes().to_vec());

    hash.into()
}

#[test]
fn archive_book_works() {
    new_test_env().execute_with(|| {
        let title: Vec<u8> = "title".into();
        let author: Vec<u8> = "author".into();
        let url: Vec<u8> = "url".into();

        assert_ok!(TemplateModule::archive_book(
            RuntimeOrigin::signed(1),
            title.clone(),
            author.clone(),
            url.clone(),
        ));

        let pre_image: String = format!("{:?}{:?}", title, author);
        let digest: &[u8] = pre_image.as_bytes().into();
        let hash: Hash = Hash::from_slice(&digest.to_vec()).into();

        let stored_book_summary = TemplateModule::archive_store(hash).unwrap();
        assert_eq!(stored_book_summary.url, url);
    });
}
