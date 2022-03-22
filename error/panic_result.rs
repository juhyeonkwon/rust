use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {

    //panic!("패닉");

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        //&는 참조자를 매치하고 그 값을 제공, ref는 값을 매치하여 그 참조자를 제공
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };





    //unwrap, Result값이 Ok면 Ok의 값을 반환, Result가 Err면 panic! 호출

    let f = File::open("hello.txt").unwrap();



    //expect는 panic!에러 메세지를 낼 수 있다람쥐

    let f = File::open("hello.txt").expect("파일이 없다람쥐임니다람쥐");

    read_username_from_file();

    //에러 넘기기 위한 ?
    match read_username_from_file2() {
        Ok(s) => println!("{}", s),
        Err(e) =>  println!("{}", e),
    }
}


//함수로 에러를 넘기는 방법
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

}

//?
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)

}