fn main() {

    // basic variable

    // number

    let x= 1;

    let y =1000;

    let z:f32 = 100000.08;

    // printing out the variable
    
    // println!("x:{}",x);
    // println!("y:{}",y);
    // println!("z:{}",z);

    // boolean

    let is_male= true;
    let is_above18 = false;

    // if is_male{
    //     println!("You are male")
    // }else {
    //     println!("you are female")
    // }

    // if is_male && is_above18{
    //     println!("You are ready to go")
    // }


    // string

    let ax = "yash";

    // this is the most simple way of init a string in rust

    let name:String = String::from("Yash chandra");
    
    println!("{}",name);

    // it is giving me error as it can be undefined also

    // println!("{}",name.chars().nth(4));


    // here is how you can by pass it

    let char1= name.chars().nth(5);

    match char1 {
        Some(c) =>println!("{}",c),
        None => println!("No character at the index 5")
        
    }



}
