#[tokio::test]
#[ignore]
async fn bootstrap_then_refresh_detects_changes() {
    // Requires docker-compose postgres up
    let cfg = feytra_core::Config::from_env();
    let v1 = feytra_core::jobs::execute_bootstrap(&cfg).await.unwrap();
    assert!(v1 >= 1);
    let r2 = feytra_core::jobs::execute_refresh(&cfg).await.unwrap();
    assert!(r2.version >= v1);
}


