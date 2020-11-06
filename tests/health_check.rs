// `actix_rt::test` is the testing equivalent of `actix_web:main`.
// It also spares you from having to specify the `#[test]` attribute.
// You can inspect the code by using `cargo expand --test health_check` (<-name of the test file)
#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    spawn_app();
    // We brought `reqwest` in as a _development _dependency
    // to perform HTTP requests against our application.
    // Either add it manually under [dev-dependencies] in Cargo.toml
    // or run `cargo add reqwest --dev`
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute reqwest.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the backend.
// This is the only piece that will, reasonably, depend on our application code.
fn spawn_app() {
    // New dev dependency - let's add tokio to the party with
    // `cargo add tokio --dev --vers 0.2.22`
    let server = zero2prod::run().expect("Failed to bind address");

    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it, hence the non-binding let
    let _ = tokio::spawn(server);
}
