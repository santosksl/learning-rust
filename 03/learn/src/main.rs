fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("O valor de x no escopo interno é: {x}")
    }

    println!("O valor de x é: {x}")
}
