use mylib::intial_method::parse_initial;

fn main() {
    // let sym = parse_initial("AAPL".to_owned(), None);
    // let sym2 = parse_initial("ASM4".to_owned(), None);

    let sym = parse_advanced("AAPL".to_owned(), None);
    let sym2 = parse_initial("ASM4".to_owned(), None);

    println!("{:?} {:?}", sym, sym2);
}
