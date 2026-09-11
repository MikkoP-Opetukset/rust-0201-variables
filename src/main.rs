// You can ignore the next line. It suppressess warnings about unreachable
// code, unused variables, and unused mutable keywords.
#![allow(dead_code, unused_variables, unused_mut)]

use std::io;

/// This is the entry point to the application.
fn main() {
    variables();
    scalar_types();
    compound_types();
}

fn variables() {
    println!("\n{:=>80}", ""); // Fills 80 spaces with '='
    println!("variables()\n");

    // Variables are immutable by default.
    let x = 6;
    // x = 6; // Uncomment this line to see the error.

    // You can declare variables mutable with the `mut` keyword.
    let mut y = 7;
    println!("Value of y: {y}");
    y = 12;
    println!("Value of y: {y}");

    // You can declare constants with the `const` keyword.
    // Constants cannot be mutable and the value cannot be computed at runtime.
    // So for example constants cannot be the result of a function unless the
    // function is also constant.
    const MEANING_OF_LIFE: i32 = 20 * 2 + 2;
    // const TWO: i32 = add_one(1); // Uncomment this to see the error.

    // Rust allows shadowing of variables. Earlier we declared x and here we
    // declare a new variable x with the same name. From now on the new value
    // of x will be used.
    let x = x + 1;

    // Shadowing is different to mutability, because we assign a NEW variable.
    // The new variable just has the same name as the previous one. This lets
    // us for example change the type of the variable.
    let x = x.to_string();

    // Mutating the type of an existing variable is not possible.
    // y = y as f32; // Uncomment to see the error.
}

fn type_annotations() {
    // As seen in the `variables()` function, often you don't need to annotate
    // the type of a variable, but Rust can determine the type automatically.
    // Sometimes you need to annotate the type though. For example when
    // declaring constants, or when Rust can not determine the type
    // automatically at compile time.

    // Without type annotation there is no way for Rust to know what types of
    // elements do you want to save to the Vector, since when creating it we
    // don't add any elements in there.
    // Try removing the type annotation from the following line to see the error.
    let items: Vec<i32> = Vec::new();

    // In this example we create the vector from a list of elements so Rust can
    // determine the type of the elements at compile time. No type annotation
    // needed.
    let items = Vec::from([1, 2, 3]);
}

fn scalar_types() {
    println!("\n{:=>80}", "");
    println!("scalar_types()\n");

    // Integers
    // Usually you can just default to using i32, unless there is a specific
    // reason to use some other size integer. There are 8, 16, 32, 64, and
    // 128-bit integers. All of them have unsigned (u) and signed (i) variants.

    // The `mut` keyword here is for a later example.
    let mut a: u8 = 253; // 8-bit unsigned integer. Min: 0; Max: 255.

    // _ can be used as a thousands separator.
    let b: i32 = 2_147_483_647; // 32-bit signed integer. Min: -(2^32-1); Max: (2^32-1)-1

    // Special integer type.
    // Size varies based on architecture, on 64-bit architecture the size is
    // 64-bit and on 32-bit architecture the size is 32-bit. These are commonly
    // used when storing sizes of variables or counts of elements in a
    // collection. Also has signed version isize.
    let c: usize = 42;

    // The size of an integer does matter.
    // Uncomment the following code and try what happens when you compile the
    // code in debug mode, and then try compiling in release mode. See the difference?

    // for i in 1..7 {
    //     println!("{a}");
    //     a += 1;
    // }

    // Floating points
    // Unless there is a reason to use f32, just default to using f64. It has
    // better precision.
    let d: f32 = 42.0;
    let e: f64 = 3.14;

    // Booleans
    let f: bool = true;
    let g = true;

    // Characters
    // Char type is surrounded by single quotes. A char can be any single
    // unicode scalar value. Commonly this means that a char type variable can
    // represent any single character. But unfortunately that is not always the
    // case when it comes to complex characters like emojis.
    let h: char = 'A'; // Simple.
    let i = '👍'; // Emojis are mostly fine.
    // Except not always. Some emojis are actually multiple unicode values
    // combined. For example most emojis that are related to a job or contain a
    // skin tone.
    // let j = '👮‍♀️'; // Try uncommenting this.
    // Your IDE should warn you when a "character" does not fit into a single
    // char type variable.
}

fn compound_types() {
    println!("\n{:=>80}", "");
    println!("compound_types()\n");

    // Tuple
    // Can be used to group together different types of values.
    let person1: (u32, &str, i8) = (1234, "Paul Simon", 85);
    let person2 = (4321, "Art Garfunkel", 84); // Note that Rust defaults to i32.

    // Accessing the elements via the index:
    let id = person1.0;
    let name = person1.1;
    let age = person1.2;
    // Side-note: here is one style of how to format prints.
    println!("ID: {id}, Name: {name}, Age: {age}");

    // A tuple can be destructured into separate variables. This uses pattern matching.
    let (id, name, age) = person2;
    println!("ID: {id}, Name: {name}, Age: {age}\n");

    // Array
    // Array is a compound type that can only hold one type of values. Arrays
    // are of fixed length and the lenght must be known at compile time. In the
    // type annotation first comes the type and then the size.
    let arr1: [i32; 3] = [12, 42, 7];
    let arr2 = [321, 123, 987];

    // Can be initialized with default values.
    let arr3 = [0; 20]; // First the default value, then size.
    // Side-note: another way of formatting prints. Try to move the arr1[0]
    // inside the curly brackets and see what happens. (It won't work.)
    println!("Accessing first value in arr1: {}\n", arr1[0]);

    // Trying to access elements out of bounds.
    // Rust will check the index you try to access at compile time and will
    // give an error if you try to access an element out of bounds.
    // Uncomment the next line to see the error.
    // println!("Accessing a value out of bounds: {}\n", arr1[3]);

    // Rust will also perform the check at runtime if it is not possible to
    // determine the result at compile time.
    let i = get_index();
    println!("Accessing the index {i} in arr1: {}", arr1[i]);
}

/// This function is used as an example in the `variables()` function.
fn add_two(x: i32) -> i32 {
    x + 2
}

// ========================================================================= //
// NOTE! After this point the functions are just helpers to make the above code
// examples slightly cleaner. You can check them out but they are not the focus
// of this material.
// ========================================================================= //

fn get_index() -> usize {
    println!("Try to access an element in bounds and out of bounds.");
    println!("Enter a number: ");
    let mut buffer = String::new();

    // Reading the input and saving it in buffer.
    io::stdin()
        .read_line(&mut buffer)
        .expect("Reading line failed."); // Error message in case of error.

    // Trimming and parsing the input into a number.
    let index: usize = buffer.trim().parse().expect("Input was not valid.");
    index
}
