// factory for views of all views
use actix_web::web;
mod path;
mod auth;
mod bfp;

pub fn views_factory(app: &mut web::ServiceConfig) {
    auth::auth_factory(app);
    bfp::bfp_factory(app);
}