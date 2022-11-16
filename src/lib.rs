use std::{path::{PathBuf, Path}, fs};

pub fn parse(text: &str) -> Vec<Token> {
    // cases:
    // 0. whitespace -> consume
    // 1. newline -> consume and increment line count
    // 2. comment'/'
    //     2a. line comment: '//' -> consume until newline
    //     2b. block comment: '/*' -> consume until '*/'
    // 3. symbol -> make symbol token
    // 4. digit (\d*)-> consume all \d
    // 5. identifier ([^\d\W]{1}\w*)-> consume all \w

    let mut tokens = Vec::new();
    let mut char_iterator = text.chars().peekable(); // needs to be mutable
    let mut line_count = 1;

    while let Some(c) = char_iterator.next() {
        match c {
            // newline
            '\n' => line_count += 1,
            // whitespace
            c if c.is_whitespace() => continue,
            // comments or slash
            '/' => {
                if char_iterator.peek() == Some(&'/') {
                    char_iterator.next();
                    // line comment
                    loop {
                        match char_iterator.peek() {
                            Some(ch) => {
                                if ch == &'\n' {
                                    break
                                } else {
                                    char_iterator.next();
                                }
                            }
                            None => break
                        }
                    }

                } else if char_iterator.peek() == Some(&'*') {
                    char_iterator.next();
                    // block comment
                    loop {
                        match char_iterator.peek() {
                            Some(ch) => {
                                if ch == &'\n' {
                                    // increment line counts at newlines
                                    line_count += 1;
                                    char_iterator.next();
                                } else if ch == &'*' {
                                    // might either be end, or just a character
                                    char_iterator.next();
                                    match char_iterator.peek() {
                                        Some(ch) => {
                                            if ch == &'/' {
                                                // end of block comment
                                                char_iterator.next();
                                                break;
                                            } else {
                                                // continue consuming chars
                                                char_iterator.next();
                                            }
                                        }
                                        None => break
                                    }
                                } else {
                                    // continue consuming chars
                                    char_iterator.next();
                                }
                            }
                            None => break
                        }
                    }

                } else {
                    // '/' symbol
                    tokens.push(Token::new(TokenType::Slash, line_count));
                }
            }
            // symbols (except '/', which is handled in comment segment)
            '{' => tokens.push(Token::new(TokenType::LeftBrace, line_count)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, line_count)),
            '(' => tokens.push(Token::new(TokenType::LeftParen, line_count)),
            ')' => tokens.push(Token::new(TokenType::RightParen, line_count)),
            '[' => tokens.push(Token::new(TokenType::LeftBracket, line_count)),
            ']' => tokens.push(Token::new(TokenType::RightBracket, line_count)),
            '.' => tokens.push(Token::new(TokenType::Dot, line_count)),
            ',' => tokens.push(Token::new(TokenType::Comma, line_count)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, line_count)),
            '+' => tokens.push(Token::new(TokenType::Plus, line_count)),
            '-' => tokens.push(Token::new(TokenType::Minus, line_count)),
            '*' => tokens.push(Token::new(TokenType::Star, line_count)),
            '&' => tokens.push(Token::new(TokenType::Ampersand, line_count)),
            '|' => tokens.push(Token::new(TokenType::Bar, line_count)),
            '<' => tokens.push(Token::new(TokenType::LessThan, line_count)),
            '>' => tokens.push(Token::new(TokenType::GreaterThan, line_count)),
            '=' => tokens.push(Token::new(TokenType::Equals, line_count)),
            '~' => tokens.push(Token::new(TokenType::Tilde, line_count)),
            // indentifier
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();

                identifier.push(c);

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if ch.is_ascii_alphanumeric() || *ch == '_' {
                                identifier.push(char_iterator.next().unwrap());
                            } else {
                                break
                            }
                        },
                        None => break,
                    }
                }
                match identifier.as_str() {
                    "class" => tokens.push(Token::new(TokenType::Class, line_count)),
                    "constructor" => tokens.push(Token::new(TokenType::Constructor, line_count)),
                    "function" => tokens.push(Token::new(TokenType::Function, line_count)),
                    "method" => tokens.push(Token::new(TokenType::Method, line_count)),
                    "field" => tokens.push(Token::new(TokenType::Field, line_count)),
                    "static" => tokens.push(Token::new(TokenType::Static, line_count)),
                    "var" => tokens.push(Token::new(TokenType::Var, line_count)),
                    "int" => tokens.push(Token::new(TokenType::Int, line_count)),
                    "char" => tokens.push(Token::new(TokenType::Char, line_count)),
                    "boolean" => tokens.push(Token::new(TokenType::Boolean, line_count)),
                    "void" => tokens.push(Token::new(TokenType::Void, line_count)),
                    "true" => tokens.push(Token::new(TokenType::True, line_count)),
                    "false" => tokens.push(Token::new(TokenType::False, line_count)),
                    "null" => tokens.push(Token::new(TokenType::Null, line_count)),
                    "this" => tokens.push(Token::new(TokenType::This, line_count)),
                    "let" => tokens.push(Token::new(TokenType::Let, line_count)),
                    "do" => tokens.push(Token::new(TokenType::Do, line_count)),
                    "if" => tokens.push(Token::new(TokenType::If, line_count)),
                    "else" => tokens.push(Token::new(TokenType::Else, line_count)),
                    "while" => tokens.push(Token::new(TokenType::While, line_count)),
                    "return" => tokens.push(Token::new(TokenType::Return, line_count)),
                    ident => tokens.push(Token::new(TokenType::Identifier(ident.to_string()), line_count)),
                }
            },
            // integer literal
            '0'..='9' => {
                let mut int_literal = String::new();

                int_literal.push(c);

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if ch.is_ascii_digit() {
                                int_literal.push(char_iterator.next().unwrap());
                            } else {
                                break
                            }
                        },
                        None => break,
                    }
                }
                tokens.push(Token::new(TokenType::IntLiteral(int_literal), line_count))
            }
            // string literal
            '"' => {
                let mut string_literal = String::new();
                // consume start quote
                char_iterator.next();
                string_literal.push(c);

                loop {
                    match char_iterator.peek() {
                        Some(ch) => {
                            if *ch != '"' {
                                string_literal.push(char_iterator.next().unwrap());
                            } else {
                                // consume end quote
                                char_iterator.next();
                                break
                            }
                        },
                        None => break,
                    }
                }
                tokens.push(Token::new(TokenType::IntLiteral(string_literal), line_count))
            }
            // catchall for invalid characters
            _ => println!("invalid character while parsing")
        }
        
    }
    dbg!(&tokens);
    tokens
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    // Symbols
    LeftBrace, RightBrace, LeftParen, RightParen, LeftBracket, RightBracket,
    Dot, Comma, Semicolon, Plus, Minus, Star, Slash, Ampersand, Bar, 
    LessThan, GreaterThan, Equals, Tilde,

    // Keywords
    Class, Constructor, Function, Method, Field, Static, Var, Int, Char, Boolean,
    Void, True, False, Null, This, Let, Do, If, Else, While, Return,

    // Literals
    Identifier(String), StringLiteral(String), IntLiteral(String),
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
    token_type: TokenType,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line: u32) -> Token {
        Token {token_type, line}
    }
}

#[derive(Debug)]
pub struct Config {
    pub file_paths: Vec<PathBuf>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("incorrect number of arguments");
        };
        
        let path = Path::new(&args[1]);
        let mut file_paths: Vec<PathBuf> = Vec::new(); 

        let metadata = match fs::metadata(path) {
            Ok(md) => md,
            // What if I want to return the actual error?
            Err(_) => {
                return Err("problem accesing filepath metadata")
            },
        };

        // Check if the path is a file or a directory
        if metadata.is_file() {

            let ext = match path.extension() {
                Some(ext) => ext,
                None => return Err("unable to access file extension")
            };

            // If file, check that it is a .jack file and add to config
            if ext == "jack" {
                file_paths.push(path.to_path_buf());
            } else {
                return Err("filename had incorrect extension")
            }

        } else {
            // If directory, add all .jack files to config
            let paths = match fs::read_dir(path) {
                Ok(paths) => paths,
                Err(_) => return Err("unable to access directory")
            };

            // Using unwrap here instead of handling the errors
            file_paths = paths
                .map(|path| path.unwrap().path())
                .filter(|path| { path.is_file() && path.extension().unwrap() == "jack" })
                .collect();
        }

        Ok(Config{ file_paths })
        
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn parse_symbols() {
        let text = String::from("{} () [] . , ; + - * & | < > = ~ /");

        let result = parse(&text);

        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::LeftBrace, 1));
        expected.push(Token::new(TokenType::RightBrace, 1));
        expected.push(Token::new(TokenType::LeftParen, 1));
        expected.push(Token::new(TokenType::RightParen, 1));
        expected.push(Token::new(TokenType::LeftBracket, 1));
        expected.push(Token::new(TokenType::RightBracket, 1));
        expected.push(Token::new(TokenType::Dot, 1));
        expected.push(Token::new(TokenType::Comma, 1));
        expected.push(Token::new(TokenType::Semicolon, 1));
        expected.push(Token::new(TokenType::Plus, 1));
        expected.push(Token::new(TokenType::Minus, 1));
        expected.push(Token::new(TokenType::Star, 1));
        expected.push(Token::new(TokenType::Ampersand, 1));
        expected.push(Token::new(TokenType::Bar, 1));
        expected.push(Token::new(TokenType::LessThan, 1));
        expected.push(Token::new(TokenType::GreaterThan, 1));
        expected.push(Token::new(TokenType::Equals, 1));
        expected.push(Token::new(TokenType::Tilde, 1));
        expected.push(Token::new(TokenType::Slash, 1));

        assert!(result == expected);

    }

    #[test]
    fn parse_identifiers_and_keywords() {
        let text = String::from("class constructor function method field
                                         static var int char boolean void true
                                         false null this let do if else while return
                                         abcdefghijklmnopqrstuvwxyz_ABCDEPGHIJKLMOPQRSTUVWXYZ0123456789");

        let result = parse(&text);

        let mut expected: Vec<Token> = Vec::new();
        expected.push(Token::new(TokenType::Class, 1));
        expected.push(Token::new(TokenType::Constructor, 1));
        expected.push(Token::new(TokenType::Function, 1));
        expected.push(Token::new(TokenType::Method, 1));
        expected.push(Token::new(TokenType::Field, 1));
        expected.push(Token::new(TokenType::Static, 2));
        expected.push(Token::new(TokenType::Var, 2));
        expected.push(Token::new(TokenType::Int, 2));
        expected.push(Token::new(TokenType::Char, 2));
        expected.push(Token::new(TokenType::Boolean, 2));
        expected.push(Token::new(TokenType::Void, 2));
        expected.push(Token::new(TokenType::True, 2));
        expected.push(Token::new(TokenType::False, 3));
        expected.push(Token::new(TokenType::Null, 3));
        expected.push(Token::new(TokenType::This, 3));
        expected.push(Token::new(TokenType::Let, 3));
        expected.push(Token::new(TokenType::Do, 3));
        expected.push(Token::new(TokenType::If, 3));
        expected.push(Token::new(TokenType::Else, 3));
        expected.push(Token::new(TokenType::While, 3));
        expected.push(Token::new(TokenType::Return, 3));
        expected.push(Token::new(TokenType::Identifier("abcdefghijklmnopqrstuvwxyz_ABCDEPGHIJKLMOPQRSTUVWXYZ0123456789".to_string()), 4));

        assert!(result == expected);

    }

    #[test]
    fn read_file_jack() {
        // requires directory 'test_dir' to contain file 'test1.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir/test1.jack"));

        let result_config = Config::build(&args).unwrap();

        let filepath = PathBuf::from_str(&args[1]).unwrap();

        assert!(result_config.file_paths.contains(&filepath));
    }

    #[test]
    fn read_directory_with_jack_files() {
        // requires directory 'test_dir' to contain files 'test1.jack' & 'test2.jack'
        let mut args = Vec::new();
        args.push(String::from("arg1"));
        args.push(String::from("./test_dir"));

        let result_config = Config::build(&args).unwrap();

        // both these .jack file paths should be saved in the config
        let filepath1 = PathBuf::from_str("./test_dir/test1.jack").unwrap();
        let filepath2 = PathBuf::from_str("./test_dir/test2.jack").unwrap();
        
        // this .txt file path should not be saved
        let badfilepath = PathBuf::from_str("./test_dir/test3.txt").unwrap();

        assert!(result_config.file_paths.contains(&filepath1));
        assert!(result_config.file_paths.contains(&filepath2));
        assert!(!result_config.file_paths.contains(&badfilepath));
    }
}
