use once_cell::sync::Lazy;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub enum SymbolType {
    StockSymbol,
    FutureSymbol,
    Unknown,
}

#[derive(Debug)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub raw_symbol: String,
    pub underlying_symbol: Option<String>,
    pub expiration: String,
}

pub static US_STOCKS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut file = File::open("src/us_stocks.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split("\n").map(|e| e.to_owned()).collect()
});

static FUTURE_MONTHS: [char; 12] = [
 'F', // January –
 'G', //February - 
 'H', // March –
 'J', // April –
 'K', // May –
 'M', // June –
 'N', // July –
 'Q', //August -
 'U', // September –
 'V', // October –
 'X', // November –
 'Z', // December –
];

pub fn parse_advanced(raw_symbol: String, type_hint: Option<SymbolType>) -> Symbol {
    let symbol_type = Some(SymbolType::StockSymbol);

    type ResultType = (String, String);
    let mut results: Vec<ResultType> = vec![];
    let mut curr_value: String = "".to_owned();
    let mut iter = raw_symbol.chars().peekable();
    let mut curr: char;

    while let Some(curr) = iter.next() {
        curr_value.push(curr);

        let peeked = 
        if let Some(nextVal) = iter.peek() {
            if FUTURE_MONTHS.contains(&curr) && nextVal.is_digit(10) {
                results.push(("symbol".to_owned(), curr_value[0..(curr_value.len() - 2)].to_owned()))
                curr_value = "".to_owned();
            }
        }
    }

    Symbol {
        symbol_type: type_hint.or(symbol_type).unwrap(),
        expiration: "".to_owned(),
        underlying_symbol: Some(raw_symbol),
        raw_symbol,
    }
}
