#![forbid(missing_docs)]

//! This projects serves to enable automatic rendering of `openapi.json` files, and provides
//! facilities to also serve the documentation alongside your api.
//!
//! # Usage
//!
//! First, add the following lines to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! schemars = "0.8"
//! serde = "1.0"
//! okapi = { version = "0.6.1", package = "okapi_fork" }
//! rocket_okapi = { version = "0.8.0-rc.1", package = "rocket_okapi_fork" }
//! ## Add rocket_okapi_ui if you want do embedd Swagger UI
//! rocket_okapi_ui = "0.1.0-rc.1"
//! ```
//!
//! To add documentation to a set of endpoints, a couple of steps are required:
//! - The request and response types of the endpoint must implement
//!   [JsonSchema](schemars::JsonSchema).
//! - Route handlers must be marked with [`#[openapi]`](openapi).
//! - After that, you can simply replace [routes!](rocket::routes!) with
//!   [routes_with_openapi!]. This will append an additional route to the
//!   resulting [Vec]<[Route](rocket::Route)>, which contains the `openapi.json`
//!   file that is required by swagger.
//!
//! To serve [Swagger UI](https://swagger.io/tools/swagger-ui/) directly from
//! your Rocket application additional steps are required:
//! - Add the `rocket_okapi_ui` dependency to your `Cargo.toml`
//! - Attach the [SwaggerUIConfig](rocket_okapi_ui::SwaggerUIConfig) fairing to Rocket.
//! - Mount the Swagger UI routes created with [swagger_ui_routes![]](rocket_okapi_ui::swagger_ui_routes!).
//!
//! Now you should be able to load the example in the browser!
//!
//! ### Example
//! ```rust
//! #[macro_use] extern crate rocket;
//! #[macro_use] extern crate rocket_okapi;
//! #[macro_use] extern crate rocket_okapi_ui;
//!
//! use rocket::serde::json::Json;
//! use rocket_okapi::JsonSchema;
//! use rocket_okapi_ui::{SwaggerUIConfig, UrlObject};
//! use serde::Serialize;
//!
//! #[derive(Serialize, JsonSchema)]
//! struct Response {
//!     reply: String,
//! }
//!
//! #[openapi]
//! #[get("/")]
//! fn my_controller() -> Json<Response> {
//!     Json(Response {
//!         reply: "show me the docs!".to_string(),
//!     })
//! }
//!
//! fn get_docs() -> SwaggerUIConfig {
//!     SwaggerUIConfig {
//!         urls: vec![UrlObject::new("API v1", "/api/v1/openapi.json")],
//!         ..Default::default()
//!     }
//! }
//!
//! #[launch]
//! fn rocket() -> _ {
//!     rocket::build()
//!         .attach(get_docs().fairing())
//!         .mount("/api/v1", routes_with_openapi![my_controller])
//!         .mount("/swagger",     swagger_ui_routes![])
//! }
//! ```

mod error;

/// Contains the `Generator` struct, which you can use to manually control the way a struct is
/// represented in the documentation.
pub mod gen;
/// Contains several `Rocket` `Handler`s, which are used for serving the json files and the swagger
/// interface.
pub mod handlers;
/// This module contains several traits that correspond to the `Rocket` traits pertaining to request
/// guards and responses
pub mod request;
/// Contains the trait `OpenApiResponder`, meaning that a response implementing this trait can be
/// documented.
pub mod response;
/// Contains then `OpenApiSettings` struct, which can be used to customise the behaviour of a
/// `Generator`.
pub mod settings;
/// Assorted function that are used throughout the application.
pub mod util;

pub use error::*;

extern crate rocket_okapi_codegen;

pub use rocket_okapi_codegen::*;
pub use schemars::JsonSchema;

/// Contains information about an endpoint.
pub struct OperationInfo {
    /// The path of the endpoint
    pub path: String,
    /// The HTTP Method of this endpoint.
    pub method: rocket::http::Method,
    /// Contains information to be showed in the documentation about this endpoint.
    pub operation: okapi::openapi3::Operation,
}
