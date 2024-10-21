use actix_web::web;
use actix_web::web::Json;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: u32,
    pub course_id: Option<u32>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    // Convert data from incoming HTTP requests to Rust structs
    fn from(course: Json<Course>) -> Self {
        Self {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}