fn main() {
    #[derive(Debug)]
    enum Gender {
        Male,
        Female,
    }
    let aa = Gender::Male;

    match aa {
        Gender::Male => 1,
        Gender::Female => 2,
    };
    
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let x = Some(5);
    let y = Some(7);
    let sum: Option<i32> = x + y;
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}