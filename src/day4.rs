pub mod run1{
    use crate::day4::helper::read_input;
    use crate::day4::word_search::{WordSearch, EvaluationResult};
    use crate::day4::point::Point;
    use crate::day4::matrix::Matrix;

    pub fn run(input: String){

        let mut occurrences = 0;
        let matrix = Matrix::new(read_input(input));
        let word_search = WordSearch::new("XMAS".to_owned());
        let direction_transformation_points: Vec<Point> = vec![
            Point::new(-1, -1),
            Point::new(0, -1),
            Point::new(1, -1),
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(-1, 1),
            Point::new(0, 1),
            Point::new(1, 1),
        ];

        for (y, line) in matrix.matrix.iter().enumerate(){
            for (x, _) in line.iter().enumerate(){
                occurrences += search_matrix_from_point(&matrix, &Point::new(x as i32, y as i32), &direction_transformation_points, &word_search);
            }
        }

        println!("The result for day4 task1 is {}", occurrences);
    }

    fn search_matrix_from_point(matrix: &Matrix<char>, origin: &Point, possible_direction_transformation_points: &Vec<Point>, word_search: &WordSearch) -> u32{
        let mut origin_word_search = word_search.clone();
        if origin_word_search.evaluate(matrix.get(origin)) == EvaluationResult::WRONG{
            return 0;
        }

        let mut successful_searches = 0;
        for direction_transformation_point in possible_direction_transformation_points {
            let directed_word_search = origin_word_search.clone();
            if search_matrix_in_direction(matrix, origin, &direction_transformation_point, directed_word_search){
                successful_searches += 1;
            }
        }

        return successful_searches;
    }

    fn search_matrix_in_direction(matrix: &Matrix<char>, origin: &Point, direction: &Point, mut word_search: WordSearch) -> bool{
        let mut next_point = origin + direction;
        loop {
            if !matrix.is_point_in_matrix(&next_point){
                return false;
            }

            let char = matrix.get(&next_point);
            match word_search.evaluate(char){
                EvaluationResult::WRONG => return false,
                EvaluationResult::CORRECT => {}
                EvaluationResult::COMPLETE => return true,
            }

            next_point = &next_point + direction;
        }
    }
}

pub mod run2{
    use crate::day4::helper::read_input;
    use crate::day4::matrix::Matrix;
    use crate::day4::point::Point;

    pub fn run(input: String){
        let mut occurrences = 0;
        let matrix = Matrix::new(read_input(input));
        let direction_transformation_points: Vec<Vec<Point>> = vec![
            vec![
                Point::new(-1, -1),
                Point::new(1, 1)
            ],
            vec![
                Point::new(1, -1),
                Point::new(-1, 1),
            ],
        ];

        for (y, line) in matrix.matrix.iter().enumerate(){
            for (x, _) in line.iter().enumerate(){
                if search_matrix_for_point(&matrix, &Point::new(x as i32, y as i32), &direction_transformation_points){
                    occurrences += 1
                }
            }
        }

        println!("The result for day4 task2 is {}", occurrences);
    }

    fn search_matrix_for_point(matrix: &Matrix<char>, origin: &Point, possible_direction_transformation_points: &Vec<Vec<Point>>) -> bool{
        if matrix.get(origin) != 'A'{
            return false;
        }
        
        for direction_transformation_points in possible_direction_transformation_points {
            let first_point = origin + &direction_transformation_points[0];
            let opposite_point = origin + &direction_transformation_points[1];
            if !(matrix.is_point_in_matrix(&first_point) && matrix.is_point_in_matrix(&opposite_point)){
                return false;
            }

            let first_char = matrix.get(&first_point);
            let opposite_char = matrix.get(&opposite_point);
            let control_char = get_opposite_letter(first_char);
            if control_char.is_none() || control_char.unwrap() != opposite_char{
                return false;
            }
        }

        return true;
    }

    fn get_opposite_letter(character: char) -> Option<char>{
        match character {
            'M' => Some('S'),
            'S' => Some('M'),
            _ => None,
        }
    }
}

pub mod matrix{
    use crate::day4::point::Point;

    pub(crate) struct Matrix<T>{
        pub(crate) matrix: Vec<Vec<T>>,
        x_bound: i32,
        y_bound: i32,
    }

    impl Matrix<char>{
        pub(crate) fn new (vectors: Vec<Vec<char>>) -> Self{
            Self { matrix: vectors.clone(), y_bound: vectors[0].len() as i32 - 1, x_bound: vectors.len() as i32 -1 }
        }

        pub(crate) fn is_point_in_matrix(&self, point: &Point) -> bool {
            return point.x >= 0 && point.y >= 0 && point.x <= self.x_bound && point.y <= self.y_bound
        }

        pub(crate) fn get(&self, point: &Point) -> char {
            return self.matrix[point.y as usize][point.x as usize];
        }
    }
}

pub mod point{
    use std::fmt::Display;
    use std::ops::Add;

    pub(crate) struct Point {
        pub(crate) x: i32,
        pub(crate) y: i32,
    }

    impl Point {
        pub(crate) fn new (x: i32, y: i32) -> Self{
            Self { x, y }
        }
    }

    impl Clone for Point {
        fn clone(&self) -> Self {
            Point {
                x: self.x,
                y: self.y,
            }
        }
    }

    impl Add for &Point{
        type Output = Point;

        fn add(self, other: &Point) -> Point {
            return Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Display for &Point{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
            write!(f, "{}|{}", self.x, self.y)
        }
    }
}

pub mod word_search{
    pub struct WordSearch{
        current_string_index: usize,
        operation_string_chars: Vec<char>,
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum EvaluationResult{
        COMPLETE,
        CORRECT,
        WRONG
    }

    impl WordSearch {
        pub(crate) fn new (operation_string: String) -> Self{
            Self { current_string_index: 0, operation_string_chars: operation_string.chars().collect() }
        }

        pub(crate) fn evaluate (&mut self, character: char) -> EvaluationResult{
            if character != self.operation_string_chars[self.current_string_index]{
                return EvaluationResult::WRONG;
            }

            self.current_string_index += 1;

            if self.current_string_index == self.operation_string_chars.len(){
                self.reset();
                return EvaluationResult::COMPLETE;
            }

            return EvaluationResult::CORRECT;
        }

        fn reset (&mut self){
            self.current_string_index = 0;
        }
    }

    impl Clone for WordSearch {
        fn clone(&self) -> Self {
            WordSearch {
                current_string_index: self.current_string_index,
                operation_string_chars: self.operation_string_chars.clone(),
            }
        }
    }
}

pub mod helper {
    pub fn read_input(input: String) -> Vec<Vec<char>>{
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in input.split("\r\n") {
            let vector: Vec<char> = line.chars().collect();
            matrix.push(vector);
        }

        return matrix;
    }
}