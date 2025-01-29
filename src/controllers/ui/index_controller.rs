use rocket_dyn_templates::Template;

use crate::controllers::ui::BaseUiContext;

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        BaseUiContext {
            active_nav_element: "home",
        },
    )
}
