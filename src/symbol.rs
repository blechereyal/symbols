use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use rust_decimal::prelude::*;
use crate::symbol_parser::{ParseResult, PutCall};
const FUTURE_MONTHS: [&str; 12] =
    ["F", "G", "H", "J", "K", "M", "N", "Q", "U", "V", "X", "Z"];
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolType {
    Stock,
    Future,
    FutureOption,
    StockOption,
    Unknown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum PutOrCall {
    #[default]
    Put,
    Call,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    pub strike_price: Decimal,
    pub put_call: PutOrCall,
    pub date: Option<NaiveDate>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FutureContract {
    pub continuous: bool,
    pub month: Option<String>,
    pub year: Option<String>,
    pub expiration: Option<NaiveDate>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Symbol {
    pub symbol_type: SymbolType,
    pub original_symbol: String,
    pub underlying_symbol: String,
    pub symbol_modifier: Option<String>,
    pub option_contract: Option<OptionContract>,
    pub future_contract: Option<FutureContract>,
}

impl Default for SymbolType {
    fn default() -> Self {
        Self::Unknown
    }
}

pub fn parse_future_expiration(year: &Option<String>, month: &Option<String>) -> Option<NaiveDate> {
    match (month, year) {
        (None, None) => None,
        (None, _) => None,
        (_, None) => None,
        (Some(month), Some(year)) => {
            let month_index = FUTURE_MONTHS.iter().position(|x| x == month);
            let current_year = Local::now().year();
            let int_year = year.parse::<i32>().ok()?;

            // TODO: revisit this, might be wrong
            let year = match year.len() {
                1 => {
                    (current_year / 10 * 10) + int_year
                },
                2 => {
                    (current_year / 100 * 100) + int_year
                }
                _ => { return None; }
            };

            if let Some(month) = month_index {
                let m= Local::now().date_naive().with_day(1)?.with_month(month as u32 + 1)?.with_year(year)?;
                Some(m)
            } else {
                None
            }
        }
    }
}

// Strike price, as the price x 1000, front padded with 0s to 8 digits
pub fn parse_strike_price(strike_price: String) -> Decimal {
    if strike_price.contains('.') {
        Decimal::from_str(&strike_price).expect("REASON")
    } else { 
        // TODO: expect
        let num = strike_price.parse::<i64>().expect("String");
        match strike_price.len() {
            8 => {
                Decimal::new(num, 3)
            }, 
            _ => {
                Decimal::new(num, 0)
            }
        }
    }
}

impl From<ParseResult> for Symbol {
    fn from(val: ParseResult) -> Self {
        match val {
            ParseResult::UnknownSym(symbol) => Symbol {
                original_symbol: symbol.original_symbol,
                underlying_symbol: symbol.root_symbol,
                ..Default::default()
            },
            ParseResult::StockOptionsSym(symbol) => {
                let date = symbol.date;
                // TODO: try_into for this
                let date =
                    NaiveDate::parse_from_str(&date, "%y%m%d").expect("undefined date format");

                let option_contract = OptionContract {
                    strike_price: parse_strike_price(symbol.strike_price),
                    put_call: match symbol.put_call {
                        PutCall::Put => PutOrCall::Put,
                        PutCall::Call => PutOrCall::Call,
                    },
                    date: Some(date),
                };
                Symbol {
                    symbol_type: SymbolType::StockOption,
                    original_symbol: symbol.original_symbol,
                    underlying_symbol: symbol.root_symbol,
                    option_contract: Some(option_contract),
                    ..Default::default()
                }
            }
            ParseResult::FutureSym(symbol) => {
                let expiration = parse_future_expiration(&symbol.year, &symbol.month);

                let future_contract = FutureContract {
                    continuous: symbol.continuous,
                    month: symbol.month,
                    year: symbol.year,
                    expiration
                };

                Symbol {
                    symbol_type: SymbolType::Future,
                    original_symbol: symbol.original_symbol,
                    underlying_symbol: symbol.root_symbol,
                    future_contract: Some(future_contract),
                    ..Default::default()
                }
            }
            ParseResult::FutureOptionsSym(symbol) => {
                let option_contract = OptionContract {
                    strike_price: parse_strike_price(symbol.strike_code),
                    put_call: match symbol.put_call {
                        PutCall::Put => PutOrCall::Put,
                        PutCall::Call => PutOrCall::Call,
                    },
                    date: None,
                };
                let expiration = parse_future_expiration(&symbol.future_symbol.year, &symbol.future_symbol.month);
                let future_contract = FutureContract {
                    continuous: symbol.future_symbol.continuous,
                    month: symbol.future_symbol.month,
                    year: symbol.future_symbol.year,
                    expiration
                };
                Symbol {
                    symbol_type: SymbolType::FutureOption,
                    original_symbol: symbol.original_symbol,
                    underlying_symbol: symbol.future_symbol.root_symbol,
                    future_contract: Some(future_contract),
                    option_contract: Some(option_contract),
                    ..Default::default()
                }
            }
            ParseResult::Unused => Default::default(),
        }
    }
}
