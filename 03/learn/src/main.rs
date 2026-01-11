use std::io;

fn mutable() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor de x no escopo interno é: {x}")
    }

    println!("O valor de x é: {x}");
}

fn tups() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup_value_one = tup.0;
    let tup_value_second = tup.1;
    let tup_value_third = tup.2;

    println!("O valores de tup sâo: {tup_value_one} {tup_value_second} {tup_value_third}");
}

fn arrays() {
    let a: [i32; 5] = [-1, 2, -3, 4, -5];

    println!("Por favor, insira um índice de matriz..");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falha ao ler a linha!");

    let index: usize = index
        .trim()
        .parse()
        .expect("O índice inserido não era um número.");

    let element = a[index];

    println!("O valor do elemento no índice {index} é: {element}");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn loop_fn() {
    let mut count = 0;

    'counting_up: loop {
        println!("Contador: {count}");
        let mut remaining = 10;

        loop {
            println!("Restante: {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Contador final: {count}");
}

fn while_fn() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("While loop value: {}", a[index]);
        index += 1;
    }
}

fn for_fn() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("For loop value: {element}")
    }
}

fn main() {
    mutable();
    tups();
    arrays();

    let test = sum(5, 5);
    println!("O resultado da soma é: {test}");

    loop_fn();
    while_fn();
    for_fn();
}
