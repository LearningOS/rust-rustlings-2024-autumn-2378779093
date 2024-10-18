// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("my_option is None, cannot call it!");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 创建一个空 Vec，并声明为可变
    let mut my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5]; // 初始化 Vec
    my_empty_vec.clear(); // 使用 clear() 来清空 Vec
    println!("This Vec is empty, see? {:?}", my_empty_vec); // 打印实际的空 Vec

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换这两个值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}

