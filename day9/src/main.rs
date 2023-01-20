mod rope;
mod rope_parser;

use rope::Rope;
use rope_parser::RopeParser;

fn main() {
    let mut rp = RopeParser::new(Rope::new((0, 0)));
    rp.parse_file("src/input.txt");
    // let string = fs::read_to_string("src/input.txt").expect("file not found");
    // rp.parse(&string);

    println!("{:?}", rp.get_pos());
    println!("{}", rp.get_tail_count());
    // println!("{:?}", string);
}