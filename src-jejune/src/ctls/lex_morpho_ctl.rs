pub mod controllers {
    use std::{result, io::{self, BufRead}};

        use crate::models::lex_morpho_ast::lexicon::{Crits, Gloss, Lexer};

        pub enum Actions {GET, POST, PUT, DELETE}

        pub struct Controller {
            controller: String,
            action: String,
            id: String
        }

        impl Controller {
            pub fn new() -> Controller {
                return Controller {
                    controller: String::new(),
                    action: String::new(),
                    id: String::new()
                };
            }

            pub fn read_from_cell(_input_characters: String) -> io::Result<()> {
                let input_characters = io::stdin();
                for line in input_characters.lock().lines() {
                    println!("{:#?}", line);
                }
                // return as result variant value
                return Ok(());
            }

            pub fn index_into_table(string_slice: &'static str) -> &'static str {
                return string_slice;
            }
        }

        impl Default for Controller {
            fn default() -> Self {
                Self::new()
            }
        }
    pub mod lexicon {
        use super::Controller;

        pub struct Spreadsheeter {
            cursor_pos_x: u8,
            cursor_pos_y: u8 
        }

        impl Spreadsheeter {/* TODO */}
    }
}
