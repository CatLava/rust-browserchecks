use actix_web::web;
mod add;
mod show;
mod utils;
use super::path::Path;

pub fn bfp_factory(app: &mut web::ServiceConfig) {
    let base_path = Path{prefix: "/bfp".to_string()};
    app.route(&base_path.define("/add".to_string()), 
        web::post().to(add::add_browser_info))
        .route(&base_path.define("/show".to_string()), 
        web::get().to(show::show));
}