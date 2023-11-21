// `macro_rules!` 用于开始宏定义
// `vec_try` 为宏名称，使用时有 ! ，定义的时候没有
macro_rules! vec_try {
    // 对 () 内的内容进行匹配，以下有两个匹配分支
    // 1. 空分支
    // 返回一个 expression
    () => {
        Vec::new()
    };
    // 2. 含内容分支
    // $() 用于匹配括号内模式的值，以用于替换后面的代码
    // $x:expr 匹配任意表达式
    // , 标识这个 , 分隔符可有可无出现在匹配的 $() 代码后面
    // * 标识前面的模式，即 $() 模式零次或多次
    ($( $x:expr ),* ) => {
        // 再添加一个花括号，表示里边含有 statement
        // 最后需返回 expression
        {
            let mut temp_vec = Vec::new();
            // $()* 对匹配的模式重复操作
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec_try!(1, 2, 3);
    println!("{:?}", v);
    let mut v1: Vec<i32> = vec_try!();
    v1.push(1);
    println!("{:?}", v1);
}

// 编译过程
// #[macro_use]
// extern crate std;
// fn main() {
//     let v = {
//         let mut temp_vec = Vec::new();
//         temp_vec.push(1);
//         temp_vec.push(2);
//         temp_vec.push(3);
//         temp_vec
//     };
//     {
//         ::std::io::_print(format_args!("{0:?}\n", v));
//     };
//     let mut v1: Vec<i32> = Vec::new();
//     v1.push(1);
//     {
//         ::std::io::_print(format_args!("{0:?}\n", v1));
//     };
// }
