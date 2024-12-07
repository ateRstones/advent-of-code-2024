use std::fs;

fn test_report(report: &Vec<i32>) -> bool {
    let mut prev_report_elem: Option<&i32> = None;
    let mut prev_diff: Option<i32> = None;
    let mut is_safe: bool = true;

    for report_elem in report {
        if prev_report_elem.is_some() {
            let current_diff = report_elem - prev_report_elem.unwrap();

            if current_diff.abs() > 3 || current_diff.abs() < 1 {
                is_safe = false;
                break;
            }

            if prev_diff.is_some() && (prev_diff.unwrap() * current_diff) < 0 {
                is_safe = false;
                break;
            }

            prev_diff = Some(current_diff);
        }

        prev_report_elem = Some(report_elem);
    }
    
    return is_safe;
}

fn main() {
    let mut safe_reports = 0i32;
    let mut safe_compensated_reports = 0i32;

    let contents: String = fs::read_to_string("input").expect("Could not read data");

    for line in contents.lines() {

        let report: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        
        if test_report(&report) {
            safe_reports += 1;
        }

        for i in 0..report.len() {
            let mut one_removed: Vec<i32> = Vec::new();

            one_removed.extend_from_slice(&report[..i]);
            one_removed.extend_from_slice(&report[i+1..]);

            if test_report(&one_removed) {
                safe_compensated_reports += 1;
                break;
            }
        }
    }

    println!("Safe reports: {safe_reports}");
    println!("Safe compensated reports: {safe_compensated_reports}");
}
