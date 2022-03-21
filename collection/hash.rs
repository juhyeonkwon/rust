use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("White"), 50);

    let value = scores.get("Blue");

    match value {
        Some(&v) => println!("{}", v),
        None => (),
    }




    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    //_ 밑줄은 키와 값 타입에 대한 타입 파라미터에 대해서 사용할 수 있음, 벡터에 담긴 데이터 타입에 기초해서
    //해쉬에 담길 타입을 추론한다.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //entry 함수, 키에 값이 없을때만 할당
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}", scores);
    
    //
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}