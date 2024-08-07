use celeb_shortest_distance::data::{MovieDB, MovieDBBuilder};
use celeb_shortest_distance::graph::shortest_path;
use celeb_shortest_distance::interactive_io;

use std::env;
use std::io::{stdin, stdout};
use std::path::PathBuf;
use std::thread;

struct Config {
    db_path: PathBuf,
    // source_actor_name: String,
    // target_actor_name: String,
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

        // let source_actor_name = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a source actor name"),
        // };

        // let target_actor_name = match args.next() {
        //     Some(arg) => arg,
        //     None => return Err("Didn't get a target actor name"),
        // };

        Ok(Config {
            db_path,
            // source_actor_name,
            // target_actor_name,
        })
    }
}

#[tokio::main]
async fn main() {
    // get db file
    let config = Config::build(env::args())
        .unwrap_or_else(|err| panic!("Problem parsing arguments: {:?}", err));

    let actor_file = config.db_path.join("actors.csv");
    let actors = MovieDBBuilder::read_actors(&actor_file).unwrap_or_else(|err| {
        panic!(
            "Problem reading actors from file {:?}: {:?}",
            &actor_file, err
        );
    });

    // spawn threads to read movies and connections
    let movie_conns_handler =
        thread::spawn(move || MovieDBBuilder::build_movies_connections(&config.db_path));

    // get source and target actors
    println!("{}", ["#"; 20].concat());
    println!("Enter source actor name: ");
    let source_actor = interactive_io::get_unique_actor(stdin().lock(), stdout(), &actors);

    println!("{}", ["#"; 20].concat());
    println!("Enter target actor name: ");
    let target_actor = interactive_io::get_unique_actor(stdin().lock(), stdout(), &actors);

    if source_actor == target_actor {
        println!("Source and target actors are the same");
        println!("0 Degrees of connection");
        return;
    }

    // join handles
    let (movies, actor_to_movies, movie_to_actors) = movie_conns_handler.join().unwrap().unwrap();
    // make db and return
    let db = MovieDB {
        actor_to_movies,
        movie_to_actors,
        actors,
        movies,
    };

    // get shortest path
    println!("{}", ["#"; 20].concat());
    println!("Calculating shortest path...");
    println!("{}", ["#"; 20].concat());
    let shortest_path = shortest_path(source_actor, target_actor, &db).await;

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
                    actor_1.full_name, movie.title, actor_2.full_name
                )
            }
        }
        Err(err) => {
            println!("No path found");
            println!("{}", err);
        }
    }
}
