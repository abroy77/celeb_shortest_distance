use crate::data::Actor;
use std::collections::{HashMap, HashSet};
/// module for handline stdin inpt and otput for the program
use std::io::{BufRead, Write};
use strsim::jaro_winkler;

pub fn get_actor_by_name(actors: &HashMap<usize, Actor>, name: &str) -> HashSet<Actor> {
    let selected_actors = actors
        .iter()
        .filter(|(_, actor)| actor.full_name == name)
        .map(|(_, actor)| actor.clone())
        .collect();
    selected_actors
}

pub fn get_actor_by_id(actors: &HashMap<usize, Actor>, id: usize) -> Option<Actor> {
    actors.get(&id).cloned()
}

fn fuzzy_search_actor<'a, T>(actors: T, name: &str) -> Vec<&'a String>
where
    T: Iterator<Item = &'a String>,
{
    // get the top 3 similarity names
    let similar_names = actors
        .map(|actor| {
            let similarity = jaro_winkler(actor, name);
            (actor, similarity)
        })
        .collect::<Vec<_>>();

    let mut top_3: Vec<&(&String, f64)> = similar_names.iter().take(3).collect();

    let mut smallest_index = top_3
        .iter()
        .enumerate()
        .min_by(|a, b| a.1 .1.partial_cmp(&b.1 .1).unwrap())
        .unwrap()
        .0;
    let mut lowest_score = top_3[smallest_index].1;

    for combo in similar_names.iter().skip(3) {
        if combo.1 > lowest_score {
            // remove smallest
            top_3[smallest_index] = combo;
            // update smallest
            smallest_index = top_3
                .iter()
                .enumerate()
                .min_by(|a, b| a.1 .1.partial_cmp(&b.1 .1).unwrap())
                .unwrap()
                .0;
            lowest_score = top_3[smallest_index].1;
        }
    }

    top_3.iter().map(|(name, _)| *name).collect()
}

fn get_unique_actor_by_id<R, W>(
    mut reader: R,
    mut writer: W,
    actors: &HashMap<usize, Actor>,
) -> usize
where
    R: BufRead,
    W: Write,
{
    println!("Please enter the ID of the actor you want to search for:");
    let mut actor_id_str = String::new();
    reader
        .read_line(&mut actor_id_str)
        .expect("Failed to read line");
    let actor_id = actor_id_str.trim().parse::<usize>().unwrap();

    match get_actor_by_id(actors, actor_id) {
        None => {
            writeln!(writer, "No actor found with id: {} \nTry again!", actor_id).unwrap();
            get_unique_actor_by_id(reader, writer, actors)
        }

        Some(actor) => {
            writeln!(writer, "Found actor: {}", actor).unwrap();
            actor.id
        }
    }
}

pub fn get_unique_actor<R, W>(mut reader: R, mut writer: W, actors: &HashMap<usize, Actor>) -> usize
where
    R: BufRead,
    W: Write,
{
    let mut actor_name = String::new();
    reader
        .read_line(&mut actor_name)
        .expect("Failed to read line");
    let actor_name = actor_name.trim().to_ascii_lowercase();
    let selected_actors = get_actor_by_name(actors, &actor_name);
    match selected_actors.len() {
        0 => {
            // use fuzzy search to find similar names
            let similar_names =
                fuzzy_search_actor(actors.values().map(|actor| &actor.full_name), &actor_name);
            writeln!(
                writer,
                "No actor found with name: {} \nHere are similar matches:",
                actor_name
            )
            .unwrap();
            for name in similar_names {
                writeln!(writer, "{}", name).unwrap();
            }
            writeln!(writer, "Try again!\n").unwrap();
            get_unique_actor(reader, writer, actors)
        }
        1 => {
            let actor = selected_actors.into_iter().next().unwrap();
            writeln!(writer, "Found actor: {}", actor).unwrap();
            actor.id
        }
        n => {
            writeln!(
                writer,
                "Found {} actors with name: {} \nPlease specify which actor by their ID.",
                n, actor_name
            )
            .unwrap();
            // print out actors
            for actor in selected_actors {
                writeln!(writer, "{}", actor).unwrap();
            }
            get_unique_actor_by_id(reader, writer, actors)
        }
    }
}

#[cfg(test)]
mod test {

    use crate::data::{Actor, MovieDBBuilder};
    use std::path::PathBuf;

    use super::*;

    fn make_test_actors(actor_file: &str) -> HashMap<usize, Actor> {
        let actor_file = PathBuf::from(actor_file);
        let mut actors = MovieDBBuilder::read_actors(&actor_file).unwrap_or_else(|err| {
            panic!(
                "Problem reading actors from file {:?}: {:?}",
                &actor_file, err
            );
        });
        // push a second tom cruise entry
        let tom_cruise = Actor {
            id: 1,
            full_name: "tom cruise".to_string(),
            birth_year: Some(2006),
        };
        actors.insert(1, tom_cruise);
        actors
    }

    #[test]
    fn data_read_actor() {
        let actors = make_test_actors("data/new_small/actors.csv");
        let result = HashSet::from([
            Actor {
                id: 129,
                full_name: "tom cruise".to_string(),
                birth_year: Some(1962),
            },
            Actor {
                id: 1,
                full_name: "tom cruise".to_string(),
                birth_year: Some(2006),
            },
        ]);

        assert_eq!(result, get_actor_by_name(&actors, "tom cruise"))
    }

    #[test]
    fn test_get_id() {
        let actors = make_test_actors("data/new_small/actors.csv");
        let result = 129;
        let actor = get_actor_by_id(&actors, 129);
        assert_eq!(result, actor.unwrap().id)
    }

    #[test]
    fn test_cli_id() {
        let actors = make_test_actors("data/new_small/actors.csv");
        let result = 129;
        let reader = b"20947\n129";
        let mut writer = Vec::new();
        let actor = get_unique_actor_by_id(&reader[..], &mut writer, &actors);
        assert_eq!(result, actor);
    }

    #[test]
    fn get_tom() {
        let actors = make_test_actors("data/new_small/actors.csv");
        let input = b"Tom cruise\n129";
        let mut output = Vec::new();
        let id = get_unique_actor(&input[..], &mut output, &actors);
        assert_eq!(id, 129);
    }
    #[test]
    fn get_hanks() {
        let actors = make_test_actors("data/new_small/actors.csv");
        let input = b"ToM hAnKs";
        let mut output = Vec::new();
        let id = get_unique_actor(&input[..], &mut output, &actors);
        assert_eq!(id, 158);
    }

    #[test]
    //ignore because long
    #[ignore = "takes too long"]
    fn get_fuzzy_penelope() {
        let actors = make_test_actors("data/new_large/actors.csv");
        let names: HashSet<_> =
            fuzzy_search_actor(actors.values().map(|actor| &actor.full_name), "Tom Cruise")
                .into_iter()
                .collect();
        let matches = [
            "tom cruise".to_string(),
            "tom kruse".to_string(),
            "tom cruise".to_string(),
        ];
        assert_eq!(names, matches.iter().collect::<HashSet<_>>());
    }
}
