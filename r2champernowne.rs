fn champ(mut i: u32) -> char {
    let count = |i: u32| 10u32.pow(i - 1) * i;

    let mut n = 1u32;
    while i >= 9 * count(n) {
        i -= 9 * count(n);
        n += 1;
    }

    let w = (i + count(n)) / (count(n) / 10u32.pow(i % n)) % 10;
    char::from_digit(w, 10).unwrap()
}

fn main() {
    println!("{}", champ(1));
}
