use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
use token::Tok;

pub mod ast;
pub mod lexer;
pub mod location;
pub mod error;
pub mod token;
pub mod types;
lalrpop_mod!(ninescript);

#[test]
fn calculator1() {
    let src = r#"
test = 123
if x == 3
    b = 8

"#.trim_start();
    let tokens = Lexer::new(src, 4).collect::<Vec<_>>();
    let tokens = tokens.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    //let loc = tokens.last().unwrap().0;
    //tokens.push((loc, Tok::EndOfFile, loc));
    println!("{:#?}", tokens.iter().map(|x| &x.1).collect::<Vec<_>>());
    println!("{:#?}", ninescript::StatementsParser::new().parse(tokens));
}
