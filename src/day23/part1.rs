use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // Read input from input.txt
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    // Parse the connections into a graph representation
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let nodes: Vec<&str> = line.trim().split('-').collect();
        if nodes.len() != 2 {
            continue;
        }
        let (a, b) = (nodes[0], nodes[1]);

        graph.entry(a).or_insert_with(HashSet::new).insert(b);
        graph.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    // Find all triangles in the graph
    let mut triangles: HashSet<Vec<&str>> = HashSet::new();

    for (&node, neighbors) in &graph {
        for neighbor1 in neighbors {
            for neighbor2 in neighbors {
                // Ensure that the triangle is unique
                if neighbor1 < neighbor2
                    && graph
                        .get(neighbor1)
                        .map_or(false, |s| s.contains(neighbor2))
                {
                    let mut triangle = vec![node, neighbor1, neighbor2];
                    // Sort the nodes in the triangle to ensure uniqueness
                    triangle.sort();
                    triangles.insert(triangle);
                }
            }
        }
    }

    // Filter triangles where at least one node starts with 't'
    let triangles_with_t: Vec<&Vec<&str>> = triangles
        .iter()
        .filter(|triangle| triangle.iter().any(|&node| node.starts_with('t')))
        .collect();

    println!("Total triangles: {}", triangles_with_t.len());
}
