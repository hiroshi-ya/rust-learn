// use core::num;
use std::io::{self, Write};

// immutable and mutable
fn mutability() {
    let mut x = 5;
    println!("x is {}", x);

    x = 6;
    println!("x is {}", x);
}

// constant
fn constant() {
    let a = 3;

    const A_CONSTANT: u32 = 5;
    const ANOTHER_CONSTANT: u32 = A_CONSTANT * 2;

    println!("a is {a} and constant is {ANOTHER_CONSTANT}");
}

// shadowing
fn shadowing() {
    let x = 33;

    let x = x + 1;

    // a block
    {
        let x = x * 2;
        println!("x的值为：{x}");
    }

    println!("x的值为：{x}");

    print!("\n请输入一堆空格：");
    io::stdout().flush().unwrap();

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read");
    guess = guess.replace('\n', "").replace('\r', "");

    // guess = guess.len(); // this is illegal
    let guess = guess.len(); // this is legal

    println!("长度为：{guess}");
}

// data types
fn data_types() {
    // let guess = "42".parse().expect("not a number"); // this is illegal
    let guess: i32 = "42".parse().expect("not a number"); // this is legal
    println!("我是{{{guess}}}");

    let mut x: u8 = 22;
    // x = x + 255; // will panick in debug binary but won't in release binary
    println!("测试溢出：{{{x}}}");

    let x = 1.9 + 0.1;
    let y: f32 = 3.2;
    println!("x is {x} and y is {y}");
}

// some basic arithemtics
fn arithmetics() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为 0

    // 余数
    let reminder = (-43) % 5;
    let reminder1 = 43 % -5;
    println!(
        "
5 + 10 = {sum},
95.5 - 4.3 = {difference}
4 * 30 = {product}
56.7 / 32.2 = {quotient}
2 / 3 = {floored}
-43 % 5 = {reminder}
43 % -5 = {reminder1}"
    );
}

// boolean
fn boolean() {
    let mut t = true;

    let f: bool = false; // 带有显式类型注解

    println!("t is {t} and f is {f}");

    // t = 2; // this is illegal
}

// chars
fn chars() {
    let c = 'c';
    let z: char = 'ℤ'; // 带有显式的类型注解
    let heart_eyed_cat = '😻';
    let nerd: char = '';
    println!("c 为 {c}, z 为 {z}, 爱心猫: {heart_eyed_cat}, vscode: {nerd}");
}

// tuple
fn tuple() {
    let tup: (i32, f64, char) = (233, 9.9, '');
    let tup0 = tup.0; // necessary, because a tuple can contain different data types
    let tup1 = tup.1; // so the compiler cannot perform type inference
    let tup2 = tup.2;
    println!("Trying to print a tuple: ({tup0}, {tup1}, {tup2})");
}

// array
fn array() {
    // all elements in an array must share a same type
    let arr0 = [1, 2, 3, 4, 5]; // [i32, 5]
    let arr1 = ['a', 'b', 'c']; // [char, 3]
    let arr2 = [8.0, 1.9, 10.0]; // [f64, 3]
    // [9, 9.0] is invalid
    println!("The array is {arr1:?}"); // the ":?" is necessary for printing arrays
    // or use {var_name:#?} to print elements line by line

    let arr3: [i32; 5] = [6, 7, 8, 9, 10]; // we can explicitly declare the length of the array
    println!("A declared array with declared size: {arr3:?}");

    let arr4 = ['c'; 4]; // equivalent to `let arr4 = ['c', 'c', 'c', 'c'];`
    println!("Fill the declared array with same value: {arr4:?}");

    println!(
        "Trying to access the element. First: {}, last: {}",
        arr0[0],
        arr0[arr0.len() - 1]
    ); // `{arr[0]}` is invalid in format string
}

fn array_access() {
    let a = [1, 2, 3, 4, 5, 6];
    print!("Enter an index: ");
    io::stdout().flush().unwrap();

    // read an input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to readline");

    let index: usize = input.trim().parse().expect("not a number");

    let element = a[index];
    println!("The value of a[{index}] is {element}");
}

fn main() {
    array_access();
}
