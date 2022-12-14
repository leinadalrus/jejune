pub mod lexicon {
    use std::io::{self, BufRead};

    use lindera::LinderaResult;
    use lindera::error::LinderaError;
    use lindera::{
        mode::Mode,
        tokenizer::{DictionaryConfig, Tokenizer, TokenizerConfig, UserDictionaryConfig},
        DictionaryKind,
    };
    use regex::Regex;

    pub enum Crits {
        ILLEGAL,
        END_OF_FILE,
        // Identifiers
        INDENTIFIER,
        INTEGER,
        // Operators
        HAT,
        PLUS,
        HYPHEN,
        DOLLAR,
        MODULUS,
        ASTERISK,
        AMPERSAND,
        UNDERSCORE,
        ASSIGNMENT,
        VERTICAL_BAR,
        FORWARD_SLASH,
        BACKWARD_SLASH,
        EXCLAMATION_MARK,

        FULL_POINT,
        LESSER_THAN,
        GREATER_THAN,
        QUESTION_MARK,

        // Delimiters
        COMMA,
        SEMICOLON,

        GRAVE,
        TILDE,
        ACUTE,
        QUOTATION,

        LEFT_PARENTHESES,
        LEFT_BRACE,
        RIGHT_PARENTHESES,
        RIGHT_BRACE,
        // Keywords
        FUNCTION,
        LET,
    }

    struct Tokens<'a, T> {
      tokens: Result<Vec<lindera_core::token::Token<'a>>, T>,
    }

    struct Symbol {
        token_type: Crits,
        literal_type: String,
    }

    pub struct Lexer {
        tokenizer: lindera::tokenizer::Tokenizer,
        sym: Symbol,
        cursor: u8,
    }

    pub trait Gloss {
        type Annals;

        fn into_next_token(input_characters: String) -> LinderaResult<()>;
        fn read_token_character() -> lindera::LinderaResult<()>;
        fn read_identifier();
        fn is_letter(is_letter_value: bool) -> bool;
        fn skip_whitespace();
        fn check_for_stop_words();
        fn lookup_identifier();
        fn destroy_tokens_health();
    }

    impl Gloss for Lexer {
        type Annals = String;
        // TokenLexer(TokenTypes *token_type) token(&token_type)
        // ~TokenLexer(); // unnecessary for constant, persistant values

        fn into_next_token(input_characters: String) -> LinderaResult<()> {
            let tokenizer = Tokenizer::new()?; // would end with a `?` but lead to error
            // TODO
            Tokens{tokens: tokenizer.tokenize(&input_characters)};

            return Ok(()); // I want to return tokens
        }

        fn read_token_character() -> lindera::LinderaResult<()> {
            let input_characters = io::stdin();
            for line in input_characters.lock().lines() {
                // return as error variant value
                return Self::into_next_token(line.unwrap());
            }
            // return as result variant value
            return Ok(());
        }

        fn read_identifier() {}

        fn is_letter(is_letter_value: bool) -> bool {
            return is_letter_value;
        }

        fn skip_whitespace() {}

        fn check_for_stop_words() {}

        fn lookup_identifier() {}

        fn destroy_tokens_health() {}
    }

    impl Default for Lexer {
        fn default() -> Self {
            let dict = lindera::tokenizer::DictionaryConfig {
                kind: Some(lindera::DictionaryKind::UniDic),
                path: None,
            };

            let conf = lindera::tokenizer::TokenizerConfig {
                dictionary: dict,
                user_dictionary: None,
                mode: lindera::mode::Mode::Normal,
            };
            // let tokens = tokenizer.tokenize("????????????????????????????????????")?; // tokenize the text
            return Self {
                tokenizer: lindera::tokenizer::Tokenizer::with_config(conf).unwrap(),
                sym: Symbol { token_type: Crits::END_OF_FILE, literal_type: String::new()},
                cursor: 0,
            };
        }
    }

    impl std::ops::Deref for Lexer {
        type Target = Option<()>;

        fn deref(&self) -> &Self::Target {
        todo!()
    }
    }
}
