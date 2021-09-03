use rocket::response::{self, Redirect, Responder};
use rocket::Request;

/// A handler that instead of serving content always redirects to some specified destination URL.
#[derive(Clone)]
pub struct RedirectHandler<'r> {
    dest: &'r str,
}

impl<'r> RedirectHandler<'r> {
    /// Create a new `RedirectHandler` that redirects to the specified URL.
    #[must_use]
    pub fn to(dest: &'r str) -> Self {
        Self {
            dest: dest.trim_start_matches('/'),
        }
    }
}

impl<'r, 'o : 'r> Responder<'r, 'o> for RedirectHandler<'r> {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        let path = request
            .route()
            .unwrap()
            .uri
            .base()
            .trim_end_matches('/');
        Redirect::to(format!("{}/{}", path, self.dest))
            .respond_to(request)
    }
}
