use crate::{data::MovieDB, graph::shortest_path};
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use tokio::time::{Duration, timeout};
#[derive(Deserialize)]
pub struct TwoActors {
    actor_1: usize,
    actor_2: usize,
}
#[derive(Serialize)]
pub struct Connection {
    actor_1: String,
    movie: String,
    actor_2: String,
}

pub async fn get_shortest_path(
    query: web::Form<TwoActors>,
    movie_db: web::Data<MovieDB>,
) -> impl Responder {
    if !movie_db.actors.contains_key(&query.actor_1) {
        return HttpResponse::NotFound().body("Actor 1 not found");
    }
    if !movie_db.actors.contains_key(&query.actor_2) {
        return HttpResponse::NotFound().body("Actor 2 not found");
    }

    let timeout_duration = Duration::from_secs(60);

    let shortest_path_result = timeout(
        timeout_duration,
        shortest_path(query.actor_1, query.actor_2, &movie_db),
    )
    .await;

    let path = match shortest_path_result {
        Ok(Ok(path)) => path,
        Ok(Err(_)) => {
            let actor_1_name = &movie_db.actors.get(&query.actor_1).unwrap().full_name;
            let actor_2_name = &movie_db.actors.get(&query.actor_2).unwrap().full_name;

            return HttpResponse::InternalServerError().body(format!(
                "No connection found between {} and {}",
                actor_1_name, actor_2_name
            ));
        }
        Err(_) => {
            let actor_1_name = &movie_db.actors.get(&query.actor_1).unwrap().full_name;
            let actor_2_name = &movie_db.actors.get(&query.actor_2).unwrap().full_name;

            return HttpResponse::InternalServerError().body(format!(
                "No connection found between {} and {} within 60s",
                actor_1_name, actor_2_name
            ));
        }
    };

    let mut shortest_path_json = Vec::new();

    for node_index in 0..(path.len() - 1) {
        let actor_1 = movie_db.actors.get(&path[node_index].actor_id).unwrap();
        let actor_2 = movie_db.actors.get(&path[node_index + 1].actor_id).unwrap();
        let movie = movie_db
            .movies
            .get(&path[node_index + 1].movie_id.unwrap())
            .unwrap();

        let connection = Connection {
            actor_1: actor_1.full_name.clone(),
            movie: movie.title.clone(),
            actor_2: actor_2.full_name.clone(),
        };

        shortest_path_json.push(connection);
    }

    HttpResponse::Ok().json(shortest_path_json)
}

// tests
#[cfg(test)]
mod tests {

    use crate::data::Actor;
    use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions};

    fn setup_actor_db() -> SqlitePool {
        let cnnection_options = SqliteConnectOptions::new().filename("actors.db");

        SqlitePoolOptions::new().connect_lazy_with(cnnection_options)
    }
    fn srk() -> Actor {
        Actor {
            full_name: "Shah Rukh Khan".to_string(),
            id: 451321,
            birth_year: Some(1965),
        }
    }

    fn tom_cruise() -> Actor {
        Actor {
            full_name: "Tom Cruise".to_string(),
            id: 129,
            birth_year: Some(1962),
        }
    }

    //
}
