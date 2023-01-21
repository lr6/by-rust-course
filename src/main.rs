/**
 * 函数要点
 * 1. 函数名和变量名使用 蛇形命名法(sneak case)
 * 2. 函数的位置，rust只关心你是否定义了函数，不在乎你在哪里定义的
 * 3. 每个函数都需要标注类型
 * 
 * 单元类型 ()，是一个零长度的元组
 * 函数没有返回值，就返回一个单元类型()
 * 通过分号结尾的表达式，放回一个单元类型()
 */

 fn main() {
    another_fn(5, 6);
 }

fn another_fn(x: i32, y: i32) {
   println!("x={}", x);
   println!("y={}", y);
}

// 特别的，这种语法往往用做会导致程序崩溃的函数
// 发散函数，永不返回
fn _diverge_fn() -> ! {
   panic!("拜拜吧")
}