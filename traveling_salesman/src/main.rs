        // ***** TRAVELING SALESMAN WITH GENETIC ALGORITHM *****

// Import modules defining three structs: node ∈ path ∈ population, where
// population = [path_1, ..., path_(POPULATION_SIZE)]
// path_i = [node_(i1), ..., node_(in)]
// node_(ij) = [id_(ij), x_(ij), y_(ij)]
mod node;
mod path;
mod population;

// My crates
use path::Path;
use population::Population;
use node::random_nodes;
// To save distance history for python
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

// Fixed parameters
pub const TOP_FITTEST: usize = 5;
pub const POPULATION_SIZE: usize = 10;
pub const GENERATION_SIZE: usize = 2000;
pub const MUTATION_RATE: f64 = 0.02;

// Make the output text easier on the eyes
fn print_seperators() {
    println!("{}", "-".repeat(90));
    println!("{}", "-".repeat(90));
}

// Here we go!
fn main() {

    print_seperators();


    // Number of nodes/cities/genes
    let n = 20;
    let nodes = random_nodes(n); 

            // Testing

    // Test if random_nodes works
    println!("\n \t testing random_nodes: \n");
    for node in &nodes {
        println!("node {:.3}: ({:.3}, {:.3})", node.id, node.x, node.y);
    }

    // Test if path_distance works on the path [0,1,...,n-1]
    let mut path = Path::new(nodes.clone());
    path.path_distance();
    println!("distance for the path [0...{}]: {:.3}", n-1, path.distance);

            // Genetic algorithm start here

    println!("\n\t running GA: \n");
    let mut population = Population::new(&nodes);
    // Initial best path before evolution (will get updated in the for loop below)
    let mut fittest = population.find_fittest_path().clone(); 
    // The nodes for every newfound, fittest path will be saved here
    let mut history = Vec::new();
    // The fittest distance at every iteration of the loop 
    let mut fittest_python = Vec::new(); 
    // We create a family tree with depth = GENERATION_SIZE
    for g in 0..GENERATION_SIZE {

        // Crossover + mutation
        population.evolve();
        let fittest_new = population.find_fittest_path();

        // Update the fittest path if a better one is found
        if fittest_new.distance < fittest.distance {
            fittest = fittest_new.clone(); 
            println!("fittest distance = {:.3} for generation {}", fittest.distance, g);

            // Add the nodes for the new fittest path to the history
            let mut fittest_nodes = Vec::new();
            for node in &fittest.nodes {
                fittest_nodes.push(node.id);
            }
            history.push(fittest_nodes);
        }

        // Save history to python
        fittest_python.push(fittest.distance);

    }

    // History of the best city paths
    println!("\nhistory:");
    for nodes in &history {
        println!("{:?}", nodes);
    }

    // Writing fittest_python to a file
    let file = File::create("fittest_python.txt").expect("File creation error :(");
    let mut buf_writer = BufWriter::new(file);
    for distance in fittest_python {
        writeln!(buf_writer, "{}", distance).expect("File writing error :(");
    }

    print_seperators();

}
