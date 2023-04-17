// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.


fn main() {
    my_macro!();
}
#[macro_export]//需要导出，如果未导出，则在顺序编译系统时，无法在编译器无法使用
//#[macro_export] 注释将宏进行了导出，这样其它的包就可以将该宏引入到当前作用域中，然后才能使用。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!")
    };
}
