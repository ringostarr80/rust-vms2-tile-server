pub mod demo_controller;
pub mod index_controller;
pub mod open_api_controller;

use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct BaseUiContext {
    active_nav_element: &'static str,
}
