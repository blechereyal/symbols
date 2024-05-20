use std::{fs::File, io::Read};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
enum SymbolType {
    StockSymbol,
    FutureSymbol,
    Unknown,
}

#[derive(Debug)]
struct Symbol {
    symbol_type: SymbolType,
    raw_symbol: String,
    underlying_symbol: Option<String>,
    expiration: String,
}
static MONTH_REGEX: &str = r"(?<month>F|G|H|J|K|M|N|Q|U|V|X|Z)";
static YEAR_REGEX: &str = r"(?<year>\d{1,2}|\d{4})";

static FUTURE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(&format!("(?<symbol>.+){MONTH_REGEX}{YEAR_REGEX}")).unwrap());
static US_STOCKS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut file = File::open("src/us_stocks.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|e| e.to_owned()).collect()
});

impl Symbol {
    pub fn parse(raw_symbol: String, type_hint: Option<SymbolType>) -> Symbol {
        let (symbol_type, underlying_symbol) = if US_STOCKS.contains(&raw_symbol) {
            (Some(SymbolType::StockSymbol), None)
        } else if FUTURE_RE.is_match(&raw_symbol) {
            let underlying_symbol = FUTURE_RE.captures(&raw_symbol).unwrap();
            let underlying_symbol = underlying_symbol.name("symbol");
            (
                Some(SymbolType::FutureSymbol),
                Some(underlying_symbol.unwrap().as_str().to_owned()),
            )
        } else {
            (Some(SymbolType::Unknown), None)
        };

        Symbol {
            symbol_type: type_hint.or(symbol_type).unwrap(),
            expiration: "".to_owned(),
            underlying_symbol,
            raw_symbol,
        }
    }
}

fn main() {
    let sym = Symbol::parse("AAPL".to_owned(), None);
    let sym2 = Symbol::parse("ASM4".to_owned(), None);

    println!("{:?} {:?}", sym, sym2);
}
