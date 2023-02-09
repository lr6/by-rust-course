/**
 * 元组: 由多种类型组成的复合类型。
 * 元组的长度固定，其中的顺序也是固定的。
 */

fn main() {
    let tup: (i32, f64, u8) = (500, 6.5, 1);
    // 解构取值
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    println!("tup的第二个值：{}", tup.1);
    let hello = String::from("hello");
    let (s2, len) = calculate_length(hello);
    println!("s2={}, len={}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
