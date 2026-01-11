fn main() {
    /*
        gives_ownership move seu retorno
        valor em s1
    */

    let s1 = gives_ownership();

    /*
        s2 entra no escopo;
        s2 é movido para;
        takes_and_gives_back que tambem move seu valor de retorno para s3;

    */

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("{s1}");
    println!("{s3}");
} //s3 sai do escopo entao é .drop
//s1 sai do escopo entao é .drop
//
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
