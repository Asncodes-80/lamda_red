mod export;
mod parsing;

fn main() {
    parsing::read_input("sample.zz").unwrap();
    export::convert_to_png("output.xml");
}
