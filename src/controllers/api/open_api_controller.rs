use rocket::http::ContentType;
use utoipa::OpenApi;

use super::ApiError;

#[derive(OpenApi)]
#[openapi(
    info(description = "Vms2TileServer API"),
    paths(crate::controllers::api::tile_controller::tile)
)]
struct ApiDoc;

#[get("/open_api.json")]
pub fn open_api() -> Result<(ContentType, String), ApiError> {
    Ok((ContentType::JSON, ApiDoc::openapi().to_pretty_json()?))
}
