// Consts can be global, must have a type definition, and must be set to a constant expression.
const Z_CONST: &str = "This is a global const";

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("the value of x is {x}");

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is: {y}");
    println!("{Z_CONST}");
}
