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

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    if slice == &[2, 3] {
        println!("equal slices");
    };
}
