use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;
use regex::Regex;

enum Crits {
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

struct Symbol {
    token_type: Crits,
    literal_type: String,
}

struct Lexer {
    tokenizer: Tokenizer,
    cursor: u8,
}

trait Gloss {
    type Annals;

    fn proc() -> LinderaResult<()>;
    fn into_next_token(input_characters: String) -> String;
    fn read_token_character() -> String;
    fn read_identifier();
    fn is_letter(is_letter_value: bool) -> bool;
    fn skip_whitespace();
    fn check_for_stop_words();
    fn lookup_identifier();
    fn destroy_tokens_health();
}

impl Gloss for Lexer {
    type Annals = String;

    fn proc() -> LinderaResult<()> {
        let tokenizer = Tokenizer::new()?;
        let tokens = tokenizer.tokenize(""); // TODO

        Ok(())
    } // TokenLexer(TokenTypes *token_type) token(&token_type)
      // ~TokenLexer(); // unnecessary for constant, persistant values

    fn into_next_token(input_characters: String) -> String {
        return input_characters;
    }

    fn read_token_character() -> String {
        let ret_value: String = "".to_owned();
        return ret_value;
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

fn main() {
  let lexer = Lexer::proc();
}
