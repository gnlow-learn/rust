fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);

    let fourth = v.get(3);

    for i in &v {
        println!("{}", i);
    }

    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(10),
        Cell::Text(String::from("blue")),
    ];
}
