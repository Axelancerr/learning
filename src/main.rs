mod median_and_mode;

fn main() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!("Mode: {:?}", median_and_mode::median(&numbers));
    numbers.push(7);
    println!("Mode: {:?}", median_and_mode::median(&numbers));
    numbers.push(7);
    println!("Mode: {:?}", median_and_mode::mode(&numbers));
}
