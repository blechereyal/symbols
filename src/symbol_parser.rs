
use std::future::{self, Future};

use pest::{iterators::{Pair, Pairs}, Parser};

use crate::symbol::{self, Symbol};

#[derive(Debug)]
enum PutCall {
    Put,
    Call
}

#[derive(pest_derive::Parser)]
#[grammar = "symbols.pest"]
pub struct SymbolParser;

#[derive(Debug)]
pub struct UnknownSymbol {
    root_symbol: String,
    symbol_modifier: Option<String>
}

#[derive(Debug)]
pub struct FutureSymbol {
    continuous: bool,
    root_symbol: String,
    month: Option<String>,
    year: Option<String>,
    symbol_modifier: Option<String>
}

#[derive(Debug)]
pub struct StockOptionsSymbol {
    root_symbol: String,
    strike_price: String,
    put_call: PutCall,
    date: String
}


#[derive(Debug)]
pub struct FutureOptionsSymbol {
    future_symbol: FutureSymbol,
    strike_code: String,
    put_call: PutCall,
    symbol_modifier: Option<String>
}


#[derive(Debug)]
pub enum ParseResult {
    UnknownSym(UnknownSymbol),
    StockOptionsSym(StockOptionsSymbol),
    FutureSym(FutureSymbol),
    FutureOptionsSym(FutureOptionsSymbol),
    Unused,

}

pub fn parse_symbol(raw_symbol: &str) -> Result<ParseResult, ()> {
    let reversed_symbol = raw_symbol.chars().rev().collect::<String>();

    let tokens = SymbolParser::parse(Rule::symbol, &reversed_symbol).map_err(|err| {
        eprintln!("Error in parsing symbol: {err}");
    })?;

    for tree in tokens  {
        // Get only the root symbols
        match tree.as_rule() {
            Rule::future_options_symbol | Rule::stock_options_symbol | Rule::future_symbol | Rule::unknown_symbol => {
               return parse_tokens(tree);
            }, 
            _ => {}
        }
    }
    Err(())
}

fn reverse(str: String) -> String {
    str.chars().rev().collect()
}

fn find_by_rule<'a>(symbol_tokens: &'a [Pair<'a, Rule>], rule: Rule) -> Option<&Pair<'a, Rule>> {
    symbol_tokens.iter().find(|sym| sym.as_rule() == rule)
}

fn parse_put_call(token: &Pair<Rule>) -> Result<PutCall, ()> {
    if token.as_rule() != Rule::put_call {
        return Err(())
    }

    for inner_token in token.to_owned().into_inner() {
        match inner_token.as_rule() {
            Rule::put => { return Ok(PutCall::Put) },
            Rule::call => { return Ok(PutCall::Call) }
            _ => {}
        }
    }
    Err(())
}

type IndividualContract = (String, String, String);
fn parse_individual_contact(token: &Pair<Rule>) -> Result<IndividualContract, ()> {
    let symbol_tokens: Vec<Pair<Rule>> = token.to_owned().into_inner().collect();
    let future_month: Option<&Pair<Rule>> = find_by_rule(&symbol_tokens, Rule::future_month);
    let year: Option<&Pair<Rule>> = find_by_rule(&symbol_tokens, Rule::year);
    let root_symbol: Option<&Pair<Rule>> = find_by_rule(&symbol_tokens, Rule::root_sym);

    if root_symbol.is_none() || year.is_none() || future_month.is_none() {
        return Err(())
    }

    Ok((
        reverse(root_symbol.unwrap().as_span().as_str().to_owned()),
        reverse(future_month.unwrap().as_span().as_str().to_owned()),
        reverse(year.unwrap().as_span().as_str().to_owned()),
    )) 
}

fn parse_tokens(tokens: Pair<Rule>) -> Result<ParseResult, ()> {
    match tokens.as_rule() {
        Rule::unknown_symbol => {
            let symbol_tokens: Vec<Pair<Rule>> = tokens.into_inner().collect();
            let root_symbol = find_by_rule(&symbol_tokens, Rule::root_sym);
            let symbol_modifier = find_by_rule(&symbol_tokens, Rule::symbol_modifier);

            if root_symbol.is_none() {
                return Err(())
            }

            Ok(ParseResult::UnknownSym(UnknownSymbol {
                symbol_modifier: symbol_modifier.map(|pair| reverse(pair.as_span().as_str().to_owned())),
                root_symbol: reverse(root_symbol.unwrap().as_span().as_str().to_owned()),
            }))
        },
        Rule::stock_options_symbol => {
            let symbol_tokens: Vec<Pair<Rule>> = tokens.into_inner().collect();
            let root_symbol = find_by_rule(&symbol_tokens, Rule::root_sym);
            let strike_price = find_by_rule(&symbol_tokens, Rule::strike_price);
            let put_call = find_by_rule(&symbol_tokens, Rule::put_call);
            let date = find_by_rule(&symbol_tokens, Rule::date);

            if root_symbol.is_none() || strike_price.is_none() || date.is_none() || put_call.is_none() {
                return Err(());
            }

            let put_call = parse_put_call(put_call.unwrap())?;
            
            Ok(ParseResult::StockOptionsSym(StockOptionsSymbol {
                root_symbol: reverse(root_symbol.unwrap().as_span().as_str().to_owned()),
                strike_price: reverse(strike_price.unwrap().as_span().as_str().to_owned()),
                date: reverse(date.unwrap().as_span().as_str().to_owned()),
                put_call
            }))
        },
        Rule::future_symbol => {
            let symbol_tokens: Vec<Pair<Rule>> = tokens.into_inner().collect();
            let root_symbol = find_by_rule(&symbol_tokens, Rule::root_sym);
            let symbol_modifier = find_by_rule(&symbol_tokens, Rule::symbol_modifier);
            let individual_contract = find_by_rule(&symbol_tokens, Rule::individual_contract);
            let continuous_modifier = find_by_rule(&symbol_tokens, Rule::continuous_modifier);

            if individual_contract.is_some() {
                let individual_contract = individual_contract.unwrap();

                let (root_symbol, future_month, year) = parse_individual_contact(individual_contract)?;


                return Ok(ParseResult::FutureSym(FutureSymbol {
                    continuous: continuous_modifier.map_or_else( || false, |_| true),
                    root_symbol,
                    year: Some(year), 
                    month: Some(future_month),
                    symbol_modifier: symbol_modifier.map(|pair| reverse(pair.as_span().as_str().to_owned())),
                }))
            } else {
                if root_symbol.is_none() || continuous_modifier.is_none() {
                    return Err(());
                }

                return Ok(ParseResult::FutureSym(FutureSymbol {
                    continuous: true,
                    root_symbol: reverse(root_symbol.unwrap().as_span().as_str().to_owned()),
                    year: None, 
                    month: None,
                    symbol_modifier: symbol_modifier.map(|pair| reverse(pair.as_span().as_str().to_owned())),
                }))
            }

            Err(())
        },
        Rule::future_options_symbol => {
            let symbol_tokens: Vec<Pair<Rule>> = tokens.into_inner().collect();
            println!("{:?}", symbol_tokens);
            let future_symbol = find_by_rule(&symbol_tokens, Rule::future_symbol);
            let symbol_modifier = find_by_rule(&symbol_tokens, Rule::symbol_modifier);

            let strike_code = find_by_rule(&symbol_tokens, Rule::strike_code);
            let put_call = find_by_rule(&symbol_tokens, Rule::put_call);

            if future_symbol.is_none() || strike_code.is_none() || put_call.is_none() {
                return Err(());
            }
            let put_call = parse_put_call(put_call.unwrap())?;
            let future_symbol = parse_tokens(future_symbol.unwrap().to_owned())?;
            if let ParseResult::FutureSym(future_symbol) = future_symbol {
                Ok(ParseResult::FutureOptionsSym(FutureOptionsSymbol {
                    future_symbol,
                    strike_code: reverse(strike_code.unwrap().as_span().as_str().to_owned()),
                    put_call,
                    symbol_modifier: symbol_modifier.map(|pair| reverse(pair.as_span().as_str().to_owned())),
                }))
            } else {
                Err(())
            }
           
        },
        _ => todo!("TODO")
    }
}
