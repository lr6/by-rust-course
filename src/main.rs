/**
 * 规则总结
 * 1. 同一时刻，你只能拥有一个可变引用或者多个不可变引用
 * 2. 引用必须总是有效的
 */

 fn main() {
   let x = 5;
   // y是x的一个引用
   let y = &x;
   assert_eq!(5, x);
   // *y 解出引用所指向的值（也叫做解引用）
   assert_eq!(5, *y);
   quote();
   mut_quote();
   mult_quote();
   all_quote();
   majorzation();
   dangle();
}

// 不可变引用
fn quote() {
   let s = String::from("hello");
   let len = calculate_length(&s);
   println!("{} 的 length = {}", s, len);
}

fn calculate_length(s: &String) -> usize {
   s.len()
}

// 可变引用
// 同一作用域，特定数据只能有一个可变引用
fn mut_quote() {
   let mut s = String::from("hello");
   change_str(&mut s);

   println!("s={}", s);
}

fn change_str(s: &mut String) {
   s.push_str(", world");
}

fn mult_quote() {
   let mut str = String::from("Hello");
   let s1 = &mut str;
   println!("s1 = {}", s1);
   // 第二个可变借用需要在第一个可变借用使用之前
   // 这样避免了数据竞争,数据竞争行为会带来下面3个行为
   // 1. 两个或更多的指针同时访问同一数据
   // 2. 至少有一个指针被用来写入数据
   // 3. 没有同步数据访问的机制
   let s2 = &mut str;
   println!("s2 = {}", s2);
}

/**
 * 同一作用域，可变借用和不可变引用不可同时存在
 * 不可变引用可以存在多份，可变引用不行
 */
fn all_quote() {
   let mut str = String::from("Hello");
   let s1 = &str;
   let s2 = &str;
   println!("{}, {}", s1, s2);
   {
      let s3 = &mut str;
      println!("{}", s3);
   }
}

// 编译器也一直在优化
// 对于编译器优化行为叫做：Non-Lexical Lifetimes(NLL) 专门用于找到某个引用在作用域（}）结束前，就不在使用的代码位置
fn majorzation() {
   let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束

/**
 * 悬垂引用（Dangling References）也叫做悬垂指针
 * 意思是某个指针指向某个值后，这个值后面被释放了，而指针仍然存在，指针指向的空间可能
 * 没有任何值或者已经被其他变量所使用
 */
 fn dangle() -> String { // dangle 返回一个字符串的引用
 
     let s = String::from("hello"); // s 是一个新字符串
 
     //&s // 返回字符串 s 的引用
     s // 返回s就好了
 } // 这里 s 离开作用域并被丢弃。其内存被释放。
   // 危险！
 
 