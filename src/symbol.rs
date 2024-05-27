use wasm_bindgen::prelude::wasm_bindgen;

use crate::symbol_parser::ParseResult;


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum SymbolType {
    Stock,
    Future,
    FutureOption,
    StockOption,
    Unknown,
}


// #[derive(Debug, Default)]
// pub struct Contract {
//     pub strike_price: u32
// }

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub original_symbol: String,
    pub underlying_symbol: Option<String>,
    pub symbol_modifier: Option<String>,
    // pub contract: Option<Contract>
}

impl Default for SymbolType { 
    fn default() -> Self {
        Self::Unknown
    }
}

// impl Symbol {
//     fn new_from_unkown(symbol: UnknownSymbol) -> Self {
//         Symbol { raw_symbol: symbol.root_symbol, ..Default::default() }
//     }
// }

impl Into<Symbol> for ParseResult {
    fn into(self) -> Symbol {
        match self {
            ParseResult::UnknownSym(symbol) => {
                Symbol { original_symbol: symbol.original_symbol, underlying_symbol: Some(symbol.root_symbol), ..Default::default() }
            },
            ParseResult::StockOptionsSym(symbol) => {
                Symbol { original_symbol: symbol.original_symbol, underlying_symbol: Some(symbol.root_symbol), ..Default::default() }
            },
            ParseResult::FutureSym(_) => todo!(),
            ParseResult::FutureOptionsSym(_) => todo!(),
            ParseResult::Unused => todo!(),
        }
    }
}
