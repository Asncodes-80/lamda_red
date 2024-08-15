mod export;
mod parsing;

fn main() {
    parsing::read_input().unwrap();
    export::convert_to_png();
}
