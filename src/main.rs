extern crate dotenvy;
#[macro_use]
extern crate rocket;
extern crate utoipa;
extern crate vms2_tile_db_reader;

mod controllers;
mod routes;

use std::env;
use std::path::Path;
use std::process;
use std::sync::{Arc, Mutex};

use rocket::fs::{relative, FileServer};
use rocket::Error;
use rocket_dyn_templates::Template;
use routes::{api_route, ui_route};
use vms2_tile_db_reader::sources::SQLite;

pub struct GlobalData {
    pub vms2_db_source: Arc<Mutex<SQLite>>,
}

#[rocket::main]
pub async fn main() -> Result<(), Error> {
    dotenvy::dotenv_override().unwrap_or_default();

    let sqlite_path = match env::var("SQLITE_DB_PATH") {
        Ok(sqlite_path) => sqlite_path,
        Err(_) => String::from("tiles.sqlite3"),
    };
    let vms2_db_source = match SQLite::new(Path::new(sqlite_path.as_str())) {
        Ok(vms2_db_source) => vms2_db_source,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let global_data = GlobalData {
        vms2_db_source: Arc::new(Mutex::new(vms2_db_source)),
    };

    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("static")))
        .mount("/", ui_route::get_routes())
        .mount("/api", api_route::get_routes())
        .mount("/vms2-tile-server/api", api_route::get_routes())
        .attach(Template::fairing())
        .manage(global_data)
        .launch()
        .await?;

    Ok(())
}
