use crate::game::asset_server::asset_reference::AssetReference;
use crate::tests::mock::mock_default_service_provider;
use std::path::PathBuf;

#[test]
fn test_load() {
    let sp = mock_default_service_provider();

    let test_image_path = PathBuf::from("./example_data/assets/test.png");
    let test_reference = AssetReference::new_png("test", &test_image_path);
    sp.asset_server().put_reference(test_reference);

    let image_data = std::fs::read(&test_image_path).unwrap();
    let asset = sp.asset_server().get_asset("test").unwrap().unwrap();

    assert_eq!(image_data, *asset.as_png().unwrap().get_data());
}
