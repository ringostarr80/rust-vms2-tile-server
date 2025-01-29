use std::str::FromStr;

use rocket::State;
use utoipa::ToSchema;
use vms2_tile_db_reader::{data_type::DataType, sources::Source};

use super::ApiError;

#[derive(ToSchema)]
#[schema(value_type = String, format = Binary)]
#[allow(dead_code)]
pub struct Binary(String);

#[utoipa::path(
    get,
    path = "/api/tile/{z}/{y}/{x}",
    params(
        ("z" = u8, Path, description = "Zoomlevel"),
        ("y" = u32, Path, description = "Y coordinate"),
        ("x" = u32, Path, description = "X coordinate"),
        ("k" = String, Query, description = "The OSM key"),
        ("v" = Option<String>, Query, description = "The OSM value"),
        ("t" = Option<String>, Query, description = "The OSM type (point, line, polygon)"),
    ),
    responses(
        (
            status = 200,
            description = "Binary data",
            body = Binary,
            content_type = "application/octet-stream"
        ),
    ),
    operation_id = "getTile",
    tags = ["default"],
)]
#[get("/tile/<z>/<y>/<x>?<k>&<v>&<t>")]
pub fn tile(
    z: u8,
    y: u32,
    x: u32,
    k: String,
    v: Option<String>,
    t: Option<String>,
    global_data: &State<crate::GlobalData>,
) -> Result<Vec<u8>, ApiError> {
    let vms2_db_source = global_data.vms2_db_source.lock()?;

    let t = match t {
        Some(t) => Some(DataType::from_str(t.as_str()).unwrap_or(DataType::Points)),
        None => None,
    };
    Ok(vms2_db_source.get_raw_data(x, y, z, k, v, t)?)
}
