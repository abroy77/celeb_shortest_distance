use actix_web::{App, HttpServer, web, HttpResponse, Responder,
dev::Server};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::TcpListener;
use std::path::PathBuf;
use serde::Deserialize;
use crate::data::{MovieDB, MovieDBBuilder};
use std::thread;
use crate::configuration::{Settings, DatabaseSettings};
use tracing_actix_web::TracingLogger;
use crate::webapp::routes::get_actor_prefix;

pub struct Application {
    pub port: u16,
    pub server: Server,
}

impl Application { 
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let movie_db = build_movie_db(&configuration.movie_data.file_path);

        let server = run(listener, connection_pool, movie_db)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(db_config: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(db_config.with_db())
}

pub fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    movie_db: MovieDB,
) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let movie_db = web::Data::new(movie_db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            // .route("/health_check", web::get().to(health_check::health_check))
            // .route("/subscriptions", web::post().to(subscriptions::subscribe))
            // .route("/actor", web::get().to(get_actor::get_actor))
            .route("/actor_prefix", web::get().to(get_actor_prefix))
            .app_data(connection_pool.clone())
            .app_data(movie_db.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
#[derive(Deserialize)]
struct ActorQuery {
    name: String,
}

fn build_movie_db(data_dir: &PathBuf) -> MovieDB {
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

    MovieDB {
        actors,
        movies,
        actor_to_movies,
        movie_to_actors,
    }


}