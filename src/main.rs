use itertools::Itertools;

fn main() {
    let mut res = vec![];

    for i0 in 0..10 {
    for i1 in 0..10 {
    for i2 in 0..10 {
    for i3 in 0..10 {
    for i4 in 0..10 {
    for i5 in 0..10 {
        if let Some(_) = Password::new(vec![i0, i1, i2, i3, i4, i5]) {
            res.push(vec![i0, i1, i2, i3, i4, i5])
        }
    }}}}}}

    println!("Count: {}", res.len());
}

#[derive(Debug)]
struct Password {
    digits: [u8; 6],
}

impl Password {
    fn new(from: Vec<u8>) -> Option<Self> {
        if !(from[0] <= from[1] && from[1] <= from[2] && from[2] <= from[3] && from[3] <= from[4] && from[4] <= from[5]) {
            return None;
        }

        if !(from[0] == from[1] || from[1] == from[2] || from[2] == from[3] || from[3] == from[4] || from[4] == from[5]) {
            return None;
        }

        let number = from[5] as u32 * 10u32.pow(0)
            + from[4] as u32 * 10u32.pow(1)
            + from[3] as u32 * 10u32.pow(2)
            + from[2] as u32 * 10u32.pow(3)
            + from[1] as u32 * 10u32.pow(4)
            + from[0] as u32 * 10u32.pow(5);
        if !(number >= 137683 && number <= 596253) { 
            return None;
        }

        let digits = [from[0], from[1], from[2], from[3], from[4], from[5]];
        Some(Password { digits })
    }
}

#[test]
fn test_pw_new() {
    Password::new(vec![2, 3, 4, 4, 5, 6]).unwrap();

    assert!((0..10).permutations(6).any(|i| i == vec![2, 3, 4, 4, 5, 6]));
}
