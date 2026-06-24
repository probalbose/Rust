fn main() {
    // s1 OWNS a heap-allocated String.
    let mut s1 = String::from("order book");

    let _r = &mut s1;

    // print_it(&s1);
    // print_it(&s1);

    add_bang(&mut s1);

    // println!("{r}");

    // This MOVES ownership s1 -> s2. After this line, s1 is invalid.
    // let s2 = s1;

    // println!("{s2}"); // fine: s2 is the owner now

    // EXERCISE: uncomment the next line, run `cargo run`, and read the error.
    // println!("{s1}");

    // println!("still own it: {s1}");
}

fn _print_it(s: &String) {
    println!("{s}");
}

fn add_bang(s: &mut String) {
    s.push_str("!");
}
