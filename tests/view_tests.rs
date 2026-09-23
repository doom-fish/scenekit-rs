use scenekit::View;

#[test]
fn views_are_refused_off_the_main_thread() {
    let result = std::thread::spawn(|| View::new(80.0, 60.0).map(drop)).join();
    let error = result
        .expect("thread")
        .expect_err("SCNView must not be created off the main thread");
    assert!(error.to_string().contains("main thread"), "{error}");
}
