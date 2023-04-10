// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.



struct ColorClassicStruct {
    red: i32,
    green: i32,
    blue: i32,
}

struct ColorTupleStruct(i32, i32, i32);

#[derive(Debug)] 
//利用trait中给的Debug来它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。
//不过我们必须为结构体显式选择这个功能。为此，在结构体定义之前加上外部属性 #[derive(Debug)]
// {:?}横着输出 or {:#?} 竖着输出
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct{
            red: 0,
            green: 255,
            blue: 0,
        };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct(0, 255, 0);
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);// 像是一种填充
        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
