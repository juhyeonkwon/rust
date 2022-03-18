//러스트에서 함수는 뱀형식(밑줄)로 표현
//매개변수에는 반드시 타입이 들어가야함니다람쥐...

//구문은 어떤 명령들의 나열로 값을 반환하지 않는 어떤 동작을 수행
//표현식은 결과 값을 산출
//표현식은 ;가 들어가면 암댐

fn main() {
  let x = 6; //이건 구문이다

  let y = {
    let _x = 3;  //블록스코프
    _x + 1       // 여기에 ; 가 들어가면 구문이 되어버려서 오류가 난다람쥐...
  };

  let z = has_return_fnc2(5);

  println!("{}",z);

  println!("이건 블록 표현식을 사용한 구문 {}", y);

  print_str();
  function_with_parameter(x, 2);
  println!("{}", has_return_fnc());
}


fn print_str() {
  println!("function call")
}

fn function_with_parameter(x: i32, y: i32) {
  println!("{} {}", x, y);
}

//반환값이 있는 함수
//일반적으로 맨 밑에있는 표현식이 반환이 된다. 명시적으로 return을 해줘도 댐 화살표를 쓴다람쥐

fn has_return_fnc() -> u32 {
  5
}

fn has_return_fnc2(x : i32) -> i32 {
  x + 1
}
