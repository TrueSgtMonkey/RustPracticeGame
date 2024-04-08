#
# [Back](./../../README.md)

## If Statements
* Expressions
* Can be assigned to a variable:
```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

```rs
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```
* This would be invalids since the i32 type is not a boolean.
* Can only do if statements in this style if there is a boolean there.

You have to instead run it like this:
```rs
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
```
* *Explicitly* write `number != 0` to have the C equivalent.

## Loops
* You can return values from a break statement when the loop ends:
```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

## While loops
* They do exist.

You can loop through arrays normally:
```rs
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

Or, you can use an iterator:
```rs
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

For static numbers, you can also use a range generator:
```rs
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

To show off a bunch of the stuff learned here, here is a temperature converter:
```rs
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
    println!("Farenheight Celsius\n{}        {}", farenheight, temperature);
}

fn celsius_to_farenheight(temperature: &f64) {
    let celsius = *temperature;
    let temperature = (*temperature * (9.0f64 / 5.0f64)) + 32.0f64;
    println!("Farenheight Celsius\n{}        {}", temperature, celsius);
}

fn get_input(message: &str, choice: &mut String) {
    print!("{}", message);

    // need to flush to print on same line as input?
    let _flush = std::io::stdout().flush();
    std::io::stdin()
        .read_line(choice)
        .expect("Failed to read line!");
}
```

#
# [Back](./../../README.md)