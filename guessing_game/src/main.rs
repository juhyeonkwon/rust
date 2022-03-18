extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //!는 매크로 함수라는 표시

    println!("Guess the number~");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    

    println!("Secret number is : {}", secret_number);


    //loop는 무한루프를 제공함니다람쥐쥐쥐
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
        //let은 변수선언
        //let guess = 5; 하면은 변할수 없는 상수값, mut으로 선언하면 변하는 값이다.
        
        
        //Read_line은 반환값으로 io::Result를 준다.
        //Result는 값이 Err, Ok이며 Err이면 expect가 실행이 된다.
        //Ok이면 반환값은 바이트의 개수
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        //shadowing이라고 하는것... 값의 타입을 변환하고 싶을 경우에 사용한다고 하네요..
        //문자열에서의 parse는 숫자로 변환, 타입 명시가 필요하다람쥐
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("type a number...");
                continue;
            }
        };

        //expect 메소드의 호출을 match로 바꾸는것이 일반적인 방법
        //parse()메소드의 Result타입을 돌려줄때의 값이 Ok, Err이냐에 따라 다르게 함.
            
    
        //&에서 mut을 넣는 이유는 & 참조자가 기본적으로 불변값이기 때문에 가변값으로 바꾸기 위해서 mut을 넣는것이다.
        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less  =>  println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal =>  {
                println!("you win!");
                break;
            },
        }
    }

    

}
