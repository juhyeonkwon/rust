use std::io;

fn main() {

  println!("Enter a number.");

  let mut number = String::new();

  io::stdin().read_line(&mut number)
      .expect("error occured");

  let number : i32 = number.trim().parse()
                            .expect("Type a number");


  println!("{}", fibonacci(number));

}


fn fibonacci(n : i32) -> i32 {
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}