use crate::controllers::api::open_api_controller::open_api;
use crate::controllers::api::tile_controller::tile;
use rocket::{routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![open_api, tile]
}
