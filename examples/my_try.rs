use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    // let ret = f3(f2(f1("hello")?)?)?;
    let ret = my_try!(f3(my_try!(f2(my_try!(f1("hello"))))));
    println!("Final result: {}", ret);
    Ok(())
}

fn f1(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1: {}", s.as_ref()))
}

fn f2(s: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2: {}", s.as_ref()))
}

/**
 * 函数f3用于演示如何返回一个错误结果。
 *
 * 参数s: 实现了AsRef<str>特化的任意类型。这允许函数接受字符串字面量、字符串切片或其他可以转换为字符串引用的类型。
 * 返回值: 返回一个Result类型，其中包含一个错误消息。这是因为在实际应用中，函数可能会遇到各种失败的情况，需要向调用者报告错误。
 *
 * 该函数通过直接构造一个错误消息并返回一个Err实例来模拟一个失败的场景。这在实际开发中常见于处理外部资源访问、验证失败等情况。
 */
fn f3(s: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3: {}", s.as_ref()))
}

#[macro_export]
macro_rules! my_try {
    ($e:expr) => {{
        match $e {
            Ok(val) => val,
            Err(err) => {
                return Err(err.into());
            }
        }
    }};
}
