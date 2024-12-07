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

fn generate_compensated_report(report: &Vec<i32>) -> Vec<i32> {
    let mut compensated_report: Vec<i32> = report.clone();

    println!("Previous {report:?}");

    for i in 1..report.len() {
        let current_diff = report[i] - report[i-1];

        if i >= 3 {
            let d = report[i];
            let c = report[i-1];
            let b = report[i-2];
            let a = report[i-3];
            let dmc = d - c;
            let dmb = d - b;
            let cmb = c - b;
            let cma = c - a;
            let bma = b - a;
            let abbc = cmb * bma;
            let bccd = dmc * cmb;
            let accd = dmc * cma;
            let abbd = dmb * bma;

            let mut check_order: [i32; 4] = [-3, -2, -1, 0];
            let mut check_scores: [i32; 4] = [bccd, abbd, accd, abbc];

            // do a dumb sort, can't be asked
            for _q in 0..check_order.len() {
                for g in 1..check_order.len() {
                    if check_scores[g] > check_scores[g-1] {
                        let temp = check_scores[g];
                        let temp_i = check_order[g];
                        check_scores[g] = check_scores[g-1];
                        check_scores[g-1] = temp;
                        check_order[g] = check_order[g-1];
                        check_order[g-1] = temp_i;
                    }
                }
            }

            // Problem detected
            if abbc < 0 || bccd < 0 {
                if check_scores[0] > 0 {
                    let offset = check_order[0];
                    println!("Removed {offset}");
                    compensated_report.remove(i-((-offset) as usize));
                }

                break;
            }
        }

        if current_diff == 0 { // fix 0 diff elements, but just once
            compensated_report.remove(i);
            break;
        }

        if current_diff > 3 {
            if i == 1 {
                compensated_report.remove(0);
                break;
            }

            else if i == report.len() - 1 {
                compensated_report.remove(i);
                break;
            }
        }
    }
    
    println!("Compensated {compensated_report:?}");

    return compensated_report;
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

        let compensated_report = generate_compensated_report(&report);

        if test_report(&compensated_report) {
            safe_compensated_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
    println!("Safe compensated reports: {safe_compensated_reports}");
}
