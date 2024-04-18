use std::{fs::File, io::Read};

use actix_web::{web, HttpResponse};
use log::{error, info};
use utils::{
    error::{ApiError, ApiErrorStatus},
    uuid_utils::UuidUtils,
};

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref ASSETS_DIRECTORY: String = "assets".to_string();
}

pub struct LogosService {}

impl LogosService {
    pub fn init(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/logos/{company_name}").get(LogosService::get));
    }

    async fn get(company_name: web::Path<String>) -> HttpResponse {
        let track_id = UuidUtils::generate_new_as_string();

        let file_path = format!("{}/{}.png", *ASSETS_DIRECTORY, company_name);

        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                return ApiError::new(&track_id, "Logo not found", ApiErrorStatus::NotFound).into()
            }
        };

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => {
                info!(
                    "Get in storage client is success with company name {} for track id {}",
                    &company_name,
                    &track_id
                );

                HttpResponse::Ok().content_type("image/png").body(buffer)
            }
            Err(e) => {
                error!(
                    "Failed to read the file with message {} and track_id {}",
                    e, &track_id
                );

                ApiError::new(
                    &track_id,
                    "Failed to retrive logo",
                    ApiErrorStatus::InternalError,
                )
                .into()
            }
        }
    }
}
