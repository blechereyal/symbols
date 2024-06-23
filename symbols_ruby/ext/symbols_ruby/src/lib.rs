use magnus::{class, define_module, function, method, prelude::*, Error, RBignum, TryConvert, Value};
use symbols::symbol::{FutureContract, OptionContract, Symbol, SymbolType};
use rust_decimal::prelude::ToPrimitive;
#[derive(Debug)]
#[magnus::wrap(class = "SymbolsRuby::FutureContract", free_immediately, size)]
pub struct RFutureContract {
    pub future_contract: FutureContract
}

impl RFutureContract {
    pub fn inspect(&self) -> String {
        format!("{:?}", self.future_contract)
    }

    pub fn continuous(&self) -> bool {
        self.future_contract.continuous
    }

    pub fn expiration(&self) -> Option<String> {
        self.future_contract.expiration.map(|e| e.to_string())
    }
}


#[derive(Debug)]
#[magnus::wrap(class = "SymbolsRuby::OptionContract", free_immediately, size)]
pub struct ROptionContract {
    pub option_contract: OptionContract
}

impl ROptionContract {
    pub fn inspect(&self) -> String {
        format!("{:?}", self.option_contract)
    }

   pub fn strike_price(&self) -> f64 {
    let a = self.option_contract.strike_price.to_f64().unwrap();
    
    a
   }

   pub fn put_call(&self) -> magnus::Symbol {
    match self.option_contract.put_call {
        symbols::symbol::PutOrCall::Put => magnus::Symbol::new("put"),
        symbols::symbol::PutOrCall::Call => magnus::Symbol::new("call"),
    }
   }

   pub fn date(&self) -> Option<String> {
    self.option_contract.date.map(|e| e.to_string())
}
}


#[derive(Debug)]
#[magnus::wrap(class = "SymbolsRuby::Symbol", free_immediately, size)]
pub struct RSymbol {
    pub symbol: Symbol
}

impl RSymbol {
    pub fn inspect(&self) -> String {
        format!("{:?}", self.symbol)
    }

    pub fn original_symbol(&self) -> String {
        self.symbol.original_symbol.to_owned()
    }

    pub fn underlying_symbol(&self) -> String {
        self.symbol.underlying_symbol.to_owned()
    }

    pub fn symbol_modifier(&self) -> Option<String> {
        self.symbol.symbol_modifier.to_owned()
    }
    
    pub fn symbol_type(&self) -> magnus::Symbol {
        match self.symbol.symbol_type {
            SymbolType::Stock => magnus::Symbol::new("stock"),
            SymbolType::Future => magnus::Symbol::new("future"),
            SymbolType::FutureOption => magnus::Symbol::new("future_option"),
            SymbolType::StockOption => magnus::Symbol::new("stock_option"),
            SymbolType::Unknown => magnus::Symbol::new("unknown"),
        }
    }

    pub fn future_contract(&self) -> Option<RFutureContract> {
        self.symbol.future_contract.as_ref().map(|e| {
            RFutureContract { future_contract: e.to_owned() }
        })
    }

    pub fn option_contract(&self) -> Option<ROptionContract> {
        self.symbol.option_contract.as_ref().map(|e| {
            ROptionContract { option_contract: e.to_owned() }
        })
    }
}


fn parse_symbol(subject: String) -> RSymbol {
    let result = symbols::symbol_parser::parse_symbol(&subject);
    match result {
        Ok(result) => {
            let symbol: symbols::symbol::Symbol = result.into();
            RSymbol { symbol }
        },
        Err(_) => todo!()
    } 
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("SymbolsRuby")?;
    let option_class = module.define_class("OptionContract", class::object())?;
    option_class.define_method("inspect", method!(ROptionContract::inspect, 0))?;
    option_class.define_method("strike_price", method!(ROptionContract::strike_price, 0))?;
    option_class.define_method("put_call", method!(ROptionContract::put_call, 0))?;
    option_class.define_method("date", method!(ROptionContract::date, 0))?;

    let future_class = module.define_class("FutureContract", class::object())?;
    future_class.define_method("inspect", method!(RFutureContract::inspect, 0))?;
    future_class.define_method("continuous", method!(RFutureContract::continuous, 0))?;
    future_class.define_method("expiration", method!(RFutureContract::expiration, 0))?;

    let class = module.define_class("Symbol", class::object())?;
    class.define_method("inspect", method!(RSymbol::inspect, 0))?;
    class.define_method("original_symbol", method!(RSymbol::original_symbol, 0))?;
    class.define_method("underlying_symbol", method!(RSymbol::underlying_symbol, 0))?;
    class.define_method("symbol_modifier", method!(RSymbol::symbol_modifier, 0))?;
    class.define_method("symbol_type", method!(RSymbol::symbol_type, 0))?;
    class.define_method("future_contract", method!(RSymbol::future_contract, 0))?;
    class.define_method("option_contract", method!(RSymbol::option_contract, 0))?;

    module.define_singleton_method("parse_symbol", function!(parse_symbol, 1))?;
    Ok(())
}
