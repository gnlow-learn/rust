#[derive(Debug)]
enum Data {
    Num(i32),
}

use Data::Num;
fn main() {
    fn double(data: Data) -> Data {
        match data {
            Num(n) => Num(n),
        }
    }
    println!("{:?}", double(Num(2)));
    let a = Num(123);
    println!("{:?}", double(a)); // 납치
    // println!("{:?}", a); // 납치당함

    fn double2(data: &Data) -> Data {
        match data {
            Num(n) => Num(n),
        }
    }
}