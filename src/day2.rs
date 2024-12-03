pub fn run1(input: String){
    
    let reports = read_input(input);
    
    // validate
    let mut valid_report_count = 0;
    for report in reports{
        if is_report_valid2(report.clone(), false) {
            valid_report_count += 1;
        } 
    }

    println!("The result for day2 task1 is {}", valid_report_count);
}

pub fn run2(input: String){
    let reports = read_input(input);
    
    // validate
    let mut valid_report_count = 0;
    for report in reports{
        if is_report_valid2(report.clone(), true) {
            valid_report_count += 1;
        } 
    }

    println!("The result for day2 task2 is {}", valid_report_count);
}

fn is_report_valid2 (report: Vec<i32>, error_dampner: bool) -> bool{
    let level_deltas = get_level_deltas(report.clone());
    let problem_per_level = evaluate_deltas(level_deltas);
    let total_problem: i32 = problem_per_level.clone().iter().sum();
    if total_problem == 0{
        return true;
    }

    if !error_dampner{
        return false;
    } 

    let mut index_with_most_problems: Vec<usize> = Vec::new();
    let mut highest_problem = 0;
    for (index, problem) in problem_per_level.iter().enumerate() {
        if problem > &highest_problem{
            highest_problem = *problem;
            index_with_most_problems.push(index);
        }

        if highest_problem == *problem{
            index_with_most_problems.push(index);
        }
    }

    for index in index_with_most_problems{
        let mut cloned_report = report.clone();
        cloned_report.remove(index);

        if is_report_valid2(cloned_report.clone(), false){
            return true;
        }
    }

    return false;
}

fn evaluate_deltas (deltas: Vec<i32>) ->  Vec<i32> {
    // the amount of problems the level at the index is involed
    let mut problems_per_level: Vec<i32> = Vec::new();
    let mut next_level_problem = 0;
    let direction: i32 = evaluate_direction(deltas.clone());
    for delta in deltas{
        let mut current_level_problem = next_level_problem;
        next_level_problem = 0;

        if delta == 0 || delta.abs() >= 4{
            current_level_problem += 1;
            next_level_problem += 1;
        }

        if direction * delta <= 0{
            current_level_problem += 1;
            next_level_problem += 1;
        }
        problems_per_level.push(current_level_problem);
    }

    problems_per_level.push(next_level_problem);
    return problems_per_level;
}

fn evaluate_direction(deltas: Vec<i32>) -> i32{
    let mut count_positive = 0;
    let mut count_negative = 0;
    for delta in deltas{
        if delta > 0 {
            count_positive += 1;
        } else if delta < 0{
            count_negative += 1;
        }
    }

    return count_positive - count_negative;
}

fn get_level_deltas(report: Vec<i32>) -> Vec<i32> {
    let mut last_level: i32 = 0;
    let mut level_deltas: Vec<i32> = Vec::new();
    for level in report{
        // first run
        if last_level == 0{
            last_level = level;
            continue;
        }

        let level_delta = level - last_level;
        level_deltas.push(level_delta);
        last_level = level;
    }
    return level_deltas;
}

fn read_input(input: String) -> Vec<Vec<i32>>{
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in input.split("\r\n") {
        let mut report: Vec<i32> = Vec::new();
        let numbers = line.split(" ").collect::<Vec<_>>();
        for number in numbers {
            report.push(number.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    return reports;
}