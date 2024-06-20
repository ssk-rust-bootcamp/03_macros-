use macros::AutoDeref;
#[allow(unused)]
#[derive(AutoDeref, Debug)]
#[deref(field = inner)]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };
    println!("{:?}", s);
}
