fn main() {

    let mut s = String::new();

    let data = "init contents";

    let b = data.to_string();

    s.push_str("ststs");


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);


    //String은 Vec<u8>을 감싼것
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

}