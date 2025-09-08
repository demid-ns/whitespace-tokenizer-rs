use whitespace_tokenizer::{whitespace_tokenize, Token};

struct TestCase<'a> {
    description: &'a str,
    input: &'a str,
    expected: Vec<Token>,
}

#[test]
fn test_whitespace_tokenize() {
    let cases = [
        TestCase {
            description: "simple sentence with commas",
            input: "hello, ,   ÍÍÍ world ",
            expected: vec![
                Token {
                    text: "hello".into(),
                    start: 0,
                    end: 5,
                },
                Token {
                    text: ",".into(),
                    start: 5,
                    end: 6,
                },
                Token {
                    text: ",".into(),
                    start: 7,
                    end: 8,
                },
                Token {
                    text: "ÍÍÍ".into(),
                    start: 11,
                    end: 17,
                },
                Token {
                    text: "world".into(),
                    start: 18,
                    end: 23,
                },
            ],
        },
        TestCase {
            description: "complex punctuation and numbers",
            input: ", hello, ,, ÍÍÍ WORLD,: !4:56, 4.53, 4.35.27.53 3-25 3,25 state-of-art, state,of.art  ",
            expected: vec![
                Token {
                    text: ",".into(),
                    start: 0,
                    end: 1,
                },
                Token {
                    text: "hello".into(),
                    start: 2,
                    end: 7,
                },
                Token {
                    text: ",".into(),
                    start: 7,
                    end: 8,
                },
                Token {
                    text: ",".into(),
                    start: 9,
                    end: 10,
                },
                Token {
                    text: ",".into(),
                    start: 10,
                    end: 11,
                },
                Token {
                    text: "ÍÍÍ".into(),
                    start: 12,
                    end: 18,
                },
                Token {
                    text: "WORLD".into(),
                    start: 19,
                    end: 24,
                },
                Token {
                    text: ",".into(),
                    start: 24,
                    end: 25,
                },
                Token {
                    text: ":".into(),
                    start: 25,
                    end: 26,
                },
                Token {
                    text: "!".into(),
                    start: 27,
                    end: 28,
                },
                Token {
                    text: "4:56".into(),
                    start: 28,
                    end: 32,
                },
                Token {
                    text: ",".into(),
                    start: 32,
                    end: 33,
                },
                Token {
                    text: "4.53".into(),
                    start: 34,
                    end: 38,
                },
                Token {
                    text: ",".into(),
                    start: 38,
                    end: 39,
                },
                Token {
                    text: "4.35.27.53".into(),
                    start: 40,
                    end: 50,
                },
                Token {
                    text: "3-25".into(),
                    start: 51,
                    end: 55,
                },
                Token {
                    text: "3,25".into(),
                    start: 56,
                    end: 60,
                },
                Token {
                    text: "state-of-art".into(),
                    start: 61,
                    end: 73,
                },
                Token {
                    text: ",".into(),
                    start: 73,
                    end: 74,
                },
                Token {
                    text: "state".into(),
                    start: 75,
                    end: 80,
                },
                Token {
                    text: ",".into(),
                    start: 80,
                    end: 81,
                },
                Token {
                    text: "of.art".into(),
                    start: 81,
                    end: 87,
                },
            ],
        },
        TestCase {
            description: "basic sentence",
            input: "hello world",
            expected: vec![
                Token {
                    text: "hello".into(),
                    start: 0,
                    end: 5,
                },
                Token {
                    text: "world".into(),
                    start: 6,
                    end: 11,
                },
            ],
        },
        TestCase {
            description: "empty string",
            input: "",
            expected: vec![],
        },
        TestCase {
            description: "contractions and hyphens",
            input: "It's state-of-the-art, right?",
            expected: vec![
                Token {
                    text: "It's".into(),
                    start: 0,
                    end: 4,
                },
                Token {
                    text: "state-of-the-art".into(),
                    start: 5,
                    end: 21,
                },
                Token {
                    text: ",".into(),
                    start: 21,
                    end: 22,
                },
                Token {
                    text: "right".into(),
                    start: 23,
                    end: 28,
                },
                Token {
                    text: "?".into(),
                    start: 28,
                    end: 29,
                },
            ],
        },
        TestCase {
            description: "numbers and decimals",
            input: "The value is 3.14 and 22.5, not 7.",
            expected: vec![
                Token {
                    text: "The".into(),
                    start: 0,
                    end: 3,
                },
                Token {
                    text: "value".into(),
                    start: 4,
                    end: 9,
                },
                Token {
                    text: "is".into(),
                    start: 10,
                    end: 12,
                },
                Token {
                    text: "3.14".into(),
                    start: 13,
                    end: 17,
                },
                Token {
                    text: "and".into(),
                    start: 18,
                    end: 21,
                },
                Token {
                    text: "22.5".into(),
                    start: 22,
                    end: 26,
                },
                Token {
                    text: ",".into(),
                    start: 26,
                    end: 27,
                },
                Token {
                    text: "not".into(),
                    start: 28,
                    end: 31,
                },
                Token {
                    text: "7".into(),
                    start: 32,
                    end: 33,
                },
                Token {
                    text: ".".into(),
                    start: 33,
                    end: 34,
                },
            ],
        },
        TestCase {
            description: "punctuation at start and end",
            input: "!Wow, amazing!",
            expected: vec![
                Token {
                    text: "!".into(),
                    start: 0,
                    end: 1,
                },
                Token {
                    text: "Wow".into(),
                    start: 1,
                    end: 4,
                },
                Token {
                    text: ",".into(),
                    start: 4,
                    end: 5,
                },
                Token {
                    text: "amazing".into(),
                    start: 6,
                    end: 13,
                },
                Token {
                    text: "!".into(),
                    start: 13,
                    end: 14,
                },
            ],
        },
        TestCase {
            description: "multiple spaces and tabs",
            input: "hello\t  world\nnew line",
            expected: vec![
                Token {
                    text: "hello".into(),
                    start: 0,
                    end: 5,
                },
                Token {
                    text: "world".into(),
                    start: 8,
                    end: 13,
                },
                Token {
                    text: "new".into(),
                    start: 14,
                    end: 17,
                },
                Token {
                    text: "line".into(),
                    start: 18,
                    end: 22,
                },
            ],
        },
        TestCase {
            description: "repeated punctuation",
            input: "Wait... what?! Really!! !!!!",
            expected: vec![
                Token {
                    text: "Wait".into(),
                    start: 0,
                    end: 4,
                },
                Token {
                    text: "...".into(),
                    start: 4,
                    end: 7,
                },
                Token {
                    text: "what".into(),
                    start: 8,
                    end: 12,
                },
                Token {
                    text: "?".into(),
                    start: 12,
                    end: 13,
                },
                Token {
                    text: "!".into(),
                    start: 13,
                    end: 14,
                },
                Token {
                    text: "Really".into(),
                    start: 15,
                    end: 21,
                },
                Token {
                    text: "!!".into(),
                    start: 21,
                    end: 23,
                },
                Token {
                    text: "!!!!".into(),
                    start: 24,
                    end: 28,
                },
            ],
        },
        TestCase {
            description: "standalone hyphens vs compound words",
            input: "well-known - not good - state-of-the-art",
            expected: vec![
                Token {
                    text: "well-known".into(),
                    start: 0,
                    end: 10,
                },
                Token {
                    text: "-".into(),
                    start: 11,
                    end: 12,
                },
                Token {
                    text: "not".into(),
                    start: 13,
                    end: 16,
                },
                Token {
                    text: "good".into(),
                    start: 17,
                    end: 21,
                },
                Token {
                    text: "-".into(),
                    start: 22,
                    end: 23,
                },
                Token {
                    text: "state-of-the-art".into(),
                    start: 24,
                    end: 40,
                },
            ],
        },
    ];

    for case in cases.iter() {
        let result = whitespace_tokenize(case.input);
        assert_eq!(
            result.len(),
            case.expected.len(),
            "Length mismatch: {}",
            case.description
        );

        for (token, expected) in result.iter().zip(case.expected.iter()) {
            assert_eq!(
                token.text, expected.text,
                "Text mismatch: {}",
                case.description
            );
            assert_eq!(
                token.start, expected.start,
                "Start mismatch: {}",
                case.description
            );
            assert_eq!(
                token.end, expected.end,
                "End mismatch: {}",
                case.description
            );
        }
    }
}