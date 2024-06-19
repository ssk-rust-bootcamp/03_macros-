use macros::my_vec;

fn main() {
    println!("Hello, world!");
    let v = my_vec!(1, 2, 3, 4, 5);
    println!("v: {:?}", v)
}
