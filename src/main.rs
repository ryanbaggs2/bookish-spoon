use std::collections::{HashMap, HashSet};

fn main() {
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();

    let start = 'H';

    graph.insert( 'A', Vec::from(['B', 'G']) );
    graph.insert( 'B', Vec::from(['A', 'C', 'D', 'E']) );
    graph.insert( 'C', Vec::from(['B']) );
    graph.insert( 'D', Vec::from(['B']) );
    graph.insert( 'E', Vec::from(['B','F']) );
    graph.insert( 'F', Vec::from(['E']) );
    graph.insert( 'G', Vec::from(['A', 'H']) );
    graph.insert( 'H', Vec::from(['G', 'I']) );
    graph.insert( 'I', Vec::from(['H']) );

    let results = depth_first_search(graph, start, 'C');

    match results {
        Some(vertices) => {
            for vertex in vertices {
                println!("{vertex}");
            }
        },
        None => {
            println!("Found nothing.");
        }
    }
}

fn depth_first_search(
    graph: HashMap<char, Vec<char>>,
    start: char,
    search: char
) -> Option<Vec<char>> {
    // Put first vertex onto stack.
    let mut stack: Vec<char> = vec![start];

    // Track the discovered vertices.
    let mut discovered: HashSet<char> = HashSet::new();

    // Loop until stack is empty.
    while !stack.is_empty() {

        // Check if vertex has been discovered.
        if discovered.contains(stack.last().unwrap()) {
            // Vertex discovered and adjacency list already added/searched, remove from stack.
            stack.pop();
            continue;
        }

        // Mark the vertex as discovered.
        discovered.insert(*stack.last().unwrap());

        // See if it is the search value.
        if *stack.last().unwrap() == search {
            // Found it, return the path taken.
            return Some(stack);
        }

        // Add the adjacency list to the stack.
        for vertex in graph.get(stack.last().unwrap()).unwrap().iter() {
            stack.push(*vertex);
        }
    }

    return None;
}