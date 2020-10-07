use std::io::Read;

fn main() {
    let mut sum = 0.0;
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let num = line.parse::<f32>().unwrap();
        sum += (num / 3.0).floor() - 2.0;
    }

    println!("{}", sum);
}
