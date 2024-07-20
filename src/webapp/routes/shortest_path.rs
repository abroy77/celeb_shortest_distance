use crate::{data::MovieDB, graph::shortest_path};
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct TwoActors {
    actor_1: String,
    actor_2: String,
}

pub async fn get_shortest_path(
    query: web::Query<TwoActors>,
    movie_db: web::Data<MovieDB>,
) -> impl Responder {
    let actor_1_id = movie_db
        .actors
        .iter()
        .filter(|(_, actor)| actor.full_name == query.actor_1)
        .map(|(_, actor)| actor.id)
        .next();

    let actor_2_id = movie_db
        .actors
        .iter()
        .filter(|(_, actor)| actor.full_name == query.actor_2)
        .map(|(_, actor)| actor.id)
        .next();

    let actor_1_id = match actor_1_id {
        Some(id) => id,
        None => return HttpResponse::NotFound().finish(),
    };

    let actor_2_id = match actor_2_id {
        Some(id) => id,
        None => return HttpResponse::NotFound().finish(),
    };  

    let shortest_path = shortest_path(actor_1_id, actor_2_id, &movie_db);

    let mut shortest_path_json = Vec::new();

    match shortest_path {
        Ok(path) => {
            for node_index in 0..(path.len() - 1) {
                let actor_1 = movie_db.actors.get(&path[node_index].actor_id).unwrap();
                let actor_2 = movie_db.actors.get(&path[node_index + 1].actor_id).unwrap();
                let movie = movie_db
                    .movies
                    .get(&path[node_index + 1].movie_id.unwrap())
                    .unwrap();

                let json_obj = json!({
                    "actor_1": actor_1.full_name,
                    "movie": movie.title,
                    "actor_2": actor_2.full_name,
                });

                shortest_path_json.push(json_obj);
            }

            HttpResponse::Ok().json(shortest_path_json)
        }
        Err(err) => {
            println!("No path found");
            println!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }

}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions, SqlitePool};


    fn setup_actor_db() -> SqlitePool {
        let cnnection_options = SqliteConnectOptions::new().filename("actors.db");

        let pool = SqlitePoolOptions::new().connect_lazy_with(cnnection_options);
        pool
    }
}
