#[allow(unused_variables)]
/**
 * 切片: 引用集合中部分连续的元素序列
 */

fn main() {
    let s = String::from("hello world");
    // 右半开区间
    let hello = &s[0..5];
    let world = &s[6..];
    println!("hello={}, world={}", hello, world);
    boundary();
    demo_str();
    str_index();
    str_index2();
    operate_utf8_str();
}

/**
 * 对字符串使用切片语法时，需要注意，切片的索引必须落在
 * 字符之间的边界位置。
 * 字符串字面量的类型是切片
 */
fn boundary() {
    let s = "白露";
    // [..2]会报错，因为一个中文字符占3个字节
    let _a = &s[..3];
}

/**
 * 字符串 utf-8编码，字符串中的字符所占的字节数1-4个字节
 * 通常指：String类型和str字符串切片类型
 *  rust中的字符通常为unicode类型，占4个字节
 */
fn demo_str() {
    // String 与 &str 的转换
    let hello = "hello";
    let _he = hello.to_string();
    let _he_str = _he.as_str();
    let _hello_str = &_he[..];
}

// 字符串操作 push insert replace
fn str_index() {
    let mut welcome = String::from("hello ");
    // push 追加字符(char)
    welcome.push('w');
    println!("追加字符 push -> {}", welcome);
    // push_str 追加字符串
    welcome.push_str("orld! zhio!");
    println!("追加字符串 push_str -> {}", welcome);
    // 插入 insert
    welcome.insert(5, ',');
    println!("插入字符 insert -> {}", welcome);
    welcome.insert_str(6, " I like");
    println!("插入字符串 insert_str -> {}", welcome);
    // 替换 replace 返回新的字符串，不操作原有的字符串
    let new_welcome = welcome.replace("h", "H");
    println!("replace -> h: {}", new_welcome);
    let new_welcome_replacen = welcome.replacen("h", "H", 1);
    println!("replacen -> h: {}", new_welcome_replacen);
    // 仅适用 String 类型
    welcome.replace_range(7..8, "R");
    println!("replace_reanger: {}", welcome);
}

//
/**
 * 字符串操作 delete concatenate
 * delete 仅适用于 String 类型，共有4个方法 pop remove truncate clear
 * pop 删除并返回最后一个字符，(返回类型为option)直接操作原有的字符串，当字符串为空时，返回None
 * truncate 删除字符串中从指定位置开始到结束的全部字符,按照字节来处理，无返回值
 * clear 清空字符串，相当于truncate参数为0
 *
 * concatenate 使用 + 和 +=  连接字符串，当调用 + 时，相当于调用 std::String的add方法，
 * add 方法的第二个参数是引用类型，+ 后面得跟一个引用类型。
 * 连接字符串返回新的字符串,format!() 也可以连接字符串
 *
 */
fn str_index2() {
    let mut string_pop = String::from("rust string pop");
    let pop_str = string_pop.pop();
    println!("{:?}", pop_str);

    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove占用 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 下面代码报错，因为remove是按照字节删除的，1不是合法的字符边界
    // let remove = string_remove.remove(1);
    let remove = string_remove.remove(3);
    println!("remove 返回的结果：{}", remove);
    println!("原有字符串： {}", string_remove);

    let mut string_truncate = String::from("rust_string_truncate");
    string_truncate.truncate(5);
    println!("truncate 缩短字符串： {}", string_truncate);

    let mut string_clear = String::from("测试clear方法");
    string_clear.clear();
    println!("clear 清空字符串： {}", string_clear);

    let string_add = String::from("rust");
    let string_append = String::from(" excellent");
    let result = string_add + &string_append;
    let mut result = result + "!";
    result += "!!";
    println!("字符串连接：{}", result);
}

fn operate_utf8_str() {
    // 字符
    for c in "白露".chars() {
        println!("{}", c);
    }
    // 字节
    for b in "白露".bytes() {
        println!("{}", b);
    }
}
