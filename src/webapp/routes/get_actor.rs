use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct ActorQuery {
    name: String,
}
pub async fn get_actor_prefix(
    pg_pool: web::Data<PgPool>,
    query: web::Query<ActorQuery>,
) -> impl Responder {
    let name = &query.name;
    if name.len() < 3 {
        return HttpResponse::Ok().json(Vec::<String>::new());
    }
    let actors = prefix_query_actors(&pg_pool, &query.name).await;

    HttpResponse::Ok().json(actors)
}

async fn prefix_query_actors(pg_pool: &PgPool, name: &str) -> Vec<String> {

    sqlx::query!(
        r#"SELECT full_name FROM actors 
        WHERE full_name ILIKE $1 || '%' 
        LIMIT 5"#,
        name
        )
    .fetch_all(pg_pool)
    .await
    .unwrap()
    .into_iter()
    .map(|row| row.full_name)
    .collect()

    }
