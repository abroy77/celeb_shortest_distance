#[cfg(test)]
mod test {
    use celeb_shortest_distance::data::{MovieDB, MovieDBBuilder};
    use celeb_shortest_distance::graph::shortest_path;
    use std::path::PathBuf;

    fn make_db(data_dir: &PathBuf) -> MovieDB {
        let actor_file = data_dir.join("actors.csv");
        let actors = MovieDBBuilder::read_actors(&actor_file).unwrap();
        let movie_file = data_dir.join("movies.csv");
        let movies = MovieDBBuilder::read_movies(&movie_file).unwrap();
        let actor_movie_file = data_dir.join("connections.csv");
        let pairs = MovieDBBuilder::read_actor_movie_pairs(&actor_movie_file).unwrap();
        let (actor_to_movies, movie_to_actors) = MovieDBBuilder::get_actor_movie_maps(pairs);
        MovieDB {
            actors,
            movies,
            actor_to_movies,
            movie_to_actors,
        }
    }

    fn make_small_db() -> MovieDB {
        let data_dir = PathBuf::from("data/new_small");
        make_db(&data_dir)
    }
    fn make_large_db() -> MovieDB {
        let data_dir = PathBuf::from("data/new_large");
        make_db(&data_dir)
    }
    #[test]
    fn cruise_hanks() {
        let source_id: usize = 129;
        let target_id: usize = 158;

        let db = make_small_db();

        let path = shortest_path(source_id, target_id, &db).unwrap();
        assert_eq!(path.len(), 3);
    }
    #[test]
    #[ignore = "takes too long"]
    fn massey_fox() {
        let source_id = 5368041;
        let target_id = 289114;

        let db = make_large_db();

        let path = shortest_path(source_id, target_id, &db).unwrap();
        assert_eq!(path.len(), 8);
    }
}
