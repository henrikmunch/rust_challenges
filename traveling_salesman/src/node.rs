        // ***** NODE ***** 

use rand::Rng;

// The 'Node` type refers to a city with index id and certain (x,y) coordinates
#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

// Implement a functionality for the type `Node`
impl Node {

    // The Euclidean distance (try a more fun metric? xD) between two nodes
    pub fn node_distance(&self, node: &Node) -> f64 {
        let dx = self.x - node.x;
        let dy = self.y - node.y;
        (dx*dx + dy*dy).sqrt()
    }

}

// Function to generate random nodes
pub fn random_nodes(n: usize) -> Vec<Node> {
    let mut nodes = Vec::new(); 
    let mut rng = rand::thread_rng(); 

    // Generate nodes with id in [0,n-1] and random (x,y) in [0,1]^2
    for id in 0..n {
        let x: f64 = rng.gen(); 
        let y: f64 = rng.gen();

        // Create new node, then push it to the vector `nodes`
        nodes.push(Node { id, x, y });
    }

    nodes 
}
