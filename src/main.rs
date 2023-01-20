/**
 * 字符类型 char
 * 在rust语言中，不仅仅ASCII字符，所有的unicode值，都可以作为 rust的字符
 * rust 的字符只能用 '' 来表示，"" 是留给字符串的
 * 
 * 布尔 bool
 * 只有两个值，true 和 false， 布尔值占用内存一个字节
 * 
 * 单元类型
 * 只有一个值 就是 ()
 * main函数的返回值 就是 ()
 * println!()的返回值也是 ()
 * 没有返回的函数，叫做发散函数( diverge function )
 * 可以用 () 作为map的值，表示我们只关注key,不关注 value(不占用内存).
 */
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
    let x = '中';
    println!("{},{},{},{}", c, z, g, heart_eyed_cat);
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));

    bool();
}

fn bool() {
    let x = true;
    let y: bool = false;
    println!("x={}, y= {}", x, y);
}