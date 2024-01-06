// Consts can be global, must have a type definition, and must be set to a constant expression.
const Z_CONST: &str = "This is a global const";

fn main() {
    mutability();
    shadowing();
    const_print();
    operations();
    tuple();
    array();
}

fn mutability() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
}

fn shadowing() {
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }

    println!("The value of y is: {y}");
}

fn const_print() {
    println!("{Z_CONST}");
}

fn operations() {
    //addition
    let sum = 5 + 10;
    println!("{sum}");

    //substraction
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    let product = 4 * 30;
    println!("{product}");

    // divistion
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("{quotient}");
    println!("{truncated}");

    //remainder
    let remainder = 43 % 5;
    println!("{remainder}");
}

fn tuple() {
    // Type annotation in this example is optional
    let tup: (u32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("Values of x, y, and z are {x}, {y}, {z}");
    println!("Printing directly from the tuple, x is equal to {}", tup.0);
}

fn array() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 6];

    println!("Full print of array2: {arr2:#?}");
    println!("Print of element 4 of array1: {}", arr1[3]);
}
