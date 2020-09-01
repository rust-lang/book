// ANCHOR: here
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(u8, u8, u8); // tuple struct
                                          // ANCHOR_END: here

fn main() {}
