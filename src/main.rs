mod error;

use crate::error::{HitError, HitResult};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use dotenv::dotenv;
use qstring::QString;
use rusqlite::{params, Connection, NO_PARAMS};
use std::env;

const BADGE: &str = include_str!("./badge.svg");

#[derive(Clone, Copy, Debug)]
struct Hit {
    count: i32,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    init_db().expect("Failed to init the database");

    let host = env::var("HITS_HOST").unwrap_or("0.0.0.0".to_string());
    let port = env::var("HITS_PORT").unwrap_or("8088".to_string());
    let endpoint = env::var("HITS_ENDPOINT").unwrap_or("/hits.svg".to_string());

    println!(
        "Server ready, open http://{}:{}{} to see the live counter",
        host, port, endpoint
    );

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .route(&endpoint, web::get().to(handler))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}

fn init_db() -> HitResult<()> {
    let db = Connection::open("./data/hits.db")?;

    db.execute("CREATE TABLE IF NOT EXISTS hits (count INTEGER)", NO_PARAMS)?;

    db.execute("INSERT INTO hits (count) VALUES (?1)", params![0])?;

    Ok(())
}

async fn handler(req: HttpRequest) -> HttpResponse {
    let query_str = req.query_string();
    let qs = QString::from(query_str);

    let badge_text = match qs.get("lower") {
        Some("true") => BADGE.clone().replace("HITS", "hits"),
        _ => BADGE.to_string(),
    };

    match get_count() {
        Ok(Hit { count }) => HttpResponse::Ok()
            .header("Content-Type", "image/svg+xml")
            .header(
                "Cache-Control",
                "max-age=0, no-cache, no-store, must-revalidate",
            )
            .body(badge_text.replace("{count}", &format!("{}", count))),

        _ => HttpResponse::Ok().body(""),
    }
}

fn get_count() -> HitResult<Hit> {
    let conn = Connection::open("./data/hits.db")?;

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
