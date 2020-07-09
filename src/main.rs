mod error;

use crate::error::{HitError, HitResult};
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use rusqlite::{params, Connection, NO_PARAMS};

const BADGE: &str = include_str!("./badge.svg");

#[derive(Clone, Copy, Debug)]
struct Hit {
    count: i32,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_db().expect("Failed to init the database");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/hits.svg", web::get().to(handler))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

fn init_db() -> HitResult<()> {
    let db = Connection::open("./hits.db")?;

    db.execute("CREATE TABLE IF NOT EXISTS hits (count INTEGER)", NO_PARAMS)?;

    db.execute("INSERT INTO hits (count) VALUES (?1)", params![0])?;

    Ok(())
}

async fn handler() -> HttpResponse {
    match get_count() {
        Ok(Hit { count }) => HttpResponse::Ok()
            .header("Content-Type", "image/svg+xml")
            .header(
                "Cache-Control",
                "max-age=0, no-cache, no-store, must-revalidate",
            )
            .body(BADGE.replace("{count}", &format!("{}", count))),

        _ => HttpResponse::Ok().body(""),
    }
}

fn get_count() -> HitResult<Hit> {
    let conn = Connection::open("./hits.db")?;

    conn.execute("UPDATE hits SET count = count + 1", NO_PARAMS)?;

    let mut stmt = conn.prepare("SELECT count FROM hits")?;
    let result_iter = stmt.query_map(NO_PARAMS, |row| Ok(Hit { count: row.get(0)? }))?;

    let mut result_count = vec![];
    for result in result_iter {
        result_count.push(result?)
    }

    let hit = result_count
        .get(0)
        .ok_or(HitError::GenericError("No results from db".to_string()))?;

    Ok(*hit)
}
