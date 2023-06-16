use std::collections::{HashSet, VecDeque};

fn depth_first_search(graph: &Graph, start: Vertex, objective: Vertex) -> Option<Vec<u32>>{
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    // Get first vertex of queue while there are still vertices in it.
    while let Some(current) = queue.pop_front(){
        // Add the current vertex to the history of visited vertices.
        history.push(current.value());

        // Check if current vertex is the objective.
        if current == objective {
            // Return the Optional with history of visited vertices.
            return Some(history);
        }

        // Loop through all vertices adjacent to the current vertex.
        for adjacent in current.adjacent_vertices(graph).into_iter() {
            // Insert adjacent vertex into visited if not visited yet.
            if visited.insert(adjacent) {
                // Add the adjacent vertex to the front of the queue.
                queue.push_front(adjacent);
            }
        }
    }

    None
}

// Data Structures.

#[derive(Copy, Clone, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl From<u32> for Vertex {
    fn from(value: u32) -> Self {
        Vertex(value)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn adjacent_vertices(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1)
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(value: (u32, u32)) -> Self {
        Edge(value.0, value.1)
    }
}
