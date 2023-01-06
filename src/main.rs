// 数值类型: 有符号整数 (i8, i16, i32, i64, isize), 无符号整数 (u8, u16, u32, u64, usize), 浮点数 (f32， f64)， 以及有理数，复数
// 字符串： 字符串字面量和字符串切片 &str
// 布尔类型： true和false
// 字符类型： 表示单个 Unicide字符, 存储为4个字节
// 单元类型: 即 () ，其唯一的值也是 ()


use num::complex::Complex;
fn main() {
    let x = 13.14_f32.round();
    println!("x={}", x);
    float_pitfall();
    nan();
    calculate();
    bit();
    range();
    yls();
}
// 浮点数陷阱
fn float_pitfall() {
    println!("----浮点数陷阱----");
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("0.3 = {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("0.3 = {:x}", (xyz.2).to_bits());

    // 相等
    assert!(abc.0 + abc.1 == abc.2);
    // 不相等
    assert!(xyz.0 + xyz.1 != xyz.2);
    println!();
}

// NaN
fn nan() {
    println!("----NaN----");
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    assert!(x != x);
    println!();
}

// 数字运算
fn calculate() {
    println!("----数字运算----");
    // 加法
    let sum = 5 + 10;
    // 减法
    let difference = 95.5 - 4.3;
    // 乘法
    let product = 4 * 30;
    // 除法
    let quotient = 56.7 / 32.2;
    // 求余
    let remainder = 43 % 5;
    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    // 编译器会进行自动推导，给予twenty i32的类型
    let twenty = 20;
    // 类型标注
    let twenty_one: i32 = 21;
    // 通过类型后缀的方式进行类型标注：22是i32类型
    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    // 定义一个f32数组，其中42.0会自动被推导为f32类型
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 打印数组中第一个值，并控制小数位为2位
    println!("{:.2}", forty_twos[0]);
    println!();
}

// 位运算
fn bit() {
    println!("----位运算----");
    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
    println!();
}

// 序列
fn range() {
    println!("----位运算----"); 
    for i in 1..=5 {
        println!("{}", i);
    }
    println!("--字母--"); 
    for z in 'a'..='z' {
        println!("{}", z);
    }
    println!(); 
}

// 有理数
fn yls() {
    println!("----有理数----");  
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
    println!(); 
}