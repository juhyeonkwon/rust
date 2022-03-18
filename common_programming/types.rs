//Rust는 타입이 고정되어있는 언어다.
//모든 변수의 타입이 컴파일 시에 반드시 정해져있어야한다.

fn main() {

  /*
  * 정수 타입
  * i8, u8
  * i16, u16, 32, 64
  * arch 타입은 32bit면 32, 64면 64bit
  * -(2^n-1) ~ 2^n-1 - 1 까지
  * 
  * 정수형 리터럴
  * Decimical, Hex, octal, Binary, Byte(u8)
  * 
  * 일반적으로 i32를 쓰에용 왜냐 기본적으로 가장 빠르다고 하네용
  */

  /*
   * 
   * 부동소수점 타입
   * f32, f64
   * 기본은 f64 
   * 
   */

    //bool
    

    //char는 Unicode Scalar임

    //튜플
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    //튜플을 구조해제로 할당할 수 있다.
    let (x, y, z) = tuple;

    println!("{} {} {}", x, y, z);

    //1,2,3 으로 직접 접근할 수 있다
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);


    //배열은 똑같음
   

}