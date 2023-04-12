// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());//slice.to_owned() 将slice的副本作为一个全新分配的String返回
    string("nice weather".into());// 类型转换 into() 属于trait中的内容 将传入值转为impl中的值
    string(format!("Interpolation {}", "Station"));// 加法
    string_slice(&String::from("abc")[0..1]);//切片 
    string_slice("  hello there ".trim());// 修剪 从字符串开头和末尾去掉内容（通常是空白符） 其中 slice.trim_left()只省略开头的空白符，而slice.trim_right反之 其返回值是slice的一个子切片
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//slice.replace(pattern,replacement) 返回以relpacement替换所有匹配pattern的内容之后得到新的string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());// 大小写转换，返回slice文本
}
