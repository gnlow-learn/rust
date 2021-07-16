fn main() {
    println!("{} days", 31);
    println!("{0}, World! {0}, {1}!", "Hello", "Rust");
    println!("{s} {v} {o}",
        v="jumps over",
        s="the quick brown fox",
        o="the lazy dog"
    );

    let pi = 3.14;
    println!("{}", pi);
}