
//옵션 타입은 값이 있거나 없을 수도 있는 아주 흔한 상황을 나타냄
//러스트는 null이 없다람쥐

/**
 * enum Option<T> {
 *  Some(T),
 *  None,
 * }
 */

//Some, None을 바로 사용할 수 있다.


fn main() {
    
    let number = Some(5);
    let str = Some("a String");

    //None을 쓰고 싶다면 타입을 명시해야한다.
    let n2: Option<i32> = None;


    //Option을 사용하려면 여러 연산을 해야한다.
    //https://doc.rust-lang.org/std/option/enum.Option.html 여기서 확인가능
    assert_eq!(number.is_some(), true);

    //연산을 하기위해선 Option<T>를 T로 변환해야한다.

    
    


}