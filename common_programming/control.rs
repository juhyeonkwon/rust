fn main() {

  //조건
  let number = 3;

  if number < 5 {
    println!("less");
  } else {
    println!("great")
  }

  //이렇게도 변수를 할당 할 수 있다람쥐
  let number2 = if number < 5 {
    2
  } else {
    3
  };
  // 기억해야할 것 코드 블록은 그들이 마지막에 위치한 표현식을 산출하게 된다! 숫자는 그 자체로 표현식이다.
  // 여기서 반환하는 값들의 데이터 타입이 일치해야 한다. 다르면 오류가남

  //반복문

  // 1. loop
  let mut num = 1;

  loop {
    if num > 3 {
      break;
    } else {
      println!("number : {}", num);
      num = num + 1;
    }
  }

  // 2. while 

  let mut num2 = 0;
  while num2 != 0 {
    println!("while {}", num2);
    num2 = num2 - 1;
  }

  println!("{}", number2);

  // 3. for

  let arr = [1,2,3,4,5];

  for ele in arr.iter() {
    println!("arr in : {}", ele);
  }

  for number in (1..4).rev() {
    println!("{}!", number);
  }

}