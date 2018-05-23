mod lib;

use lib::matcher::Matcher;
use lib::parser::Parser;
use lib::scanner::Scanner;

fn main() {
    let scan = Scanner::new("(12*3)ab(123(456)*)*c(4|5)*");
    let mut parser = Parser::new(scan);
    let ast = parser.parse();
    if let Some(a) = ast {
        let matcher = Matcher::new(a);
        match matcher.match_("12223ab123456456123456456c454545545454") {
            Ok(_) => println!("success"),
            Err(e) => println!("failed: {}", e)
        }
    }
}
