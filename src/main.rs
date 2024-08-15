mod export;
mod parsing;

fn main() {
    parsing::read_input("rules.txt").unwrap();
    export::convert_to_png("output.xml");
}
