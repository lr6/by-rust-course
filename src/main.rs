fn main() {
    greet_hello();
}

fn greet_hello() {
    let southern_germany: &str = "Grüß Gott!";
    let chinese: &str = "世界，你好";
    let english: &str = "world, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for region in regions {
        println!("{}", &region);
    }
}
