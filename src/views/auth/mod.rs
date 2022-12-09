// factory for view
use actix_web::web;
mod login;
mod logout;
mod signup;
use super::path::Path;

pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: "/auth".to_string()};
    app.route(&base_path.define("/login".to_string()), 
        web::get().to(login::login))
        .route(&base_path.define("/logout".to_string()), 
        web::get().to(logout::logout));
}

