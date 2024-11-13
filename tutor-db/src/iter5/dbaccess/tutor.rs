use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, Tutor, UpdateTutor};
use sqlx::postgres::PgPool;

pub async fn get_all_tutors_db(pool: &PgPool) -> Result<Vec<Tutor>, EzyTutorError> {
    // Prepare SQL statement
    let tutor_rows =
        sqlx::query!("SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile FROM ezy_tutor_c6")
            .fetch_all(pool)
            .await?;
    // Extract result
    let tutors: Vec<Tutor> = tutor_rows
        .iter()
        .map(|row| Tutor {
            tutor_id: row.tutor_id,
            tutor_name: row.tutor_name,
            tutor_pic_url: row.tutor_pic_url,
            tutor_profile: row.tutor_profile,
        })
        .collect();
    match tutors.len() {
        0 => Err(EzyTutorError::NotFound("No routes found".into())),
        _ => Ok(tutors),
    }
}

pub async fn get_tutor_details_db(pool: &PgPool, tutor_id: i32) -> Result<Tutor, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!(
        "SELECT tutor_id, tutor_name, tutor_pic_url, tutor_profile
        FROM ezy_tutor_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map(|row| Tutor {
        tutor_id: row.tutor_id,
        tutor_name: row.tutor_name,
        tutor_pic_url: row.tutor_pic_url,
        tutor_profile: row.tutor_profile,
    })
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;
    Ok(tutor_row)
}

pub async fn post_new_tutor_db(pool: &PgPool, tutor: NewTutor) -> Result<Tutor, EzyTutorError> {
    let row = sqlx::query!(
        "insert into ezy_tutor_c6(
        tutor_name, tutor_pic_url, tutor_profile) values ($1, $2, $3)
        returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        turor.tutor.name,
        tutor.tutor_pic_url,
        tutor.profile
    )
    .fetch_one(pool)
    .await?;

    // Retrieve result
    Ok(Tutor {
        tutor_id: row.tutor_id,
        tutor_name: row.tutor_name,
        tutor_pic_url: row.tutor_pic_url,
        tutor_profile: row.tutor_profile,
    })
}
