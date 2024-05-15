use actix_web::{App, HttpServer, web, HttpResponse, Responder};
use std::path::PathBuf;
use serde::Deserialize;
use crate::data::{MovieDB, MovieDBBuilder};
use std::thread;
use crate::interactive_io::get_actor_by_name;

#[derive(Deserialize)]
struct ActorQuery {
    name: String,
}
pub async fn launch() {
    let addrs = "127.0.0.1:8000";
    // build the movie_db
    // use different threads to read actors, movies, and connections



    let data_dir = PathBuf::from("data/new_large/");
    let actor_file = data_dir.join("actors.csv");
    let actor_read_handler = thread::spawn( move || {
        MovieDBBuilder::read_actors(&actor_file)
        .expect("Failed to read actors file")
    });
    let movie_file = data_dir.join("movies.csv");
    let movie_read_handler = thread::spawn(move || {
        MovieDBBuilder::read_movies(&movie_file)
        .expect("Failed to read movies file")
    });

    let connections_file = data_dir.join("connections.csv");
    let connections_read_handler = thread::spawn(move || {
        let connections = MovieDBBuilder::read_actor_movie_pairs(&connections_file)
        .expect("Failed to read connections file");
        MovieDBBuilder::get_actor_movie_maps(connections)
    });

    let actors = actor_read_handler.join().unwrap();
 

    let movies = movie_read_handler.join().unwrap();
    let (actor_to_movies, movie_to_actors) = connections_read_handler.join().unwrap();

    let movie_db = MovieDB {
        actors,
        movies,
        actor_to_movies,
        movie_to_actors,
    };

    let movie_db = web::Data::new(movie_db);
    let _ = HttpServer::new(move || {
        App::new()
        .app_data(movie_db.clone())
        .route("/actor", web::get().to(get_actor))
    }).bind(addrs).unwrap().run().await;

   
}


async fn get_actor(movie_db: web::Data<MovieDB>,
    query: web::Query<ActorQuery>) -> impl Responder {

    let actor = get_actor_by_name(&movie_db.actors, &query.name);
   match actor.len() {
         0 => HttpResponse::NotFound().body("No actor found"),
         1 => HttpResponse::Ok().json(actor.into_iter().next().unwrap().id),
         _ => HttpResponse::BadRequest().body("Multiple actors found"),
   }
}