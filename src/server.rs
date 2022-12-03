use rocket::{get, post, response::content::Html, routes, Config};
use rocket_contrib::json::Json;

use crate::{clipboard::write_clipboard, transmitter::ClipboardTask};

#[get("/")]
pub fn index() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>Clipboard Sync</title>
            </head>
            <body>
                <h1>Clipboard Sync</h1>
                <p>Nothing to see here</p>
            </body>
        </html>
    "#,
    )
}

#[post("/clipboard", format = "json", data = "<task>")]
pub fn clipboard(task: Json<ClipboardTask>) -> &'static str {
    println!("Received clipboard contents from {}", task.sender);

    write_clipboard(task.text.clone());

    "OK"
}

pub fn start_server(port: u16) {
    let server_config = Config::build(rocket::config::Environment::Production)
        .address("localhost")
        .port(port)
        .log_level(rocket::config::LoggingLevel::Off)
        .finalize()
        .expect("Failed to build server config");

    println!("Starting server on http://localhost:{}", port);

    rocket::custom(server_config)
        .mount("/", routes![index, clipboard])
        .launch();
}
