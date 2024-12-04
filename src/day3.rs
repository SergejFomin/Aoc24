pub fn run1(input: String){

    let mut multiplication_operation = MultiplicationOperation::new();
    
    let mut sum = 0;
    for char in input.chars(){
        // println!("comparing {} : {}", char, operation_string_chars[current_string_index]);
        if multiplication_operation.evaluate(char){
            sum += multiplication_operation.result();
            multiplication_operation.reset();
        }
    }

    println!("The result for day3 task1 is {}", sum);
}

pub fn run2(input: String){
    
    let mut multiplication_operation = MultiplicationOperation::new();
    let mut do_multiply_operation = SimpleOperation::new("do()".to_owned());
    let mut dont_multiply_operation = SimpleOperation::new("don't()".to_owned());
    let mut allow_multiplication = true;


    let mut sum = 0;
    for char in input.chars(){
        // println!("comparing {} : {}", char, operation_string_chars[current_string_index]);

        if do_multiply_operation.evaluate(char){
            allow_multiplication = true;
        }

        if allow_multiplication && multiplication_operation.evaluate(char){
            sum += multiplication_operation.result();
            multiplication_operation.reset();
        }

        if dont_multiply_operation.evaluate(char){
            allow_multiplication = false;
        }
    }

    println!("The result for day3 task2 is {}", sum);
}

struct SimpleOperation{
    current_string_index: usize,
    operation_string_chars: Vec<char>,
}

impl SimpleOperation {
    fn new (operation_string: String) -> Self{
        Self { current_string_index: 0, operation_string_chars: operation_string.chars().collect() }
    }

    // evaluates a character and return true if all requirements for the operation are met.
    // autoresets the state of the SimpleOperation once true is returned.
    fn evaluate (&mut self, character: char) -> bool{
        if character == self.operation_string_chars[self.current_string_index]{
            self.current_string_index += 1;
        } else {
            self.reset();
        }

        if self.current_string_index == self.operation_string_chars.len(){
            self.reset();
            return true;
        }

        return false;
    }

    fn reset (&mut self){
        self.current_string_index = 0;
    }
}

struct MultiplicationOperation {
    current_string_index: usize,
    operation_string_chars: Vec<char>,
    factor_one_string: Vec<char>,
    factor_two_string: Vec<char>
}

impl MultiplicationOperation {
    fn new () -> Self{
        Self { current_string_index: 0, operation_string_chars: "mul(,)".chars().collect(), factor_one_string: Vec::new(), factor_two_string: Vec::new() }
    }

    // evaluates a character and return true if all requirements are met to calculate and return a result.
    // autoresets the state of the MultiplicationOperation order of the characters does not add up.
    fn evaluate (&mut self, character: char) -> bool{
        if character.is_numeric() || character == self.operation_string_chars[self.current_string_index]{

            if character.is_numeric(){
                if self.current_string_index == 4 && self.factor_one_string.len() < 3{
                    self.factor_one_string.push(character);
                } else if self.current_string_index == 5 && self.factor_two_string.len() < 3{
                    self.factor_two_string.push(character);
                } else {
                    self.reset();
                }
            } else {
                self.current_string_index += 1;
            }
        } else {
            self.reset();
        }

        if self.current_string_index == 6{
            return true;
        }

        return false;
    }

    fn result (&self) -> u32{
        let factor_one = String::from_iter(self.factor_one_string.clone()).parse::<u32>().unwrap();
        let factor_two = String::from_iter(self.factor_two_string.clone()).parse::<u32>().unwrap();
        return factor_one * factor_two;
    }

    fn reset (&mut self){
        self.current_string_index = 0;
        self.factor_one_string.clear();
        self.factor_two_string.clear();
    }
}