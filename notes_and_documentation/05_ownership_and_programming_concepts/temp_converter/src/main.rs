use std::io::Write; // need for flushing

fn main() {
    let mut choice = String::new();
    let mut temperature = String::new();
    get_input("Temperature = ", &mut temperature);
    get_input(" 1) Farenheight\n 2) Celsius\n 0) Exit\nChoice: ", &mut choice);

    let choice : u8 = choice.trim().parse().expect("Input must be a number!");
    let temperature : f64 = temperature.trim().parse().expect("Input must be a decimal!");

    match choice {
        1 => farenheight_to_celcius(&temperature),
        2 => celsius_to_farenheight(&temperature),
        _ => return,
    }
}

fn farenheight_to_celcius(temperature: &f64) {
    let farenheight = *temperature;
    let temperature = (*temperature - 32.0f64) * (5.0f64 / 9.0f64);
    println!("Farenheight: {}\nCelsius: {}", farenheight, temperature);
}

fn celsius_to_farenheight(temperature: &f64) {
    let celsius = *temperature;
    let temperature = (*temperature * (9.0f64 / 5.0f64)) + 32.0f64;
    println!("Farenheight: {}\nCelsius: {}", temperature, celsius);
}

fn get_input(message: &str, choice: &mut String) {
    print!("{}", message);

    // need to flush to print on same line as input?
    let _flush = std::io::stdout().flush();
    std::io::stdin()
        .read_line(choice)
        .expect("Failed to read line!");
}