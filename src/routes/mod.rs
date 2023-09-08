use actix_web::web::ServiceConfig;

pub mod root;
pub mod users;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(root::req);
    cfg.service(users::get::req);
    cfg.service(users::create::req);
    cfg.service(users::delete::req);
}
