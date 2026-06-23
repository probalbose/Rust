fn main() {
    // s1 OWNS a heap-allocated String.
    let s1 = String::from("order book");

    // This MOVES ownership s1 -> s2. After this line, s1 is invalid.
    let s2 = s1;

    println!("{s2}"); // fine: s2 is the owner now

    // EXERCISE: uncomment the next line, run `cargo run`, and read the error.
    // println!("{s1}");
}
