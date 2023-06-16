use std::collections::VecDeque;

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

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
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
