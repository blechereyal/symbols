use symbols::{symbol::Symbol, symbol_parser::parse_symbol};
fn main() {
    // let sym = Symbol::parse("AAPL".to_owned(), None);
    // let sym2 = Symbol::parse("ASM4".to_owned(), None);

    // stock
    // let aapl = sym("AAPL");
    // future
    // let asm4 = sym("ASM4");
    // let asm4_mod = sym("ASM4.ZERO");
    // let cont_asm4 = sym("@ASM4");
    // let cont_as = sym("@AS");
    // // Future Options
    // // E-Mini S&P 500 September 2020 2700 Call. Future Option
    // let eminicall = sym("ESU20 C2700");
    // let eminicall_full: String = sym("ESU20 CALL2700");
    // // Stock Options
    // let ms = sym("MSFT 110122C27.5");
    
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &aapl));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &asm4));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &asm4_mod));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &cont_asm4));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &cont_as));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &eminicall));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &eminicall_full));
    // println!("{:?}", SymbolParser::parse(Rule::symbol, &ms));
    // println!("{:?}", parse_symbol("AAPL"));
    println!("{:?}", parse_symbol("MSFT 110122C27.5"));
    println!("{:?}", parse_symbol("@ASM4.ZERO"));
    println!("{:?}", parse_symbol("@AS.TEST"));
    println!("{:?}", parse_symbol("ESU20 C2700"));
    
    println!("{:?}", parse_symbol("SPY251219C00650000"));

    println!("{:?}", Symbol { original_symbol: "AA".to_owned(), ..Symbol::default() });

}
