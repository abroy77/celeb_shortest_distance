use crate::data::{Actor, MovieDBBuilder};
use std::collections::{HashMap, HashSet};
/// module for handline stdin inpt and otput for the program
use std::io::{BufRead, Write};

fn get_actor_by_name(actors: &HashMap<usize, Actor>, name: &str) -> HashSet<Actor> {
    let selected_actors = actors
        .into_iter()
        .filter(|(_, actor)| actor.name == name)
        .map(|(_, actor)| actor.clone())
        .collect();
    selected_actors
}

fn get_actor_by_id(actors: &HashMap<usize, Actor>, id: usize) -> Option<Actor> {
    actors.get(&id).map(|actor| actor.clone())
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

    match get_actor_by_id(&actors, actor_id) {
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
    println!("Please enter the name of the actor you want to search for:");
    let mut actor_name = String::new();
    reader
        .read_line(&mut actor_name)
        .expect("Failed to read line");
    let actor_name = actor_name.trim().to_ascii_lowercase();
    let selected_actors = get_actor_by_name(&actors, &actor_name);
    match selected_actors.len() {
        0 => {
            writeln!(
                writer,
                "No actor found with name: {} \nTry again!",
                actor_name
            )
            .unwrap();
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

    use std::path::PathBuf;

    use super::*;

    fn make_test_actors() -> HashMap<usize, Actor> {
        let actor_file = PathBuf::from("data/new_small/actors.csv");
        let mut actors = MovieDBBuilder::read_actors(&actor_file).unwrap_or_else(|err| {
            panic!(
                "Problem reading actors from file {:?}: {:?}",
                &actor_file, err
            );
        });
        // push a second tom cruise entry
        let tom_cruise = Actor {
            id: 1,
            name: "tom cruise".to_string(),
            birth_year: Some(2006),
        };
        actors.insert(1, tom_cruise);
        actors
    }

    #[test]
    fn data_read_actor() {
        let actors = make_test_actors();
        let result = HashSet::from([
            Actor {
                id: 129,
                name: "tom cruise".to_string(),
                birth_year: Some(1962),
            },
            Actor {
                id: 1,
                name: "tom cruise".to_string(),
                birth_year: Some(2006),
            },
        ]);

        assert_eq!(result, get_actor_by_name(&actors, "tom cruise"))
    }

    #[test]
    fn test_get_id() {
        let actors = make_test_actors();
        let result = 129;
        let actor = get_actor_by_id(&actors, 129);
        assert_eq!(result, actor.unwrap().id)
    }

    #[test]
    fn test_cli_id() {
        let actors = make_test_actors();
        let result = 129;
        let reader = b"20947\n129";
        let mut writer = Vec::new();
        let actor = get_unique_actor_by_id(&reader[..], &mut writer, &actors);
        assert_eq!(result, actor);
    }

    #[test]
    fn get_tom() {
        let actors = make_test_actors();
        let input = b"Tom cruise\n129";
        let mut output = Vec::new();
        let id = get_unique_actor(&input[..], &mut output, &actors);
        assert_eq!(id, 129);
    }
    #[test]
    fn get_hanks() {
        let actors = make_test_actors();
        let input = b"ToM hAnKs";
        let mut output = Vec::new();
        let id = get_unique_actor(&input[..], &mut output, &actors);
        assert_eq!(id, 158);
    }
}
