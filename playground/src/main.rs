fn main() {
    another_function(five(), 6);
    let bb = String::from("hi");
    test(&bb);
    println!("{}", bb); // borrow of moved value

    let mut s = String::from("Hello");
    let word = first_word(&s);
    // s.clear();
    println!("{}", word);

    def_user()
}

fn another_function(x: i8, y: i8) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i8 {
    5
}

fn test(a: &String) {
    let x = String::from("hi");
    let y = x;
    // println!("{} {}", x, y)
    println!("{}", a);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    };
    &s[..]
}

fn arr() {
    const a: &[i8] = &[1, 2, 3];
}

struct User {
    username: String,
    email: String,
    color: Color,
}
struct Color(u8, u8, u8);

fn def_user() {
    let mut user1 = User {
        email: String::from("a@a.com"),
        username: String::from("abcd"),
        color: Color(1,2,3),
    };
    user1.email = String::from("b@b.com");
}