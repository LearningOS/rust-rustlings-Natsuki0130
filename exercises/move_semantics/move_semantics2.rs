// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.



fn main() {
    let mut vec0: Vec<i32> = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
//
fn fill_vec(ved: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = ved;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
    //也可以直接对利用该函数直接借用vec0并对其进行填充
    //vec.clone()
    //不让其传入vec0，用类似move_semanyics4的方式来直接新建一个Vec<>来赋值
}
