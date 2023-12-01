fn main() {
    let numbers = vec![3, 6, 9, 12];
    let result: Vec<i32> = numbers.iter().map(|n| n * 10).collect();
    println!("{:?}", result);
}
