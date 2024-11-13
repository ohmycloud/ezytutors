use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tutor {
    tutor_id: i32,
    tutor_name: String,
    tutor_pic_url: String,
    tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(tutor: web::Json<NewTutor>) -> Self {
        Self {
            tutor_name: tutor.tutor_name.clone(),
            tutor_pic_url: tutor.tutor_pic_url.clone(),
            tutor_profile: tutor.tutor_profile.clone(),
        }
    }
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(tutor: web::Json<UpdateTutor>) -> Self {
        Self {
            tutor_name: tutor.tutor_name.clone(),
            tutor_pic_url: tutor.tutor_pic_url.clone(),
            tutor_profile: tutor.tutor_profile.clone(),
        }
    }
}
