pub trait AGraph {
    type Node;
    type Edge;

    // methods would go here
}


pub fn traverse(_graph: &AGraph<Node=usize, Edge=(usize, usize)>) {
    // ...snip...
}

fn main() {}