use std::fmt::Display;

fn main() {
    fn longest_with_ann
        <'a, T>
        (x: &'a str, y:&'a str, ann: T)
        -> &'a str
        where T: Display 
    {
        println!("Announcement!: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    println!("{}", longest_with_ann("123", "4567", "Woah!"));
    
}
