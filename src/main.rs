mod libs;

use libs::matcher::Matcher;
use libs::parser::Parser;
use libs::scanner::Scanner;

fn main() {
    let scan = Scanner::new("(12*3)abc(4|5)*");
    let mut parser = Parser::new(scan);
    let ast = parser.parse();
    if let Some(a) = ast {
        let matcher = Matcher::new(a);
        match matcher.match_("12223abc454545545454") {
            Ok(_) => println!("success"),
            Err(e) => println!("failed: {}", e)
        }
    }
}
