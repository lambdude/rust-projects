extern crate itertools;

use itertools::Itertools;

fn main() {
    // foreach
    let mut words = "hello supercalifragilisticexpialidocious programmer".split(|c| c == ' ');
    words.foreach(|word| println!("{} is {} characters long", word, word.len()));

    // interleave and intersperse
    let even = (1..10).map(|x| x * 2);
    let odd = (1..5).map(|x| x * 2 + 1);
    println!("{:?}", even.interleave(odd).collect::<Vec<_>>());
    println!("{:?}", (1..10).intersperse(15).collect::<Vec<_>>());
}
