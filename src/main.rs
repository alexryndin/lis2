use lis2;
fn main() {
    let v = [1,2,3];
    let input = String::from("(+ 2 2)");

    let mut t = lis2::Tokenizer2::new(&input);

    while let Some(v) = t.next() {
        print!("{:?}", v);
    }

    println!("Hello, world!");
}

fn test(v: u8) {
    ()
}
