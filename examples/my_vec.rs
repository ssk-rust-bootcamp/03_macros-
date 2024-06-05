use anyhow::Result;

fn main() -> Result<()> {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    print!("{} ", v.len());
    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr;$n:expr)=>{
        std::vec::from_elem($n, $elem);
    };

    ($($x:expr),+ $(,)?) => {{
        // let mut temp_vec = Vec::new();
        // $(
        //   temp_vec.push($x);
        // )*
        // temp_vec
        <[_]>::into_vec(Box::new([$($x),*]))
    }};
}
