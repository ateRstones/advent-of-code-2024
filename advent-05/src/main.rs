use std::fs;
use std::collections::HashMap;

fn check_job(rules: &Vec<(i32, i32)>, job: &Vec<i32>) -> i32 {
    let mut page_indices: HashMap<i32, i32> = HashMap::new();
    job.iter().enumerate().for_each(|(i,x)| {page_indices.insert(*x, i as i32);});

    for (before, after) in rules {
        if page_indices.contains_key(before) && page_indices.contains_key(after) {
            let i_before = page_indices.get(before).unwrap();
            let i_after = page_indices.get(after).unwrap();

            if i_before > i_after {
                println!("Breaking rule {before}|{after}");
                return 0;
            }
        }
    }

    return job[job.len() / 2];
}

fn order_job(rules: &Vec<(i32, i32)>, job: &Vec<i32>) -> Vec<i32> {
    let mut ordered_job: Vec<i32> = Vec::new();

    let mut relevant_rules: Vec<&(i32, i32)> = rules.iter().filter(|(b,a)| job.contains(a) && job.contains(b)).collect();

    let mut rules_graph_forward: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rules_graph_backward: HashMap<i32, Vec<i32>> = HashMap::new();
    relevant_rules.iter().map(|t| **t).for_each(|(a,b)| {
        if !rules_graph_forward.contains_key(&a) {
            rules_graph_forward.insert(a, Vec::new());
        }

        rules_graph_forward.get_mut(&a).unwrap().push(b);

        if !rules_graph_backward.contains_key(&b) {
            rules_graph_backward.insert(b, Vec::new());
        }

        rules_graph_backward.get_mut(&b).unwrap().push(a);
    });

    while rules_graph_backward.len() > 0 {
        let mut current_rule: (&i32, &Vec<i32>) = rules_graph_backward.iter().next().unwrap();

        while rules_graph_backward.contains_key(&current_rule.1[0]) {
            current_rule = (&current_rule.1[0], rules_graph_backward.get(&current_rule.1[0]).unwrap());
        }

        println!("Found rule {:?}", current_rule);

        let current_top_elem = current_rule.1[0];
        ordered_job.push(current_top_elem);

        let forward_rule = rules_graph_forward.get(&current_top_elem).unwrap();
        
        println!("Found forward rule {:?}", forward_rule);

        for target in forward_rule {
            let mut backward_rule = rules_graph_backward.get_mut(&target).unwrap();
            if let Some(index) = backward_rule.iter().position(|value| *value == current_top_elem) {
                backward_rule.swap_remove(index);
            }
            if backward_rule.len() == 0 {
                rules_graph_backward.remove(&target);
            }
        }
    }

    println!("{:?}", ordered_job);

    return ordered_job;
}

fn run_for_input(contents: &String) {
    let split_res: Vec<&str> = contents.split("\n\n").collect();
    let rules_input = split_res[0];
    let jobs_input = split_res[1];

    let mut rules: Vec<(i32, i32)> = Vec::new();

    for line in rules_input.lines() {
        let rule: Vec<i32> = line.split("|").map(|x| x.parse::<i32>().unwrap()).collect();
        rules.push((rule[0], rule[1]));
    }

    let jobs: Vec<Vec<i32>> = jobs_input.lines().map(|x| x.split(",").map(|x| x.parse::<i32>().unwrap()).collect()).collect();

    let score: i32 = jobs.iter().map(|j| check_job(&rules, &j)).sum();


    let unordered: Vec<&Vec<i32>> = jobs.iter().filter(|j| check_job(&rules, &j) == 0).collect();
    let ordered: Vec<Vec<i32>> = unordered.iter().map(|j| order_job(&rules, j)).collect();

    let ordered_score: i32 = ordered.iter().map(|j| check_job(&rules, &j)).sum();

    println!("Score {score}");
    println!("Ordered Score {ordered_score}");
}

fn main() {
    let contents: String = fs::read_to_string("input").expect("Could not read data");
    let _test_contents: String = String::from("47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47");
    run_for_input(&contents);
}
