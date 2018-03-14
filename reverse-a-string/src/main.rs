fn main() {
    let first_string = "Hello, world!";
    let first_rev_string: String = first_string.chars().rev().collect();
    println!("{}", first_rev_string);

    let second_string = "一二三四五六七八九十";
    let second_rev_string: String = second_string.chars().rev().collect();
    println!("{}", second_rev_string);
}
