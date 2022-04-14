use actix_web::rt::spawn;

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = spawn(server);
}

#[actix_web::test]
async fn health_check_works() {
    spawn_app();
}