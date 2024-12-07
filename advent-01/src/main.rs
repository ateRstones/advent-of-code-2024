use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Starting");
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let contents: String = fs::read_to_string("input").expect("Could not read data");

    for line in contents.lines() {
        let mut i = 0;
        for elem in line.split("   ") {
            if i == 0 {
                list1.push(elem.parse::<i32>().unwrap());
            } else if i == 1 {
                list2.push(elem.parse::<i32>().unwrap());
            }
            i += 1;
        }
    }

    list1.sort();
    list2.sort();

    let sum_dist: i32 = list1.iter().zip(list2.clone()).map(|(x,y)| (x-y).abs()).sum();

    println!("Result: {sum_dist}");

    let mut element_buckets: HashMap<i32, i32> = HashMap::new();

    for location_id in list2 {
        if !element_buckets.contains_key(&location_id) {
            element_buckets.insert(location_id, 0);
        }

        element_buckets.insert(location_id, element_buckets.get(&location_id).unwrap() + 1);
    }

    let sum_dist2: i32 = list1.iter().filter(|x| element_buckets.contains_key(x))
        .map(|x| element_buckets.get(x).unwrap() * x).sum();

    println!("Result 2: {sum_dist2}");
}
