struct User {
    username : String,
    email : String,
    sign_in_count: u64,
    active: bool,
}


#[derive(Debug)]
struct Rectangle {
    length : u32,
    width: u32,
}

//구조체 안에 함수를 정의하기 위해서는 impl(구현) 블록을 시작해야함

impl Rectangle {

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        if self.area() > other.area() {
            true

        } else {
            false
        }
    }

}

//파생 트레잇으로 유용한 기능 추가할 수 있음 (derived trait)

//println은 타입들에 구현되어 있는 Display라고 알려진 포맷팅을 이용한다.
//struct나 이런것 들은 기본 구현체가 없다.

fn main() {

    let mut user1 = User {
        email: String::from("rnjstksaor@gmail.com"),
        username: String::from("kwonjuhyeon"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("rn"),
        ..user1
    };


    let rect1 = Rectangle { length: 50, width : 50};
    let rect2 = Rectangle { length: 20, width : 40};

    println!("{}", rect1.area());

    //구조체를 출력하고 싶으면 :? 를 넣어줘야함
    println!("{:#?}", rect1);

    println!("can rect1 hold rect2 ? : {}", rect1.can_hold(&rect2));
    println!("can rect2 hold rect1 ? : {}", rect2.can_hold(&rect1));



        

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}