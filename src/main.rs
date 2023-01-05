struct Struct {
    e: i32
}
fn main() {
    // 使用下划线开头忽略未使用的变量
    let _y = 10;
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);
    destructuring_assignment();
    fn2();
    shadowing();
}

fn destructuring_assignment() {
    let (a, mut b) : (bool, bool) = (true, false);
    // a = true，不可变 b = false, 可变
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
}

fn fn2() {
    // 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
    // Rust 常量的命名约定是全部字母都使用大写，并使用下划线分隔单词，另外对数字字面量可插入下划线以提高可读性
    const MAX_POINTS: u32 = 100_000;
    println!("常量为：{}", MAX_POINTS);
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// 变量遮蔽(shadowing)
// Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示：
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("花括号里面的x={}", x);
    }
    println!("花括号外面的x={}", x);

    let spaces = "123";
    let spaces = spaces.len();
    println!("spaces的长度：{}", spaces);
}
