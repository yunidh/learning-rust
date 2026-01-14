fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    //slicing reference for string
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn main() {
    let s = String::from("hello world");

    let word = first_word(&s); //returns first word found where string is separated by spaces

    println!("{word}");
}
