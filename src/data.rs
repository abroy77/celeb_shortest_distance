use csv::ReaderBuilder;
use serde::Deserialize;
use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::path::PathBuf;

use std::hash::{Hash, Hasher};
// movie struct
#[derive(Debug, Deserialize)]
pub struct Movie {
    pub id: usize,
    pub title: String,
    pub year: u32,
}
#[derive(Debug, Deserialize)]
pub struct Actor {
    pub id: usize,
    pub name: String,
    #[serde(rename = "birth")]
    pub birth_year: Option<u32>,
}

pub struct MovieDB {
    pub actor_to_movies: HashMap<usize, HashSet<usize>>,
    pub movie_to_actors: HashMap<usize, HashSet<usize>>,
    pub actors: HashMap<usize, Actor>,
    pub movies: HashMap<usize, Movie>,
}

pub struct MovieDBBuilder;

impl MovieDBBuilder {
    pub fn from_dir(dir_path: &PathBuf) -> Result<MovieDB, Box<dyn Error>> {
        let actor_file = dir_path.join("people.csv");
        let actors = MovieDBBuilder::read_actors(&actor_file).unwrap_or_else(|err| {
            panic!(
                "Problem reading actors from file {:?}: {:?}",
                actor_file, err
            )
        });

        let movie_file = dir_path.join("movies.csv");
        let movies = MovieDBBuilder::read_movies(&movie_file).unwrap_or_else(|err| {
            panic!(
                "Problem reading movies from file {:?}: {:?}",
                movie_file, err
            )
        });

        let actor_movie_file = dir_path.join("stars.csv");
        let actor_movie_pairs = MovieDBBuilder::read_actor_movie_pairs(&actor_movie_file)
            .unwrap_or_else(|err| {
                panic!(
                    "Problem reading actor movie pairs from file {:?}: {:?}",
                    actor_movie_file, err
                )
            });

        // now we need to make both maps
        let mut actor_to_movie = HashMap::new();
        let mut movie_to_actor = HashMap::new();

        for (actor_id, movie_id) in actor_movie_pairs {
            let actor = match actors.get(&actor_id) {
                Some(actor) => actor,
                None => continue,
            };
            let movie = match movies.get(&movie_id) {
                Some(movie) => movie,
                None => continue,
            };

            // update the hashset for actor to movies
            let movies_of_actor = actor_to_movie.entry(actor_id).or_insert(HashSet::new());
            movies_of_actor.insert(movie_id);

            // update the hashset for movie to actors
            let actors_of_movie = movie_to_actor.entry(movie_id).or_insert(HashSet::new());
            actors_of_movie.insert(actor_id);
        }

        // make db and return
        let db = MovieDB {
            actor_to_movies: actor_to_movie,
            movie_to_actors: movie_to_actor,
            actors,
            movies,
        };

        Ok(db)
    }

    fn read_actors(fpath: &PathBuf) -> Result<HashMap<usize, Actor>, Box<dyn Error>> {
        let mut actors = HashMap::new();
        let mut rdr = ReaderBuilder::new().from_path(fpath)?;
        for record in rdr.deserialize() {
            let actor: Actor = record?;
            actors.insert(actor.id, actor);
        }

        Ok(actors)
    }

    fn read_movies(fpath: &PathBuf) -> Result<HashMap<usize, Movie>, Box<dyn Error>> {
        let mut movies = HashMap::new();
        let mut rdr = ReaderBuilder::new().from_path(fpath)?;
        for record in rdr.deserialize() {
            let movie: Movie = record?;
            movies.insert(movie.id, movie);
        }

        Ok(movies)
    }

    fn read_actor_movie_pairs(fpath: &PathBuf) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
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
}

impl cmp::PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl cmp::PartialEq for Actor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
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

impl MovieDB {
    // should we think of actors as nodes and movies as edges?
    pub fn get_actor_by_name(&self, name: &str) -> Result<&Actor, &'static str> {
        match self
            .actors
            .iter()
            .find(|(_, actor)| actor.name == name)
            .map(|(_, actor)| actor)
        {
            Some(actor) => Ok(actor),
            None => Err("Actor not found"),
        }
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn data_read_actor() {
        let data_file = PathBuf::from("data/small/people.csv");
        let actors = MovieDBBuilder::read_actors(&data_file).unwrap();
        assert_eq!(actors.len(), 16);
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
        let data_file = PathBuf::from("data/small/movies.csv");
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
        let data_file = PathBuf::from("data/small/stars.csv");
        let pairs = MovieDBBuilder::read_actor_movie_pairs(&data_file).unwrap();
        assert_eq!(pairs.len(), 20);
        assert!(pairs.contains(&(596520, 95953)));
    }

    #[test]
    fn data_make_db() {
        let data_dir = PathBuf::from("data/small");
        let db = MovieDBBuilder::from_dir(&data_dir).unwrap();
        assert_eq!(db.actors.len(), 16);
        assert_eq!(db.movies.len(), 5);
        assert_eq!(db.actor_to_movies[&102].len(), 2);
        assert_eq!(db.movie_to_actors[&104257].len(), 4);
    }
}
