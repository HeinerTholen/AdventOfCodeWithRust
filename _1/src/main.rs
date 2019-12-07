mod input;


fn calc_fuel_req(mass: i32) -> i32 {
    let step = mass/3 - 2;

    if step < 1 {
        return 0;
    }

    step + calc_fuel_req(step)
}

fn main() {
    let mut sum = 0;
    let mut sum_recursive = 0;
    for mass in input::MODULE_MASSES.iter() {
        sum += mass/3 - 2;
        sum_recursive += calc_fuel_req(*mass);
    }
    println!("Sum: {}", sum);
    println!("Sum incl. fuel: {}", sum_recursive);
}
