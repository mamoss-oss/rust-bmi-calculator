use std::io;
use std::io::Write;
use std::str::FromStr;

fn input<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut input: String = String::with_capacity(64);

    io::stdin()
        .read_line(&mut input)
        .expect("Input could not be read");

    input.trim().parse()
}

fn calc_bmi(weight: usize, height: usize) -> f32 {
    let height_m = height as f32 / 100.0;
    weight as f32 / (height_m * height_m)
}

fn main() {

    println!("BMI Calculator");
    println!("Provide your weight and height.");
    print!("Weight in kg: ");
    io::stdout().flush().unwrap();
    let weight: usize = input().unwrap();

    print!("Height in cm: ");
    io::stdout().flush().unwrap();
    let height: usize = input().unwrap();

    let result = calc_bmi(weight, height);
    println!("The calculated BMI is {result}")

}
