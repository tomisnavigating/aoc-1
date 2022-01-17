use std::{io::{BufReader, BufRead}, fs::File};

struct Window {
    arr: Vec<i32>,
    head: usize,
    n_pushes: usize,
}

impl Window {
    pub fn new(size: usize) -> Window {
        Window { arr: vec![0; size], head: 0 , n_pushes: 0}
    }

    pub fn push(&mut self, x: i32) {
        self.arr[self.head] = x;

        self.head = (self.head + 1) % self.arr.len();

        self.n_pushes += 1;

    }

    pub fn push_and_check_for_increase(&mut self, x: i32) -> bool {
        let prior_sum = self.sum();
        self.push(x);

        if self.n_pushes < self.arr.len() + 1 { 
            println!("Not full yet. {} pushes so far", self.n_pushes);
            return false
        }

        self.sum() > prior_sum
    }

    pub fn sum(&mut self) -> i32 {
        let mut total: i32 = 0;
        for x in &self.arr {
            total = total + x;
        }
        total
    }
}


fn main() {
    
    
    let file_name = "src/input.txt";

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut n_increasing_depths: usize = 0;

    let mut window = Window::new(3);

    for line in reader.lines() {


        let depth_input = line.unwrap().parse::<i32>().unwrap();

        if window.push_and_check_for_increase(depth_input){
            n_increasing_depths += 1;
        }
    }
    println!("{}",  n_increasing_depths);

}
