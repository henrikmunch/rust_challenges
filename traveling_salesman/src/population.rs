        // *****  POPULATION ***** 

use rand::{Rng, seq::SliceRandom};
use crate::path::Path;
use crate::node::Node;
use crate::TOP_FITTEST;
use crate::POPULATION_SIZE;
use crate::MUTATION_RATE;

// The `Population` type refers to a collection of paths
// Genetic metaphor: 
// - path = gene
// - population = chromosome/collection of genes
#[derive(Clone, Debug)]
pub struct Population {
    pub paths: Vec<Path>,
}

// Implement functionalities for the type `Population`
impl Population {
    
    // Create a new population (this is an "associated function" because there is no &self)
    pub fn new(nodes: &Vec<Node>) -> Population {
        let mut paths = Vec::new();

        // Suppose [`node_0`, ..., `node_(n-1)`] is a deck of cards
        // Then we create the population by shuffling the deck of cards `POPULATION_SIZE` many times
        for _ in 0..POPULATION_SIZE {
            let mut all_nodes = nodes.clone();
            let mut rng = rand::thread_rng();
            all_nodes.shuffle(&mut rng); 
            paths.push( Path::new(all_nodes) );
        }

        Population { paths }
    }

    // The fittest path has the smallest distance
    // So we iterate over every pair of nodes (`node_1`, `node_2`) and compare the distances
    pub fn find_fittest_path(&self) -> &Path {
        self.paths
            .iter()
            .min_by(
                |node_1, node_2| 
                node_1.distance.partial_cmp(&node_2.distance).unwrap()
            )
            .unwrap()
    }

    // Crossover between two paths (`parent_1` and `parent_2`), thereby creating a new path (`child`)
    // Example: 
    // - parent_1 = [02|341|5] ("DNA cuts" indicated by `|`), 
    // - parent_2 = [012345], then
    // - child = [(341)(025)], where (341) came from parent_1 and (025) came from parent_2
    pub fn crossover(&self, parent_1: &Path, parent_2: &Path) -> Path {
        let mut rng = rand::thread_rng();
        let n_nodes = parent_1.nodes.len();
        let mut child = Vec::new();

        // Cut the "DNA string" at random `start` and `end` "gene" sites
        let start = rng.gen_range(0..n_nodes);
        let end = rng.gen_range(start..n_nodes);

        // Take a subset of nodes ("genes") from `parent_1`
        for node in start..end {
            child.push( parent_1.nodes[node].clone() );
        }

        // The remaining nodes ("genes") are now filled in by `parent_2`
        // Must ensure that there are no duplicate genes, since we visit each city/node/gene once
        for node_2 in &parent_2.nodes {
            if !child.iter().any(|node_1| node_1.id == node_2.id) {
                child.push( node_2.clone() );
            }
        }

        Path::new(child)
    }

    // Find two parents by picking two random paths in the population
    // pub fn find_random_parents(&self) -> (&Path, &Path) {
    //     let mut rng = rand::thread_rng();
    //     let n_paths = self.paths.len();
    //     let id_1 = rng.gen_range(0..n_paths);
    //     let id_2 = rng.gen_range(0..n_paths);
    //     let parent_1 = &self.paths[id_1];
    //     let parent_2 = &self.paths[id_2];

    //     (parent_1, parent_2)
    // }

    // Find two parents among the TOP_FITTEST many most fittest paths
    // There is not enough randomness if we had just picked the top 2 fittest
    pub fn find_fittest_parents(&self) -> (Path, Path) {
        // Panic! at the Disco
        assert!(POPULATION_SIZE >= TOP_FITTEST, "POPULATION_SIZE too small compared to TOP_FITTEST");

        // Sort by iterating over all pairs of paths according to distance
        let mut sorted_paths = self.paths.clone();
        sorted_paths.sort_by(
            |path_1, path_2| path_1.distance.partial_cmp(&path_2.distance).unwrap()
        );

        // Two random indices between 0 and TOP_FITTEST
        let mut rng = rand::thread_rng();
        let id_1 = rng.gen_range(0..TOP_FITTEST);
        let id_2 = rng.gen_range(0..TOP_FITTEST);

        // A better Rust programmer could probably avoid the cloning here ...
        let parent_1 = sorted_paths[id_1].clone();
        let parent_2 = sorted_paths[id_2].clone();

        (parent_1, parent_2)
    }

    // Natural selection step
    // Here we evolve the population based on crossovers + mutations
    pub fn evolve(&mut self) {
        let mut new_paths = Vec::new();

        while new_paths.len() < POPULATION_SIZE {

            // Crossover
            // let (parent_1, parent_2) = self.find_random_parents();
            let (parent_1, parent_2) = self.find_fittest_parents();
            let mut child = self.crossover(&parent_1, &parent_2);

            // Mutuate with probability MUTATION_RATE
            let mut rng = rand::thread_rng();
            let random_number: f64 = rng.gen::<f64>();
            if random_number < MUTATION_RATE {
                child.mutate();
            }

            new_paths.push(child);
        }

        // Update the paths
        self.paths = new_paths;
    }

}
