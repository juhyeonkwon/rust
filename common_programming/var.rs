fn main() {

  let x = 5;  //불변성 변수

  print!("{}", x);

  // x = 6; 이렇게 재할당하면 오류가 난다

  let mut y = 6;

  print!("{}", y);

  y = 4;

  print!("{}", y);

  //상수의 키워드는 const이며 타입을 명시해야함

  //const CON: u32 = 32;
  
  
  //Shadowing, 선언한 변수와 같은 이름의 새 변수를 선언하는 것이다. 

  let x = 4;

  println!("{}",x);


}