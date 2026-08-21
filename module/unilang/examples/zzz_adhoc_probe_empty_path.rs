//! Adversarial probe - not part of test suite, ad-hoc verification only.
fn main() {
    let parser = unilang_parser::Parser::new(unilang_parser::UnilangParserOptions::default());
    let instr = parser.parse_repl_input(". some_unknown_param::xyz").unwrap();
    println!("command_path_slices: {:?}", instr.command_path_slices);
    println!("named_arguments keys: {:?}", instr.named_arguments.keys().collect::<Vec<_>>());
    println!("positional_arguments: {:?}", instr.positional_arguments);
}
