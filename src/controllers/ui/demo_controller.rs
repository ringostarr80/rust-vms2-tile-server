use rocket_dyn_templates::Template;

use crate::controllers::ui::BaseUiContext;

#[get("/demo")]
pub fn demo() -> Template {
    Template::render(
        "demo",
        BaseUiContext {
            active_nav_element: "demo",
        },
    )
}
