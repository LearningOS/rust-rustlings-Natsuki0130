// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option.iter() {
        res += x;
    }
    println!("{}", res);
}
//那么你可以使用 .iter() 方法来迭代这个枚举值的所有可能取值。
