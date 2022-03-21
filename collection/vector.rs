fn main() {

    //i32 값을 가질 수 있는 벡터값 생성.
    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
    
    let third: &i32 = &v3[2];
    let third2 : Option<&i32> = v3.get(2);

    let third3 : Option<&i32> = match v3.get(2) {
        Some(i) => Some(i),
        None => None,
    };

    match third3 {
        Some(i) => println!("{}", i),
        None => (),    
    }

    println!("{}", third);

    let mut v4 = vec![100, 101, 102];

    for i in &v4 {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("white")),
        Cell::Float(3.12),
    ];

    println!("{:?}", row);


    
}