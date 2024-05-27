use std::future::{self, Future};

use wasm_bindgen::prelude::wasm_bindgen;

use crate::symbol_parser::{ParseResult, PutCall};


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum SymbolType {
    Stock,
    Future,
    FutureOption,
    StockOption,
    Unknown,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone, Default)]
pub enum PutOrCall {
    #[default] 
    Put, 
    Call
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct OptionContract {
    pub strike_price: String,
    pub put_call: PutOrCall,
    pub date: Option<String>
}


#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default, Clone)]
pub struct FutureContract {
    pub continuous: bool,
    pub month: Option<String>,
    pub year: Option<String>,
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Default)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub original_symbol: String,
    pub underlying_symbol: String,
    pub symbol_modifier: Option<String>,
    pub option_contract: Option<OptionContract>,
    pub future_contract: Option<FutureContract>
}

impl Default for SymbolType { 
    fn default() -> Self {
        Self::Unknown
    }
}

impl Into<Symbol> for ParseResult {
    fn into(self) -> Symbol {
        match self {
            ParseResult::UnknownSym(symbol) => {
                Symbol { original_symbol: symbol.original_symbol, underlying_symbol: symbol.root_symbol, ..Default::default() }
            },
            ParseResult::StockOptionsSym(symbol) => {
                let option_contract = OptionContract { 
                    strike_price: symbol.strike_price,
                    put_call: match symbol.put_call {
                        PutCall::Put => PutOrCall::Put,
                        PutCall::Call => PutOrCall::Call,
                    },
                    date: Some(symbol.date)
                };
                Symbol { symbol_type: SymbolType::StockOption, original_symbol: symbol.original_symbol, underlying_symbol: symbol.root_symbol, option_contract: Some(option_contract), ..Default::default() }
            },
            ParseResult::FutureSym(symbol) => {
                let future_contract = FutureContract {
                    continuous: symbol.continuous,
                    month: symbol.month,
                    year: symbol.year
                };

                Symbol {symbol_type: SymbolType::Future,  original_symbol: symbol.original_symbol, underlying_symbol: symbol.root_symbol, future_contract: Some(future_contract), ..Default::default() }
            },
            ParseResult::FutureOptionsSym(symbol) => {
                let option_contract = OptionContract { 
                    strike_price: symbol.strike_code,
                    put_call: match symbol.put_call {
                        PutCall::Put => PutOrCall::Put,
                        PutCall::Call => PutOrCall::Call,
                    },
                    date: None
                };
                let future_contract = FutureContract {
                    continuous: symbol.future_symbol.continuous,
                    month: symbol.future_symbol.month,
                    year: symbol.future_symbol.year
                };
                Symbol { symbol_type: SymbolType::FutureOption, original_symbol: symbol.original_symbol, underlying_symbol: symbol.future_symbol.root_symbol, future_contract: Some(future_contract), option_contract: Some(option_contract), ..Default::default() }

            },
            ParseResult::Unused => Default::default(),
        }
    }
}
