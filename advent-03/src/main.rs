use std::fs;
use regex::Regex;

fn match_and_calulate(contents: &String) -> i64 {
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    let res: i64 = re.captures_iter(contents).map(|caps| {
        return caps.name("a").unwrap().as_str().parse::<i64>().unwrap()
            * caps.name("b").unwrap().as_str().parse::<i64>().unwrap()
    }).sum();
    return res;
}

fn main() {
    let contents: String = fs::read_to_string("input").expect("Could not read data");

    let res = match_and_calulate(&contents);

    println!("Result {res}");

    let mut filtered_contents: String = "".to_string();

    for content in contents.split("do()") {
        let mut split_sec = content.split("don't()");
        filtered_contents += split_sec.next().unwrap_or("");
    }

    let res2 = match_and_calulate(&filtered_contents);

    println!("Filtered: {res2}");
}
