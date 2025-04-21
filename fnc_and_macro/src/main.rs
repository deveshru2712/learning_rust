fn main() {

    let x  = add(1,1);
    let y = add(3,0 );
    let z= add(x,1);

// marcos -> are like fnc but it expands into additional code

// they have ! for invocation

print!("hello");

// it is available to use here i this specific token
// token start and end with curly braces
// :? -> debug print

print!("{:?}",x);
print!("{:?}",y);
print!("{:?}",z);


// this is debug version
print!("{z:?}");

// while this is what is going to be displayed to the user

print!("{z:?}");




}


// here it is talking two parameter and returning a 32 bit integer

fn add(a:i32,b:i32) -> i32{
    a+b
}

