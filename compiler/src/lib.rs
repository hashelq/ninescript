use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod lexer;
pub mod location;
pub mod error;
pub mod token;
pub mod types;
pub mod processor;
lalrpop_mod!(ninescript);

#[test]
fn calculator1() {
    let src = r#"
varip int<int> x = 2
"#.trim_start();
    let tokens = lexer::Lexer::new(src, 4).collect::<Vec<_>>();
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
