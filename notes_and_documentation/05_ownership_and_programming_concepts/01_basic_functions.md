#
# [Back](./../../README.md)

## Function Signatures
* Must declare the type of all params:
```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
* The `x` here needs to be `x: i32`

This is valid code actually:
```rs
fn main() {
    let x: i32 = 5;
    another_function(x);
    another_function(x);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```
So, you can call a function twice like this using the same variable.

## Expressions
* Must evaluate to value and do not end in semicolon (when on a line by themselves)

Valid
```rs
fn main() {
    let y = 6;
}
```

Invalid
```rs
fn main() {
    let x = (let y = 6);
}
```

Valid
```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    if y > 5 {
        test_another_function();
    }

    println!("The value of y is: {y}");
}
```
* The end of the statement with curly braces does not have a semi-colon, making this return a value of x+1 to y.
    * Which is 4 in this case.

Unpredictable
```rs
fn main() {
    let y = {
        let x = 3;
        x + 1;
    };

    println!("The value of y is: {y}");
}
```
* In the case above, this will lead to errors, or otherwise just not work the way you want.
    * Do **NOT** do this.
    * Don't put a semi-colon if you want to return a value from a multiline statement.
        * Or, more accurately, return an expression at the end of a multiline statement.


## Function Expressions
Valid
```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

Valid
```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```
* Of course, if a semicolon was placed at the end, this wouldn't be since we are returning an expression.

#
# [Back](./../../README.md)