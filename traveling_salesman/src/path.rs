        // *****  PATH ***** 

use rand::Rng;
use crate::node::Node;

// The `Path` type refers to a trip around all nodes
#[derive(Clone, Debug)]
pub struct Path {
    pub nodes: Vec<Node>,
    pub distance: f64,
}

// Implement functionalities for the type `Path`
impl Path {

    // Create a path from a list of nodes (in main.rs, we have nodes = random_nodes(n))
    pub fn new(nodes: Vec<Node>) -> Path {
        let mut path = Path {nodes, distance: 0.0 };
        path.path_distance();

        path
    }

    // Calculate the distance of an entire path
    pub fn path_distance(&mut self) {
        let mut distance = 0.0;
        let n_nodes = self.nodes.len();

        // Add up the distances between successive nodes
        for n in 0..n_nodes {

            // Using remainder `%` because n=n_node should wrap around to n=0
            let next_n = (n+1) % n_nodes;

            // The RHS is the distance between node[n] and node[n+1]
            distance += self.nodes[n].node_distance(&self.nodes[next_n]);
        }

        // Update the total distance
        self.distance = distance;
    }

    // Mutate a path by swapping two nodes
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let n_nodes = self.nodes.len();

        // Two random node indices
        let id_1 = rng.gen_range(0..n_nodes);
        let id_2 = rng.gen_range(0..n_nodes);

        // Swap the two nodes
        self.nodes.swap(id_1, id_2);

        // Update the path distance
        self.path_distance();
    }

}
