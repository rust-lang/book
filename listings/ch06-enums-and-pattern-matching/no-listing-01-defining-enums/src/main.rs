// ANCHOR: def
enum IpAddrKind {
    V4,
    V6,
}
// ANCHOR_END: def

fn main() {
    // ANCHOR: instance
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // ANCHOR_END: instance

    // ANCHOR: fn_call
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    // ANCHOR_END: fn_call
}

// ANCHOR: fn
fn route(ip_kind: IpAddrKind) {}
// ANCHOR_END: fn
