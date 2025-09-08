/// Characters that are considered punctuation for tokenization
const PUNCTUATION_CHARS: [char; 7] = [',', '.', '!', '?', ';', ':', '-'];

/// Represents a token with its text content and byte positions in the original input
#[derive(Debug)]
pub struct Token {
    /// The text content of the token
    pub text: String,
    /// Starting byte position in the original input
    pub start: usize,
    /// Ending byte position in the original input (exclusive)
    pub end: usize,
}

impl Token {
    fn new(text: String, start: usize, end: usize) -> Self {
        Self { text, start, end }
    }
}

/// Tokenizes input text by splitting on whitespace while handling punctuation intelligently
///
/// This tokenizer:
/// - Splits on whitespace boundaries
/// - Treats most punctuation as separate tokens
/// - Keeps punctuation within tokens for abbreviations, decimals, times, and hyphens
/// - Groups repeated punctuation (like "..." or "!!!") as single tokens
///
/// # Arguments
/// * `input` - The text to tokenize
///
/// # Returns
/// A vector of tokens with their text content and byte positions
pub fn whitespace_tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token_text = String::new();
    let mut current_token_start: Option<usize> = None;
    let mut char_iter = input.char_indices().peekable();

    while let Some((char_pos, current_char)) = char_iter.next() {
        if current_char.is_whitespace() {
            flush_current_token(
                &mut tokens,
                &mut current_token_text,
                &mut current_token_start,
                char_pos,
            );
            continue;
        }

        if PUNCTUATION_CHARS.contains(&current_char) {
            let next_char_opt = char_iter.peek().map(|(_, c)| *c);

            if is_token_inner_punctuation(current_char, next_char_opt, &current_token_text) {
                initialize_token_start(&current_token_text, &mut current_token_start, char_pos);
                current_token_text.push(current_char);
                continue;
            }

            if current_char == '.' || current_char == '!' {
                flush_current_token(
                    &mut tokens,
                    &mut current_token_text,
                    &mut current_token_start,
                    char_pos,
                );
                tokens.push(process_repeated_punctuation(
                    &mut char_iter,
                    current_char,
                    char_pos,
                ));
                continue;
            }

            flush_current_token(
                &mut tokens,
                &mut current_token_text,
                &mut current_token_start,
                char_pos,
            );
            tokens.push(Token::new(
                current_char.to_string(),
                char_pos,
                char_pos + current_char.len_utf8(),
            ));
        } else {
            initialize_token_start(&current_token_text, &mut current_token_start, char_pos);
            current_token_text.push(current_char);
        }
    }

    flush_current_token(
        &mut tokens,
        &mut current_token_text,
        &mut current_token_start,
        input.len(),
    );

    tokens
}

/// Flushes the current token buffer to the tokens vector if it's not empty
fn flush_current_token(
    tokens: &mut Vec<Token>,
    buffer: &mut String,
    token_start: &mut Option<usize>,
    end_pos: usize,
) {
    if !buffer.is_empty() {
        tokens.push(Token::new(
            std::mem::take(buffer),
            token_start.take().unwrap(),
            end_pos,
        ));
    }
}

/// Initializes the start position for a new token if the buffer is empty
fn initialize_token_start(buffer: &str, token_start: &mut Option<usize>, current_pos: usize) {
    if buffer.is_empty() {
        *token_start = Some(current_pos);
    }
}

/// Determines if a punctuation character should be kept within the current token
/// rather than being treated as a separate token
fn is_token_inner_punctuation(
    punctuation: char,
    next_char: Option<char>,
    current_token: &str,
) -> bool {
    let prev_char = current_token.chars().last();

    // Handle hyphens and dots in compound words/abbreviations
    let is_compound_element = match punctuation {
        '-' => {
            // Only treat hyphen as part of compound word if:
            // 1. There's a preceding alphanumeric character in the current token
            // 2. There's a following alphanumeric character (not just any non-whitespace)
            prev_char.is_some_and(|c| c.is_alphanumeric())
                && next_char.is_some_and(|c| c.is_alphanumeric())
        }
        '.' => {
            prev_char.is_some_and(|c| c.is_alphanumeric())
                && next_char.is_some_and(|c| c.is_alphanumeric())
        }
        _ => false,
    };

    // Handle colons in times and commas in decimals
    let is_numeric_separator = match punctuation {
        ':' | ',' => next_char.is_some_and(|c| c.is_ascii_digit()),
        _ => false,
    };

    is_compound_element || is_numeric_separator
}

/// Handles repeated punctuation characters (like "..." or "!!!")
fn process_repeated_punctuation(
    char_iter: &mut std::iter::Peekable<std::str::CharIndices>,
    punctuation_char: char,
    start_pos: usize,
) -> Token {
    let mut repeated_text = String::new();
    repeated_text.push(punctuation_char);
    let mut end_pos = start_pos + punctuation_char.len_utf8();

    // Collect consecutive identical punctuation characters
    while let Some(&(next_pos, next_char)) = char_iter.peek() {
        if next_char == punctuation_char {
            repeated_text.push(next_char);
            end_pos = next_pos + next_char.len_utf8();
            char_iter.next();
        } else {
            break;
        }
    }

    Token::new(repeated_text, start_pos, end_pos)
}
