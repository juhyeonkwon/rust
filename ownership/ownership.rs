fn main() {

    let mut s1 = String::from("Hi");

    mutmut(&mut s1);


    print!("{}", s1);




}


fn mutmut(s : &mut String) {

    s.push_str("dtdtdtd");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}