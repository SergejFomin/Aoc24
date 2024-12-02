pub fn run1(input: String){
    
    let (left_list, right_list) = read_input_sorted(input);
    
    // calculate 
    let mut sum = 0;
    for (left_item, right_item) in left_list.iter().zip(right_list.iter()){
        sum += calculate_distance(left_item, right_item);
    }

    println!("The result for day1 task1 is {}", sum);
}

pub fn run2(input: String){

    let (left_list, right_list) = read_input_sorted(input);

    // calculate 
    let mut sum = 0;
    let mut right_iter = right_list.iter().peekable();
    for left_number in left_list {
        let mut occurences = 0;
        let left_number_reference = &&left_number;
        while right_iter.peek().unwrap() < left_number_reference {
            right_iter.next();
        }

        while right_iter.peek().unwrap() == left_number_reference {
            occurences += 1;
            right_iter.next();
        }

        sum += left_number * occurences;
    }

    println!("The result for day1 task2 is {}", sum);
}

fn read_input_sorted(input: String) -> (Vec<u32>, Vec<u32>){
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();
    for line in input.split("\r\n") {
        let numbers = line.split("   ").collect::<Vec<_>>();
        left_list.push(numbers[0].parse::<u32>().unwrap());
        right_list.push(numbers[1].parse::<u32>().unwrap());
    }

    // sort lists
    left_list.sort();
    right_list.sort();

    return (left_list, right_list);
}

fn calculate_distance(point_one: &u32, point_two: &u32) -> u32{
    if point_one < point_two{
        return point_two - point_one;
    }

    return point_one - point_two;
}