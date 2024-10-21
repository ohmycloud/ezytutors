use actix_web::web;
use crate::handlers::{get_courses_for_tutor, health_check_handler, new_course};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor)),
    );
}