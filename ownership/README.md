# 소유권 규칙

1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

## String 타입

```
fn main() [
  let s_liter = "hello" //스트링 리터럴은 하드코딩 되어 있음

  let mut s_type = String::from("hello");
]

```

String타입은 변할 수 있다. 그이유는 메모리를 쓰는 방식이 다르기 때문

## 메모리와 할당

- 스트링 리터럴은 컴파일 단계에 바로 알 수 있다. 하지만 변하지 못함
- String 타입은 변경 가능, 커질 수 있는 텍스트를 지원함

이말은 즉

1. 런타임에 운영체제로 부터 메모리 요청을 해야함
2. String의 사용이 끝났을때 운영체제에게 메모리를 반납할 방법이 있어야함

일반적인 프로그래밍 언어는 alloc하고 free 쌍을 이뤄야하지만 러스트는 다르다
러스트에서는 변수가 소속되어 있는 스코프 밖으로 나가는 순간 자동 반납이 된다.

```
{
  let s = String::from("hi"); // 유효

} // 블록에서 벗어나면 s는 유효 하지 않음
```

변수가 스코프 밖으로 벗어나면, 러스트는drop이라는 함수를 호출 하고, 메모리를 반환하도록 하는 코드를 집어넣을 수 있습니다. 러스트는 } 괄호가 닫힐때 자동적으로 drop을 호출합니다.

## 이동 (Move)

```
let x = 5;
let y = x;
```

요거는 5를 x에 두고, x의 값의 복사본을 만들어 y에 넣고, 5라는 값들이 스택에 푸시되는것

String에서는 ?

```
let s1 = String::from("hello");
let s2 = s1;
```

우선 스트링은 3가지 영역으로 구성,

1. ptr(메모리 포인터)
2. len(길이)
3. capacity(용량)

이 영역은 스택에 저장이 된다.

그리고 내용물들은 힙 메모리에 저장이 된다. 이렇게 가리키는것

s1 -> "Hello"를 가리키는것이다.

s2 = s1일경우 스택에 저장되어있는 ptr,len, capacity를 복사하게 된다.

s1 -> "Hello"

s2 ↗

이렇게 가리킨다.

근데 이렇게 된다면 drop함수가 호출될때 힙 메모리를 두번 제거하게 되는데 러스트에서는 메모리 안정성을 보장하기 위해서

s1이 더이상 유효하지 않다고 간주하게 된다 즉

s1

s2 -> "Hello"

이렇게 되어 버리는것

### 클론

만약 스택 데이터와 힙 데이터를 깊게 복사하려면 clone을 쓰면 된다

```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

### 복사

컴파일 타임에 결정되어 있는 크기의 타입은 다 스택에 저장이 되서 바로 복사본이 만들어 질 수 있다.

### Copy 트레잇

특별한 어노테이션

- u32와 같은 모든 정수형 타입들
- true와 false값을 갖는 부울린 타입 bool
- f64와 같은 모든 부동 소수점 타입들
- Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 가능 (i32, String) x

### 소유권과 함수

```
fn main() {
    let s = String::from("hello");
     // s가 스코프 안으로 들어왔습니다.

    takes_ownership(s);
    // s의 값이 함수 안으로 이동해서
       더이상 유효하지 않습니다.

    let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
                                    // i32는 Copy가 되므로, x를 이후에 계속
                                    // 사용해도 됩니다.

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는 해제되었습니다.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어났습니다.
```

### 반환값과 스코프

```
n main() {
    let s1 = gives_ownership();         // gives_ownership은 반환값을 s1에게
                                        // 이동시킵니다.

    let s2 = String::from("hello");     // s2가 스코프 안에 들어왔습니다.

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back 안으로
                                        // 이동되었고, 이 함수가 반환값을 s3으로도                     이동시켰습니다.

} // 여기서 s3는 스코프 밖으로 벗어났으며 drop이 호출됩니다. s2는 스코프 밖으로 벗어났지만 이동되었으므로 아무 일도 일어나지 않습니다. s1은 스코프 밖으로 벗어나서 drop이 호출됩니다.

fn gives_ownership() -> String {             // gives_ownership 함수가 반환 값을 호출한 쪽으로 이동시킵니다.

    let some_string = String::from("hello"); // some_string이 스코프 안에 들어왔습니다.

    some_string                              // some_string이 반환되고, 호출한 쪽의
                                             // 함수로 이동됩니다.
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string  // a_string은 반환되고, 호출한 쪽의 함수로 이동됩니다.
}

```

변수의 소유권은 모든 순간 똑같은 패턴을 따름,
값들이 다른 변수에 의해 소우되도록 이동하지 않는한 drop에 의해 제거된다.






# 참조자와 빌림(References and Borrowing)

소유권을 넘기는 대신 개체에 대한 참조자를 인자로 사용하는 방법

```
fn main() {
    let s1 = String::from("hi");

    let len = cal_len(&s1);
}

fn cal_len(s : &String) -> uszie {
    s.len()
}

```

& 를 통해 값을 참조하는 참조자 생성,
참조자가 스코프 밖으로 가도 메모리 반납 x

기본적으로 참조자는 불변이지만

mut 을 가진 변수에 대해 
- &mut 참조자 가능
- 하지만 &mut 는 하나만 가질 수 있다

만약 mut 변수에 
```
let mut s1 = String::from("d");

let s2 = &s;
let s3 = &mut s // 이렇게 하면 에러가 난다

```
불변참조자가 가지고있을때 가변참조자를 하면 안댐



### slice

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

&str 값.. 스트링 리터럴도 슬라이스임

