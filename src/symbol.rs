

#[derive(Debug)]
pub enum SymbolType {
    StockSymbol,
    FutureSymbol,
    Unknown,
}

#[derive(Debug)]
pub struct Symbol {
    symbol_type: SymbolType,
    raw_symbol: String,
    underlying_symbol: Option<String>,
    expiration: String,
}


