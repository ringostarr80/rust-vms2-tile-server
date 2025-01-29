use rocket_dyn_templates::{context, Template};

use crate::controllers::ui::BaseUiContext;

#[get("/open_api")]
pub fn open_api() -> Template {
    Template::render(
        "open_api",
        BaseUiContext {
            active_nav_element: "open_api",
        },
    )
}

#[get("/open_api/stoplight")]
pub fn open_api_stoplight() -> Template {
    Template::render("open_api/stoplight", context! {})
}
