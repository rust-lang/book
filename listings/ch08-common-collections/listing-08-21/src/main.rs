fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();
    // ANCHOR_END: here
}
