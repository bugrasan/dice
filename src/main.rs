use rand::Rng;

fn main() {
    // statically include wordlist into program
    static CONTENT: &'static str = include_str!("eff_large.wordlist");

    // get a random line
    let len: usize = CONTENT.lines().count();
    let dice = rand::thread_rng().gen_range(0..len);

    // finally print the word
    println!("{}", CONTENT.lines().nth(dice).unwrap_or(""));
}
