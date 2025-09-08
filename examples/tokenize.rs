use whitespace_tokenizer::whitespace_tokenize;

fn main() {
    let text = "It's state-of-the-art! The price is $3.14... Really?! Time: 12:30.";
    let tokens = whitespace_tokenize(text);

    println!("Input: {}", text);
    println!("\nTokens:");
    for (i, token) in tokens.iter().enumerate() {
        println!(
            "{:2}: '{}' [{}..{}]",
            i + 1,
            token.text,
            token.start,
            token.end
        );
    }

    println!("\nPunctuation handling:");
    println!("- Contractions like \"It's\" stay together");
    println!("- Hyphenated words like \"state-of-the-art\" stay together");
    println!("- Decimal numbers like \"3.14\" stay together");
    println!("- Time formats like \"12:30\" stay together");
    println!("- Repeated punctuation like \"...\" and \"?!\" are handled");
}
