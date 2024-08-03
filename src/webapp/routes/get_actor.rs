use crate::{data::Actor, webapp::db_connection};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{self, FromRow, SqlitePool};

#[derive(Deserialize, FromRow)]
pub struct ActorQuery {
    name: String,
}
pub async fn get_actor_prefix(
    pg_pool: web::Data<SqlitePool>,
    query: web::Form<ActorQuery>,
) -> impl Responder {
    let name = &query.name;
    if name.len() < 4 {
        return HttpResponse::Ok().json(Vec::<Actor>::new());
    }
    let actors = db_connection::prefix_query_actors(&pg_pool, &query.name).await;

    HttpResponse::Ok().json(actors)
}

pub async fn get_actor(
    pg_pool: web::Data<SqlitePool>,
    query: web::Query<ActorQuery>,
) -> impl Responder {
    let name = &query.name;
    if name.len() < 4 {
        return HttpResponse::Ok().json(Vec::<String>::new());
    }
    let actor = db_connection::query_actor(&pg_pool, &query.name).await;

    if actor.is_empty() {
        let similar_actors = db_connection::query_similar_actor(&pg_pool, &query.name).await;
        HttpResponse::Ok().json(similar_actors)
    } else if actor.len() == 1 {
        return HttpResponse::Ok().json(actor);
    } else {
        HttpResponse::Ok().json(actor)
    }
}
