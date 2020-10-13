mod intcode;

fn main() {
    intcode::Program::load(include_str!("../input")).execute();
}
