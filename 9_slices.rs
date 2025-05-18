fn main() {
    let s = String::from("hello my friends");
    println!("{s}");
    println!("*************");
    let first_word_char = find_first_len(&s);
    println!("{first_word_char}");
    println!("*************");
    let first_world = find_first_word(&s);
    println!("{first_world}");
    println!("*************");
    slices(&s);
}

fn find_first_len(s:&String)-> usize {
    let b =s.as_bytes();
    for (i ,&iter) in b.iter().enumerate(){
        if iter==b' '{
                return i;
        }
    }
    s.len()
}

fn find_first_word(s:&String)-> &str {
    let b =s.as_bytes();
    for (i ,&iter) in b.iter().enumerate(){
        if iter==b' '{
                return &s[0..i];
        }
    }
    &s[..]
}

fn slices(s:&String) {

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = find_first_word(&s[0..6]);
    let word = find_first_word(&s[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = find_first_word(&s);
}