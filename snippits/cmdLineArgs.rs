use std::env;

fn cmdLineArgs(problem: &i32, input: &str)  {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        problem = &args[1].parse::<i32>().unwrap();
        input = &args[2];
    } else {
        println!("{} <problem:1|2> <inputPath>", &args[0]);
        std::process::exit(0)
    }   
}


fn main() {
    let mut problem = 1;
    let mut input = "";

    let res = cmdLineArgs(&problem, &input);
}
