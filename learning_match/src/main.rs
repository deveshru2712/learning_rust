fn main() {

    // match is similar to if else
    // it is exhaustive in nature
    // that is every option should be accounted for
    // _ -> stand for all the rest possibility
    let some_bol = true;
match some_bol {
    true=>println!("it is true"),
    false=>println!("it is false"),
}

let some_int = 3;
match some_int{
    1 => println!("It is one"),
    2=> println!("It is two"),
    3 =>println!("It is three"),
    _=>println!("anything else"),
}
}
