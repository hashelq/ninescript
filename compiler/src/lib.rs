use lalrpop_util::{lalrpop_mod, ParseError};
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
var array<int> x = na
array x = na
series a<b> x = 8
x.y()
x.y()
x.y<int>()
"#.trim_start();
    let tokens = Lexer::new(src, 4).collect::<Vec<_>>();
    let tokens = tokens.into_iter().map(|x| x.unwrap()).collect::<Vec<_>>();
    let result = ninescript::StatementsParser::new().parse(tokens);
    println!("{:?}", result);
    let result = match result {
        Ok(x) => x,
        Err(e) => {
            let mut y = false;
            e.map_location(|x| {
                if y == true {return;}
                y = true;
                let vis = x.visualize(src.split('\n').nth(x.row()-1).unwrap_or(""));
                println!("\n\nError at {}:{}\n{}\n\n", x.row(), x.column(), vis);
            });
            return;
        }
    };
    println!("{:#?}", result);
}
