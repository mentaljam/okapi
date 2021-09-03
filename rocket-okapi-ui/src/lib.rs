#![forbid(missing_docs)]

//! This crate contains structs and macro for embedding Swagger UI
//! into Rocket applications.

/// Contains [SwaggerUIConfig] and some other structs.
pub mod config;

mod redirect;

pub use crate::config::{SwaggerUIConfig, UrlObject};

use crate::redirect::RedirectHandler;

use rocket::serde::json::Json;
use rocket::http::{ContentType, Status};
use rocket::get;

macro_rules! swagger_static_files {
    ($file:ident, $($name:literal => $type:ident),*) => (
        match $file {
            $(
                $name => (
                    Status::Ok,
                    (ContentType::$type, include_bytes!(concat!("../swagger-ui/", $name)))
                ),
            )*
            _ => (Status::NotFound, (ContentType::Plain, &[]))
        }
    );
}

/// Route for Swagger UI configuration file
#[get("/swagger-ui-config.json")]
pub fn swagger_ui_config(
    // config: &SwaggerUIConfig,
    config: &rocket::State<SwaggerUIConfig>,
) -> Json<&SwaggerUIConfig> {
    Json(config.inner())
}

/// Route for Swagger static files
#[get("/<file>")]
pub fn swagger_ui_static(
    file: &str,
) -> (Status, (ContentType, &'static [u8])) {
    swagger_static_files!(file,
        "favicon-16x16.png"               => PNG,
        "favicon-32x32.png"               => PNG,
        "index.html"                      => HTML,
        "oauth2-redirect.html"            => HTML,
        "swagger-ui.js"                   => JavaScript,
        "swagger-ui-standalone-preset.js" => JavaScript,
        "swagger-ui-bundle.js"            => JavaScript,
        "swagger-ui.css"                  => CSS
    )
}

/// Redirects `/<swagger-ui-base>/` to `/<swagger-ui-base>/index.html`
#[get("/")]
pub fn swagger_ui_redirect<'r>() -> RedirectHandler<'r> {
    RedirectHandler::to("index.html")
}

/// Create Rocket routes for Swagger UI
#[macro_export]
macro_rules! swagger_ui_routes {
    [] => {
        rocket::routes![
            rocket_okapi_ui::swagger_ui_config,
            rocket_okapi_ui::swagger_ui_static,
            rocket_okapi_ui::swagger_ui_redirect,
        ]
    };
}
