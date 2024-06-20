use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right { a: u32 },
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}
fn main() {
    let up = Direction::Up(DirectionUp::new(10));
    let left: Direction = 10.into();
    println!("up:{:?},left:{:?}", up, left);
}
impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}
