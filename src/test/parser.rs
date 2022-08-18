// #[cfg(test)]
// mod basic_tests {
//     use std::{fs::File, io::Read};

//     use crate::{ast::{Ast, ast_printer::AstPrint}, parser::{ParseError, Parser}, scanner::Scanner};

//     const BASE_PATH: &'static str = "test/parsing/basic_expressions/";

//     fn get_ast(path: String) -> Result<Ast, ParseError> {
//         let mut file = File::options().read(true).open(path).unwrap();
//         let mut source = String::new();
//         file.read_to_string(&mut source).unwrap();

//         let scanner = Scanner::new(&source);
//         let tokens = scanner.scan_tokens().unwrap();

//         let mut parser = Parser::new(tokens);

//         parser.parse()
//     }

//     fn print_ast(ast: &Ast) -> String {
//         let mut printer = AstPrint;
//         printer.print(ast)
//     }

//     #[test]
//     fn term() {
//         let path = format!("{BASE_PATH}term.lox");

//         let repr = print_ast(&get_ast(path).unwrap());
//         assert_eq!(repr, "(+ (- (+ (- (+ 1.0 2.0) 3.0) 4.0) 5.0) 6.0)");
//     }

//     #[test]
//     fn factor() {
//         let path = format!("{BASE_PATH}factor.lox");

//         let repr = print_ast(&get_ast(path).unwrap());
//         assert_eq!(repr, "(+ (- (+ 1.0 (* 2.0 3.0)) (/ 4.0 5.0)) 6.0)");
//     }
// }