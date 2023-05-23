//////////////////////////////////////////////
///                                         //
///     This file is to learn the basics    //
///     of encrypting and decrypting        //
///     using Rust programming language     //
///     -   Matt Toal                       //
///                                         //
//////////////////////////////////////////////

use std::io;

fn main() {

    // Prompt asking for input
    println!("Please enter a password (with no special characters): ");

    // Input from user
    let mut password = String::new();                   // Password input
    io::stdin().read_line(&mut password).expect("Failed to read");


    

    println!("Choose an option: \n 1. Encryption \n 2. Decryption\n");
    let choice: i32;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read");

        choice = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid input, please try again: ");
                continue;
            }
        };
        break;
    }

    // Split password string into character vector
    let arr = split(password);

    match choice {
        1 => {
            println!("ENCRYPTION");

            // Call encryption function to encrypt password
            let en = encryption(arr);
            println!("Your encrypted password is: {}\n", en);
        }
        2 => {
            println!("DECRYPTION");
            let de = decryption(arr);
            println!("Your decrypted password is: {}\n", de);
        }
        _ => {
            println!("Invalid option, please try again: ");
        }
    }



}


// FUNCTIONS /////////////////////////////////////////////////
// FUNCTIONS /////////////////////////////////////////////////
// FUNCTIONS /////////////////////////////////////////////////

    // Desc: Turns input password into a vector so it can be used in for loops
fn split(s:String) -> Vec<char> {
    let mut new = vec![];
    for i in s.chars() {
        if i == '\n' {
            continue;
        }
        else {
            new.push(i);
        }
    }
    return new;
}


    // Desc: Returns a string of the encrypted password
fn encryption(arr:Vec<char>) -> String {

    // All characters able to be used in 'password'
    let nums = ['0','1','2','3','4','5','6','7','8','9'];
    let lower_case = ['a','b','c','d','e','f','g','h',
        'i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let upper_case = ['A','B','C','D','E','F','G','H',
        'I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let special = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '?'];


    let mut x = String::from("");       // string that will be returned
    
    for target in arr {
        if nums.contains(&target) {
            x.push(find_nums(target, nums));
        }
        else if lower_case.contains(&target) {
            x.push(find_lower(target, lower_case));
        }
        else if upper_case.contains(&target) {
            x.push(find_upper(target, upper_case));
        }
        else if special.contains(&target){
            x.push(find_special(target, special));
        }
    }
    return x;
}





fn decryption(arr:Vec<char>) -> String {
        // All characters able to be used in 'password'
        let nums = ['0','1','2','3','4','5','6','7','8','9'];
        let lower_case = ['a','b','c','d','e','f','g','h',
            'i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
        let upper_case = ['A','B','C','D','E','F','G','H',
            'I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
        let special = ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '?'];

        let mut x = String::from("");

        for target in arr {
            if nums.contains(&target) {
                x.push(dec_nums(target, nums));
            }
            else if lower_case.contains(&target) {
                x.push(dec_lower(target, lower_case));
            }
            else if upper_case.contains(&target) {
                x.push(dec_upper(target, upper_case));
            }
            else if special.contains(&target) {
                x.push(dec_special(target, special));
            }
        }
        return x;
}



    // Function pulls all characters of 'password' one index backwards in the respecting arrays (decryption)
fn dec_nums(c:char, arr:[char; 10]) -> char {
    let mut i: usize = 0;
    while c != arr[i] {
        i += 1;
    }
    if i == 0{
        return arr[arr.len()-1];
    }
    return arr[i-1];
}

fn dec_lower(c:char, arr:[char; 26]) -> char {
    let mut i: usize = 0;
    while c != arr[i] {
        i += 1;
    }
    if i == 0{
        return arr[arr.len()-1];
    }
    return arr[i-1];
}

fn dec_upper(c:char, arr:[char; 26]) -> char {
    let mut i: usize = 0;
    while c != arr[i] {
        i += 1;
    }
    if i == 0{
        return arr[arr.len()-1];
    }
    return arr[i-1];
}

fn dec_special(c:char, arr:[char; 11]) -> char {
    let mut i: usize = 0;
    while c != arr[i] {
        i += 1;
    }
    if i == 0{
        return arr[arr.len()-1];
    }
    return arr[i-1];
}





    // Function pushes all characters of 'password' one index forward in respecting arrays (encryption)
fn find_nums(c:char, arr:[char; 10]) -> char {
    let mut i = 0;
    while c != arr[i] {
        i += 1;
    }
    if i+1 == arr.len(){
        return arr[0];
    }
    return arr[i+1];
}

    // Function pushes all characters of 'password' one index forward in respecting arrays
fn find_lower(c:char, arr:[char; 26]) -> char {
    let mut i = 0;
    while c != arr[i] {
        i += 1;
    }
    if i+1 == arr.len(){
        return arr[0];
    }
    return arr[i+1];
}

    // Function pushes all characters of 'password' one index forward in respecting arrays
fn find_upper(c:char, arr:[char; 26]) -> char {
    let mut i = 0;
    while c != arr[i] {
        i += 1;
    }
    if i+1 == arr.len(){
        return arr[0];
    }
    return arr[i+1];
}

    // Function pushes all characters of 'password' one index forward in respecting arrays
fn find_special(c:char, arr:[char; 11]) -> char {
    let mut i = 0;
    while c != arr[i] {
        i += 1;
    }
    if i+1 == arr.len(){
        return arr[0];
    }
    return arr[i+1];
}