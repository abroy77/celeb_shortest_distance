#[cfg(test)]
mod test {
    use crate::data::{MovieDB, MovieDBBuilder};
    use crate::graph::shortest_path;
    use std::path::PathBuf;

    fn make_small_db() -> MovieDB {
        let data_dir = PathBuf::from("data/new_small");
        MovieDBBuilder::from_dir(&data_dir).unwrap()
    }
    fn make_large_db() -> MovieDB {
        let data_dir = PathBuf::from("data/new_large");
        MovieDBBuilder::from_dir(&data_dir).unwrap()
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
