use std::collections::HashMap;

fn main() {

    let a = [5, 4, 3, 7, 2, 1, 3, 4 ,5 ,6, 10, 2, 3, 4, 5, 7, 1, 2, 1, 3, 4, 5, 6];

    let mut max : i32 = a[0];
    let mut min : i32 = a[0];

    let mut map = HashMap::new();

    for &i in a.iter() {
        let count = map.entry(String::from(i.to_string())).or_insert(0);
        *count += 1;

        if i > max {
            max = i;
        }

        if i < min {
            min = i
        }
    }

    println!("max : {}, min : {}", max, min);
        

}