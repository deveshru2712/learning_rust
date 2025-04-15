fn main() {

    // conditionals

    let _is_even = false;

    // if the condition is not that complex you can skip the bracket for condition
   
    // if is_even {
    //     println!("The number is even");
    // }else if !is_even{
    //     println!("The number is odd");        
    // }

    // loops

    // loop will run till 99 

    for _i in 0..1000000{
        // println!("{}",i)
    }


    // iterating over an array, maps, strings

    let sentence = String::from("my name is yash chandra");

    let first_word = get_first_word(sentence);

    println!("First word is: {}",first_word);


}


// fn should also have a return type

fn get_first_word(sentence:String)->String {
    
    let mut  ans = String::from("");
    

    // chars is basically an iterator

    for char in sentence.chars()  {
        ans.push_str(char.to_string().as_str());
    
    // suppose it encounter an space

        if char == ' ' {
            break;
        }  
    }
    return  ans;

}