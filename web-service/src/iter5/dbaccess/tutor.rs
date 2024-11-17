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
            tutor_name: row.tutor_name.clone(),
            tutor_pic_url: row.tutor_pic_url.clone(),
            tutor_profile: row.tutor_profile.clone(),
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
        tutor.tutor_name,
        tutor.tutor_pic_url,
        tutor.tutor_profile
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

pub async fn update_tutor_details_db(
    pool: &PgPool,
    tutor_id: i32,
    tutor: UpdateTutor,
) -> Result<Tutor, EzyTutorError> {
    // Retrieve current record
    let current_tutor_row = sqlx::query_as!(
        Tutor,
        "SELECT * from ezy_tutor_c6 where tutor_id = $1",
        tutor_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| EzyTutorError::NotFound("Tutor id not found".into()))?;

    // Construct the parameters for update:
    let tutor_name: String = if let Some(tutor_name) = tutor.tutor_name {
        tutor_name
    } else {
        current_tutor_row.tutor_name
    };

    let tutor_pic_url: String = if let Some(tutor_pic_url) = tutor.tutor_pic_url {
        tutor_pic_url
    } else {
        current_tutor_row.tutor_pic_url
    };

    let tutor_profile: String = if let Some(tutor_profile) = tutor.tutor_profile {
        tutor_profile
    } else {
        current_tutor_row.tutor_profile
    };

    let tutor_row = sqlx::query_as!(
        Tutor,
        "update ezy_tutor_c6 set tutor_name = $1, tutor_pic_url = $2, tutor_profile = $3
        returning tutor_id, tutor_name, tutor_pic_url, tutor_profile",
        tutor_name,
        tutor_pic_url,
        tutor_profile
    )
    .fetch_one(pool)
    .await;

    if let Ok(tutor) = tutor_row {
        Ok(tutor)
    } else {
        Err(EzyTutorError::NotFound("Tutor id not found".into()))
    }
}

pub async fn delete_tutor_db(pool: &PgPool, tutor_id: i32) -> Result<String, EzyTutorError> {
    // Prepare SQL statement
    let tutor_row = sqlx::query!("DELETE FROM ezy_tutor_c6 where tutor_id = $1", tutor_id)
        .execute(pool)
        .await?;

    Ok(format!("Deleted {:#?} record", tutor_row))
}