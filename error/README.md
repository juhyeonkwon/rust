# Panic!
오류를 냅니담


# Result

```
match a {
    Ok() => "",
    Err(e) => "",
}
```

# unwrap, expect

unwrap은 오류면 panic!
expect는 메세지를 전달 가능


# ?

에러전파 숏컷
Result를 반환하는 함수에서만 사용가능

```
let muf f = File::open("asdasd")?;
-> 에러면 panic 
```


# panic!을 써도 좋은곳
## 예제, 프로토타입, 테스트


### unwrap
## Result 가 Ok를 가지고 있다고 확신 하지만, 컴파일러가 확신을 못할 수 있는 경우 

```
use std::net::IpAddr;

let home = "127.0.0.1".parse::<IpAddr>().unwrap();
```
이런경우에 쓰면 댐

