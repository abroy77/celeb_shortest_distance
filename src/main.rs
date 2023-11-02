use crate::data::{MovieDB, MovieDBBuilder};
use graph::shortest_path;
use std::env;
use std::path::PathBuf;

pub mod data;
pub mod graph;

struct Config {
    db_path: PathBuf,
    source_actor_name: String,
    target_actor_name: String,
}

impl Config {
    fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next(); // remove the first arg, which is the program name

        let db_path = match args.next() {
            Some(arg) => PathBuf::from(arg),
            None => return Err("Didn't get a db path"),
        };

        let source_actor_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a source actor name"),
        };

        let target_actor_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a target actor name"),
        };

        Ok(Config {
            db_path,
            source_actor_name,
            target_actor_name,
        })
    }
}

fn main() {
    // get db file
    let config = Config::build(env::args())
        .unwrap_or_else(|err| panic!("Problem parsing arguments: {:?}", err));

    let db: MovieDB = MovieDBBuilder::from_dir(&config.db_path).unwrap_or_else(|err| {
        panic!(
            "Problem reading db from file {:?}: {:?}",
            &config.db_path, err
        );
    });

    let source_actor_id = match db.get_actor_by_name(&config.source_actor_name) {
        Ok(actor) => actor.id,
        Err(err) => panic!("Source actor not found in db. err: {err}"),
    };

    println!(
        "ID for Source: {}, is {}",
        config.source_actor_name, source_actor_id
    );

    let target_actor_id = match { db.get_actor_by_name(&config.target_actor_name) } {
        Ok(actor) => actor.id,
        Err(err) => panic!("Target actor not found in db. err: {err}"),
    };

    println!(
        "ID for Target: {}, is {}",
        config.target_actor_name, target_actor_id
    );

    // get shortest path
    let shortest_path = shortest_path(source_actor_id, target_actor_id, &db);

    match shortest_path {
        Ok(path) => {
            println!("Degrees of connection: {}", path.len() - 1);
            println!("Shortest path is: ");

            for node_index in 0..(path.len() - 1) {
                let actor_1 = db.actors.get(&path[node_index].actor_id).unwrap();
                let actor_2 = db.actors.get(&path[node_index + 1].actor_id).unwrap();
                let movie = db
                    .movies
                    .get(&path[node_index + 1].movie_id.unwrap())
                    .unwrap();
                println!(
                    "{} was in {} with {}",
                    actor_1.name, movie.title, actor_2.name
                )
            }
        }
        Err(_) => println!("No path found"),
    }
}
