pub trait GGraph<Node, Edge> {
    fn distance<N, E, G: GGraph<N, E>>(_graph: &G, _start: &N, _end: &N) -> u32 {
        0
    }
}

pub trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}

pub fn distance_g<N, E, G: GGraph<N, E>>(_graph: &G, _start: &N, _end: &N) -> u32 {
    // ...snip...
    0
}

pub fn distance_a<G: AGraph>(_graph: &G, _start: &G::Node, _end: &G::Node) -> u32 {
    0
}

fn main() {}
