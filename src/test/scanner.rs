#[cfg(test)]
mod tests {
    const BASE_PATH: &'static str = "test/scanning/";

    use crate::scanner::Scanner;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn identifiers() {
        let path = format!("{BASE_PATH}identifiers.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![
                "Identifier andy Nil".to_string(),
                "Identifier formless Nil".to_string(),
                "Identifier fo Nil".to_string(),
                "Identifier _ Nil".to_string(),
                "Identifier _123 Nil".to_string(),
                "Identifier _abc Nil".to_string(),
                "Identifier ab123 Nil".to_string(),
                "Identifier abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_ Nil"
                    .to_string(),
                "Eof  Nil".to_string(),
            ]
        );
    }

    #[test]
    fn keywords() {
        let path = format!("{BASE_PATH}keywords.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![
                "And and Nil",
                "Class class Nil",
                "Else else Nil",
                "False false Nil",
                "For for Nil",
                "Fun fun Nil",
                "If if Nil",
                "Nil nil Nil",
                "Or or Nil",
                "Return return Nil",
                "Super super Nil",
                "This this Nil",
                "True true Nil",
                "Var var Nil",
                "While while Nil",
                "Eof  Nil",
            ]
        );
    }

    #[test]
    fn numbers() {
        let path = format!("{BASE_PATH}numbers.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![
                "Number 123 123",
                "Number 123.456 123.456",
                "Dot . Nil",
                "Number 456 456",
                "Number 123 123",
                "Dot . Nil",
                "Eof  Nil",
            ]
        );
    }

    #[test]
    fn punctuators() {
        let path = format!("{BASE_PATH}punctuators.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![
                "LeftParen ( Nil",
                "RightParen ) Nil",
                "LeftBrace { Nil",
                "RightBrace } Nil",
                "Semicolon ; Nil",
                "Comma , Nil",
                "Plus + Nil",
                "Minus - Nil",
                "Star * Nil",
                "BangEq != Nil",
                "EqEq == Nil",
                "LessEq <= Nil",
                "GreaterEq >= Nil",
                "BangEq != Nil",
                "Less < Nil",
                "Greater > Nil",
                "Slash / Nil",
                "Dot . Nil",
                "Eof  Nil",
            ]
        );
    }

    #[test]
    fn string() {
        let path = format!("{BASE_PATH}string.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![r#"String "" "#, r#"String "string" string"#, r#"Eof  Nil"#,]
        );
    }

    #[test]
    fn whitespace() {
        let path = format!("{BASE_PATH}whitespace.lox");

        let tokens = get_tokens_string(&path);
        assert_eq!(
            tokens,
            vec![
                "Identifier space Nil",
                "Identifier tabs Nil",
                "Identifier newlines Nil",
                "Identifier end Nil",
                "Eof  Nil"
            ]
        );
    }

    fn get_tokens_string(path: &str) -> Vec<String> {
        let mut file = File::options().read(true).open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let scanner = Scanner::new(&buf);
        let tokens = scanner.scan_tokens().unwrap();

        tokens.iter().map(|t| t.to_string()).collect()
    }
}
