fn main() {
    // ANCHOR: here
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // ANCHOR_END: here
}
