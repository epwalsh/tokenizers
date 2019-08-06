use tokenz::Tokenizer;

fn main() {
    let token_pattern = r"^(https?://.*)$";
    // let infix_pattern = r"[?,!]|(?<=[0-9])-(?=[0-9-])";  // look-around not supported.
    let infix_pattern = r"[?,!-]";
    let tokenizer = Tokenizer::new(token_pattern, infix_pattern);
    println!("{:#?}", tokenizer);
    let s = String::from("Héllo,, World!-huh?\r\n😃 my website is https://epwalsh.com");
    let tokens = tokenizer.tokenize(&s);
    println!("\nFinal tokens:");
    for token in &tokens {
        println!("{}", token);
    }
}
