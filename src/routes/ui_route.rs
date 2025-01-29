use crate::controllers::ui::{
    demo_controller::demo,
    index_controller::index,
    open_api_controller::{open_api, open_api_stoplight},
};
use rocket::{routes, Route};

pub fn get_routes() -> Vec<Route> {
    routes![demo, index, open_api, open_api_stoplight]
}
