use crate::data::MovieDB;
// TODO: make nodes hashable and store them in hashsets or b-trees instead of hashmaps or vectors
//derive debug

type NodeIndex = usize;
type ActorId = usize;
type MovieId = usize;

#[derive(Debug, Clone)]
pub struct Node {
    pub actor_id: ActorId,
    parent_index: Option<NodeIndex>, // optional actor_id of parent node
    pub movie_id: Option<MovieId>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.actor_id == other.actor_id && self.movie_id == other.movie_id
    }
}

pub struct Graph {
    frontier: Vec<NodeIndex>,
    explored: Vec<ActorId>, // actor ids
    nodes: Vec<Node>,
}

impl Node {
    pub fn new(actor_id: usize, parent_index: Option<NodeIndex>, movie_id: Option<usize>) -> Node {
        Node {
            actor_id,
            parent_index,
            movie_id,
        }
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            frontier: Vec::new(),
            explored: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let max_index = self.nodes.len();
        self.nodes.push(node);
        self.frontier.push(max_index);
    }

    pub fn add_nodes(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            self.add_node(node);
        }
    }

    pub fn get_neighbours(&self, node_index: NodeIndex, movie_db: &MovieDB) -> Vec<Node> {
        let parent_actor_id = self.nodes[node_index].actor_id;
        let mut neighbours = Vec::new();
        // get movies of actor
        let movies_of_actor = movie_db
            .actor_to_movies
            .get(&parent_actor_id)
            .expect("actor not found");
        for movie_id in movies_of_actor {
            // get actors of movie
            let actors_of_movie = movie_db
                .movie_to_actors
                .get(movie_id)
                .expect("movie not found");
            for actor_id in actors_of_movie {
                // continue if actor is same as parent
                if self.explored.contains(actor_id) {
                    //assuming parent is in explored
                    continue;
                }
                let node = Node::new(*actor_id, Some(node_index), Some(*movie_id));
                neighbours.push(node);
            }
        }
        neighbours
    }

    pub fn get_path_to_root(&self, mut node_index: NodeIndex) -> Vec<Node> {
        let mut path = Vec::new();
        while let Some(parent_id) = self.nodes[node_index].parent_index {
            path.push(self.nodes[node_index].clone());
            node_index = parent_id;
        }
        path.push(self.nodes[node_index].clone());
        path.reverse();
        path
    }
}

pub async fn shortest_path(
    source_actor_id: ActorId,
    target_actor_id: ActorId,
    movie_db: &MovieDB,
) -> Result<Vec<Node>, &'static str> {
    // make empty new graph
    let mut graph = Graph::new();
    // add source node to frontier
    let source_node = Node::new(source_actor_id, None, None);
    graph.add_node(source_node);
    let mut num_explored: usize = 0;
    while !graph.frontier.is_empty() {
        let node_index = graph.frontier.remove(0);
        let node = &graph.nodes[node_index];
        // add current node actor id to explored list
        graph.explored.push(node.actor_id);

        // yield control to event loop
        if num_explored % 1000 == 0 {
            tokio::task::yield_now().await;
        }

        // get neighbours of node
        let neighbours = graph.get_neighbours(node_index, movie_db);
        // check if any neighbour is target
        if let Some(neighbour) = neighbours
            .iter()
            .find(|neighbour| neighbour.actor_id == target_actor_id)
        {
            // add this node to the arena
            graph.add_node(neighbour.clone());
            println!("nodes created: {}", graph.nodes.len());
            println!("nodes explored: {}", num_explored);
            let path = graph.get_path_to_root(graph.nodes.len() - 1);
            return Ok(path);
        }

        graph.add_nodes(neighbours);
        num_explored += 1;
    }
    Err("no path found")
}
#[cfg(test)]
mod test {
    use super::*;
    use crate::data::{MovieDB, MovieDBBuilder};
    use std::path::PathBuf;

    fn make_test_db() -> MovieDB {
        let data_dir = PathBuf::from("data/new_small");
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

    #[test]
    fn adding_nodes() {
        let mut graph = Graph::new();
        let node1 = Node::new(1, None, None);
        graph.add_node(node1.clone());
        let node2 = Node::new(2, Some(0), Some(8));
        graph.add_node(node2.clone());
        let node3 = Node::new(3, Some(1), Some(7));
        graph.add_node(node3.clone());

        assert_eq!(graph.nodes, vec![node1, node2, node3]);
        assert_eq!(graph.frontier, vec![0, 1, 2])
    }
    #[test]
    fn graph_path_to_root() {
        let mut graph = Graph::new();
        let node1 = Node::new(1, None, None);
        graph.add_node(node1.clone());
        let node2 = Node::new(2, Some(0), Some(8));
        graph.add_node(node2.clone());
        let node3 = Node::new(3, Some(1), Some(7));
        graph.add_node(node3.clone());

        let path = graph.get_path_to_root(2);
        assert_eq!(path, vec![node1, node2, node3]);
    }

    #[test]
    fn test_neighbours() {
        let db = make_test_db();
        let mut graph = Graph::new();
        let tom_cruise = Node::new(129, None, None);
        graph.explored.push(tom_cruise.actor_id);
        graph.add_node(tom_cruise);

        let neighbours = graph.get_neighbours(0, &db); // Tbom Cruise

        let mut neighbour_ids = neighbours
            .iter()
            .map(|node| node.actor_id)
            .collect::<Vec<usize>>();
        neighbour_ids.sort();

        assert_eq!(neighbour_ids, vec![102, 163, 193, 197, 420, 596520]);
    }

    #[tokio::test]
    async fn test_shortest_path_cruise_nicholson() {
        let db = make_test_db();
        let source_id = 129; // tom cruise
        let target_id = 197; // Jack Nicholson

        let tom_cruise = Node::new(source_id, None, None);
        let jack_nicholson = Node::new(target_id, Some(0), Some(104257));

        let path = shortest_path(129, target_id, &db).await;
        assert_eq!(path, Ok(vec![tom_cruise, jack_nicholson]));
    }

    #[tokio::test]
    async fn test_shortest_path_cruise_hanks() {
        let db = make_test_db();
        let source_id = 129;
        let target_id = 158; // Tom Hanks

        let tom_cruise = Node::new(source_id, None, None);
        let connector = Node::new(102, Some(0), Some(104257));
        let hanks = Node::new(target_id, Some(102), Some(112384));

        let path = shortest_path(source_id, target_id, &db).await;
        assert_eq!(path, Ok(vec![tom_cruise, connector, hanks]));
    }
}
