fn main() {
    let sentence = String::from("Hello, world!");
    let first = first_word(&sentence);
    print!("{first}\n");
}

fn first_word(s: &String) -> usize {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return i;
        }
    }

    s.len()
}
