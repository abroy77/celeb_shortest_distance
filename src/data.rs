use csv::ReaderBuilder;
use serde::Deserialize;
use std::cmp;
use std::collections::{HashMap, HashSet};

use std::fmt::Display;
use std::io::Error as IoError;
use std::path::{PathBuf, Path};
use std::thread;

use std::hash::{Hash, Hasher};
// movie struct
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub id: usize,
    pub title: String,
    pub year: u32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Actor {
    pub id: usize,
    pub name: String,
    pub birth_year: Option<u32>,
    // pub connectivity: Option<usize>,
}

type Mapping = HashMap<usize, HashSet<usize>>;

pub struct MovieDB {
    pub actor_to_movies: HashMap<usize, HashSet<usize>>,
    pub movie_to_actors: HashMap<usize, HashSet<usize>>,
    pub actors: HashMap<usize, Actor>,
    pub movies: HashMap<usize, Movie>,
}

pub struct MovieDBBuilder;

impl MovieDBBuilder {
    pub fn read_actors(fpath: &PathBuf) -> Result<HashMap<usize, Actor>, IoError> {
        let mut actors = HashMap::new();
        let mut rdr = ReaderBuilder::new().from_path(fpath)?;
        for record in rdr.deserialize() {
            let actor: Actor = record?;
            actors.insert(actor.id, actor);
        }

        Ok(actors)
    }

    pub fn read_movies(fpath: &PathBuf) -> Result<HashMap<usize, Movie>, IoError> {
        let mut movies = HashMap::new();
        let mut rdr = ReaderBuilder::new().from_path(fpath)?;
        for record in rdr.deserialize() {
            let movie: Movie = record?;
            movies.insert(movie.id, movie);
        }

        Ok(movies)
    }

    pub fn read_actor_movie_pairs(fpath: &PathBuf) -> Result<Vec<(usize, usize)>, IoError> {
        let mut actor_movie_map = Vec::new();
        let mut rdr = ReaderBuilder::new().from_path(fpath)?;
        // in the csv the header names are person_id, movie_id
        // use these when deserialising
        for record in rdr.deserialize() {
            let (person_id, movie_id): (usize, usize) = record?;
            actor_movie_map.push((person_id, movie_id));
        }

        Ok(actor_movie_map)
    }

    pub fn get_actor_movie_maps(connections: Vec<(usize, usize)>) -> (Mapping, Mapping) {
        let mut actor_to_movie = HashMap::new();
        let mut movie_to_actor = HashMap::new();

        for (actor_id, movie_id) in connections {
            // update the hashset for actor to movies
            let movies_of_actor = actor_to_movie.entry(actor_id).or_insert(HashSet::new());
            movies_of_actor.insert(movie_id);

            // update the hashset for movie to actors
            let actors_of_movie = movie_to_actor.entry(movie_id).or_insert(HashSet::new());
            actors_of_movie.insert(actor_id);
        }
        (actor_to_movie, movie_to_actor)
    }

    pub fn build_movies_connections(
        dir_path: &Path,
    ) -> Result<(HashMap<usize, Movie>, Mapping, Mapping), IoError> {
        let movie_file = dir_path.join("movies.csv");
        let movies_reader_handle = thread::spawn(move || MovieDBBuilder::read_movies(&movie_file));

        let actor_movie_file = dir_path.join("connections.csv");
        let am_reader_handle = thread::spawn(move || {
            let connections = MovieDBBuilder::read_actor_movie_pairs(&actor_movie_file)
                .unwrap_or_else(|err| {
                    panic!(
                        "Problem reading actor movie pairs from file {:?}: {:?}",
                        actor_movie_file, err
                    )
                });
            let (actor_to_movie, movie_to_actor) =
                MovieDBBuilder::get_actor_movie_maps(connections);
            (actor_to_movie, movie_to_actor)
        });

        let movies = match movies_reader_handle.join() {
            Ok(thread_result) => thread_result?,
            Err(_) => {
                return Err(IoError::new(
                    std::io::ErrorKind::Other,
                    "Problem reading movies from file".to_string(),
                ))
            }
        };

        let (actor_to_movie, movie_to_actor) = match am_reader_handle.join() {
            Ok(maps) => maps,
            Err(_) => {
                return Err(IoError::new(
                    std::io::ErrorKind::Other,
                    "Problem reading actor movie pairs from file".to_string(),
                ))
            }
        };

        Ok((movies, actor_to_movie, movie_to_actor))
    }
}

impl cmp::PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

}

impl cmp::PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Movie {}
impl Eq for Actor {}

impl Hash for Movie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Hash for Actor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.birth_year {
            Some(year) => write!(
                f,
                "(id: {}, name: {}, birth_year: {})",
                self.id, self.name, year
            ),
            None => write!(
                f,
                "(id: {}, name: {}, birth_year: unknown)",
                self.id, self.name
            ),
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn data_read_actor() {
        let data_file = PathBuf::from("data/new_small/actors.csv");
        let actors = MovieDBBuilder::read_actors(&data_file).unwrap();
        assert_eq!(actors.len(), 15);
        assert_eq!(
            actors[&197],
            Actor {
                id: 197,
                name: "Jack Nicholson".to_string(),
                birth_year: Some(1937),
            }
        )
    }

    #[test]
    fn data_read_movies() {
        let data_file = PathBuf::from("data/new_small/movies.csv");
        let movies = MovieDBBuilder::read_movies(&data_file).unwrap();
        assert_eq!(movies.len(), 5);
        assert_eq!(
            movies[&93779],
            Movie {
                id: 93779,
                title: "The Princess Bride".to_string(),
                year: 1987,
            }
        )
    }
    #[test]
    fn data_read_pairs() {
        let data_file = PathBuf::from("data/new_small/connections.csv");
        let pairs = MovieDBBuilder::read_actor_movie_pairs(&data_file).unwrap();
        assert_eq!(pairs.len(), 20);
        assert!(pairs.contains(&(596520, 95953)));
    }

    #[test]
    fn data_make_db() {
        let data_dir = PathBuf::from("data/new_small");
        let actor_file = data_dir.join("actors.csv");
        let actors = MovieDBBuilder::read_actors(&actor_file).unwrap();
        let movie_file = data_dir.join("movies.csv");
        let movies = MovieDBBuilder::read_movies(&movie_file).unwrap();
        let actor_movie_file = data_dir.join("connections.csv");
        let pairs = MovieDBBuilder::read_actor_movie_pairs(&actor_movie_file).unwrap();
        let (actor_to_movies, movie_to_actors) = MovieDBBuilder::get_actor_movie_maps(pairs);
        let db = MovieDB {
            actor_to_movies,
            movie_to_actors,
            actors,
            movies,
        };

        assert_eq!(db.actors.len(), 15);
        assert_eq!(db.movies.len(), 5);
        assert_eq!(db.actor_to_movies[&102].len(), 2);
        assert_eq!(db.movie_to_actors[&104257].len(), 4);
    }
}
