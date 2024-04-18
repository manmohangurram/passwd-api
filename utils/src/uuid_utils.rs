use actix_web::HttpResponse;
use log::{debug, error};

use uuid::Uuid;
use validator::ValidationError;

use crate::error::{ApiError, ApiErrorStatus};

pub struct UuidUtils {}

impl UuidUtils {
    pub fn generate_new_as_string() -> String {
        let uuid = UuidUtils::generate_new();
        uuid.as_simple().to_string()
    }

    pub fn generate_new() -> uuid::Uuid {
        uuid::Uuid::now_v7()
    }

    pub fn validate(simple_uuid: &str) -> Result<Uuid, ValidationError> {
        let parse_result = uuid::Uuid::parse_str(simple_uuid);

        match parse_result {
            Ok(uuid) => Ok(uuid),
            Err(_) => Err(ValidationError::new("UUID validation failed")),
        }
    }

    pub fn validate_record_id(
        track_id: &str,
        note_id: &str,
        msg: &str,
    ) -> Result<Uuid, HttpResponse> {
        match UuidUtils::validate(note_id) {
            Ok(uuid) => {
                debug!(
                    "UUID validation success for note_id {} and track_id {}",
                    note_id, track_id
                );

                Ok(uuid)
            }

            Err(_) => {
                error!(
                    "UUID validation failed for note_id {} with track_id {}",
                    note_id, track_id
                );

                Err(ApiError::new(track_id, msg, ApiErrorStatus::BadRequest).into())
            }
        }
    }
}
