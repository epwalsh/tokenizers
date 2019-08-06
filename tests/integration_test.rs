use tokenz::Tokenizer;

#[test]
fn test_english_tokenize() {
    let tokenizer = Tokenizer::english();
    let s = String::from("Héllo,, World!-huh?\r\n😃 my website is https://epwalsh.com");
    let tokens = tokenizer.tokenize(&s);
    assert_eq!(
        tokens,
        vec![
            "Héllo",
            ",",
            ",",
            "World",
            "!",
            "-",
            "huh",
            "?",
            "😃",
            "my",
            "website",
            "is",
            "https://epwalsh.com",
        ]
    );
}
