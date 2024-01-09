/* comment example */
fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');
    expression();
    return_value();
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(v: i32) -> i32 {
    return v + 1; //semi-colon in this case is optional, since v+1 is our expression
}

fn return_value() {
    let z = five();

    println!("The value of z is: {z}");

    let v = plus_one(z);

    println!("The value of v is: {v}");
}
