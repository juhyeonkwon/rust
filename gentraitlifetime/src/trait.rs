extern crate gentraitlifetime;

fn main() {

  let b = NewArticle{headline : String::from("dtd"), location : String::from("dtd"), author : String::from("dtd"), content : String::from("dtd")};

  println!("{}",b.summary());

}