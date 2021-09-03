use crate::handlers::RedirectHandler;
use serde::{Deserialize, Serialize};

use rocket::fairing::{AdHoc, Fairing};
use rocket::serde::json::Json;
use rocket::http::{ContentType, Status};
use rocket::get;


/// Used to control the way models are displayed by default.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DefaultModelRendering {
    /// Expand the `example` section.
    Example,
    /// Expand the `model` section.
    Model,
}

/// Used to control the default expansion setting for the operations and tags.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DocExpansion {
    /// Expands only the tags.
    List,
    /// Expands the tags and operations
    Full,
    /// Expands nothing
    None,
}

/// Used to enable, disable and preconfigure filtering
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Filter {
    /// Use this variant to enable or disable filtering.
    Bool(bool),
    /// Use this variant to enable filtering, and preconfigure a filter.
    Str(String),
}

fn is_zero(num: &u32) -> bool {
    *num == 0
}

/// A struct containing information about where and how the `openapi.json` files are served.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerUIConfig {
    /// The url to a single `openapi.json` file that is showed when the web ui is first opened.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
    /// A list of named urls that contain all the `openapi.json` files that you want to display in
    /// your web ui. If this field is populated, the `url` field is not used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub urls: Vec<UrlObject>,
    // display options:
    /// If set to true, enables deep linking for tags and operations. See the
    /// [Deep Linking documentation](https://github.com/swagger-api/swagger-ui/blob/master/docs/usage/deep-linking.md)
    /// for more information.
    /// Default: `false`.
    pub deep_linking: bool,
    /// Controls the display of operationId in operations list.
    /// Default: `false`.
    pub display_operation_id: bool,
    /// The default expansion depth for models (set to -1 completely hide the models).
    /// Default: `1`.
    pub default_models_expand_depth: i32,
    /// The default expansion depth for the model on the model-example section.
    /// Default: `1`.
    pub default_model_expand_depth: i32,
    /// Controls how the model is shown when the API is first rendered. (The user can always switch
    /// the rendering for a given model by clicking the 'Model' and 'Example Value' links.)
    /// Default: `DefaultModelRendering::Example`.
    pub default_model_rendering: DefaultModelRendering,
    /// Controls the display of the request duration (in milliseconds) for "Try it out" requests.
    /// Default: `false`.
    pub display_request_duration: bool,
    /// Controls the default expansion setting for the operations and tags.
    /// Default: `DocExpansion::List`.
    pub doc_expansion: DocExpansion,
    /// If set, enables filtering. The top bar will show an edit box that you can use to filter the
    /// tagged operations that are shown. Filtering is case sensitive matching the filter expression
    /// anywhere inside the tag.
    /// Default: `Filter(false)`.
    pub filter: Filter,
    /// If set, limits the number of tagged operations displayed to at most this many. The default
    /// is to show all operations.
    /// Default: `None` (displays all tagged operations).
    #[serde(default, skip_serializing_if = "is_zero")]
    pub max_displayed_tags: u32,
    /// Controls the display of vendor extension (`x-`) fields and values for Operations,
    /// Parameters, and Schema.
    /// Default: `false`.
    pub show_extensions: bool,
    /// Controls the display of extensions (`pattern`, `maxLength`, `minLength`, `maximum`,
    /// `minimum`) fields and values for Parameters.
    /// Default: `false`.
    pub show_common_extensions: bool,
}

impl Default for SwaggerUIConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            urls: vec![],
            deep_linking: false,
            display_operation_id: false,
            default_model_expand_depth: 1,
            default_model_rendering: DefaultModelRendering::Example,
            default_models_expand_depth: 1,
            display_request_duration: false,
            doc_expansion: DocExpansion::List,
            filter: Filter::Bool(false),
            max_displayed_tags: 0,
            show_extensions: false,
            show_common_extensions: false,
        }
    }
}

impl SwaggerUIConfig {
    /// Fairing for loading Swagger configuration from Rocket figment
    pub fn fairing(self) -> impl Fairing {
        AdHoc::try_on_ignite("SwaggerUIConfig", move |rocket| async move {
            Ok(rocket.manage(self))
        })
    }
}

/// Contains a named url.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UrlObject {
    /// The name of the url.
    pub name: String,
    /// The url itself.
    pub url: String,
}

impl UrlObject {
    /// Create a new `UrlObject` from the provided name and url.
    #[must_use]
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}

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
            rocket_okapi::swagger_ui::swagger_ui_config,
            rocket_okapi::swagger_ui::swagger_ui_static,
            rocket_okapi::swagger_ui::swagger_ui_redirect,
        ]
    };
}
