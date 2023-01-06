#![allow(dead_code)]

use std::process::Command;
use std::fs::File; // STRUCT FOR THE FILE
use std::io::prelude::*; // TO READ AND WRITE TO A FILE
use std::fs::OpenOptions;
use std::io;
use std::collections::HashMap;
use chrono::{DateTime, Local};
extern crate rand;
use rand::Rng;
extern crate regex;
use regex::Regex;
extern crate reqwest;
use std::env;
extern crate serde_json;
use serde_json::Value as JsonValue;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize,Deserialize)]
struct PersonExample{
    name: String,
    age: u8,
    is_male: bool 
}

//use error_chain::error_chain; 
use std::io::Read;

#[cfg(test)]
mod learn_rust{
    #[test]
    #[should_panic] // To tell the compiler that the test should PANIC
    fn test_basic(){
        assert!(1 == 1);
        panic!("Oh now!");
    }

    #[test]
    fn test_equals(){
        assert_eq!(2, 1+1);
        assert_ne!(2, 1+3);
    }

    #[test]
    #[ignore]
    fn test_ignored(){
        assert!(2==2);
    }
    #[test]
    fn test_outside_fn(){
        assert_eq!(super::give_two(),1+1);
    }

    #[test]
    #[should_panic]
    fn test_struct(){
        let my_rectangle = super::Rectangle{
            width: 25,
            height: 50
        };
        assert!(my_rectangle.is_square());
    }
}

fn give_two() -> i32{
    2
}

/*
error_chain!{
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
} 
*/

//use log::error;

mod learnrust;

mod module{
    pub fn generic_message() {
        println!("Hello, how's going?");
        chicken(); //PUBLIC FN CAN CALL A PRIVATE ONE IN THE SAME MOD
    }

    fn chicken(){ // NOTE THAT CHICKEN IS PRIVATE
        println!("Chicken!");
    }

    pub mod water{
        pub fn generic_message(){
            println!("I'm water!");
        }
    }
}



//Enum Type
enum Directions{
    Up, 
    Down,
    Left, 
    Right
}

enum Day{
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturnday, Sunday
}

impl Day{
    fn is_weekday(&self) -> bool{
        match &self{
            &Day::Saturnday | &Day::Sunday => return false,
            _ => return true
        }
    }
}

//Constant
const MAX_NUMBER: u8 = 100;

//Functions
fn print_numbers(num: u32) -> bool{
    for n in 1..num{
        println!("Number is {}",n);
    }
    return true;
}

//Functions
fn is_even(num: u32) -> bool{
    if num % 2 == 0{
        return true;
    }
    return false;
}

fn log_file(path: &str, text: &str) -> std::io::Result<()> {

    let now: DateTime<Local> = Local::now();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{} : {}\n --- ", now.format("%Y-%m-%d %T"),text)?;
    Ok(())
}

//Struct
struct Color{
    red: u8, // The U8 Datatype support 0-255
    green: u8,
    blue: u8
}

struct Rectangle{
    height: u32,
    width: u32
}

impl Rectangle{
    fn print_description(&self){
        println!("Rectangle: {} x {}",self.width, self.height);
    }
    fn is_square(&self) -> bool{
        if self.width == self.height {
            return true;
        }
        return false;
    }
}

//Tuple Structs
struct TpColor(u8, u8, u8);

fn print_color(c: &Color){
    println!("Color R:{} G:{} B:{}",c.red, c.green, c.blue);
}

struct Person{
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self) -> String {
        return format!("My name is {} and I'm {} old.", self.name, self.age);   
    }
}

trait HasVoiceBox{
    fn speak(&self);

    fn can_speak(&self) -> bool;
}

impl HasVoiceBox for Person{
    fn speak(&self){
        println!("Hello, my name is {}", self.name);
    }
    fn can_speak(&self) -> bool {
        if self.age > 1{
            return true;
        } 
        return false;
    }
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Vinicios" => Some("Programmer"),
        "Ana" => Some("Lawyer"),
        _ => None
    }
}
/*
async fn httpget(url: &str){
    match reqwest::get(url).await{
        Ok(mut response) => {
            //Check if 200 ok
            if response.status() == reqwest::StatusCode::OK{
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text.")
                }
            }
            else{
                println!("Response was not 200 OK.")
            }
        }
        Err(_) => println!("Get request failed!")
    }
}
*/


async fn http_get() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get("https://api.tallk.me/dbdial_webapi.php?acao=info_conta&id_cliente=223&token=NX8RK6GTJZLSHUPCFOE0&ano_mes=202212").send()
        .await?
        .text()
        .await?;

    Ok(body)
}


//=========================================================================================================
#[tokio::main]
async fn main() {
    // The basics
    // This is an example of a comment. (single line)
    println!("### The basic:");
    println!("Hello, other worlds!");
    /*
    This is a exemple of a comment (multiple lines)
    */

    println!("---------\n");
    println!("### Variables: ");
    // Variables
    let mut x = 45;
    println!("The value of X is: {}",x);

    x = 65; // Only works if "X" is let as "mut"
    println!("The new value of X is: {}",x);

    println!("---------\n");
    println!("### Variables Types: ");
    // Variables data types
    // let mut x = 45; by default is type i32 (integer 32bits)

    let y: i64 = -64; // i64 integer 64 bits
    let z: u64 = 64; // u64 unsign integer 64bits
    let f: f32 = 6.6; // float 32 bits
    let b: bool = false; // boolean

    println!("The values of Y, Z, F and B are: {}, {}, {}, {}",y,z,f,b);

    println!("---------\n");
    println!("### IF ELSE Statements: ");

    // IF / ELSE statements

    let n = 0;

    //Standard notation > < = != 
    if n == 0 || n == 100{
        println!("The number is 0 or 100");
    }
    else if n > 30{
        println!("The number is greater than 30");
    }
    else{
        println!("The number is less than 30");
    }

    println!("---------\n");
    println!("### Loop: ");
    // LOOP
    let mut i = 0;

    loop{
        println!("The value of i is: {}", i);
        i +=1;

        if i == 7{
            println!("N equals 7... continue");
            continue; //Skips everything else and go back to the top
        }

        if i>10{
            break;
        }
    }
    println!("Loop finished");
    println!("---------\n");
    println!("### While: ");

    // WHILE
    let mut a = 1;
    let max = 50;
    while a <= max{
        if a % 5 == 0 {
            println!("(1 to {}) Multiple of 5 find: {}",max,a);
        }
        a +=1;
    }

    println!("---------\n");
    println!("### For: ");
    // FOR

    let range = 10..51;
    for o in range{
        if o % 5 == 0{
            println!("Loop using for. Multiple of 5 find: {}", o);
        }
        
    }
    
    let vector = vec!["First", "Second", "Third"];
    for (index, u) in vector.iter().enumerate(){
        println!("Loop using for with vector. Index: {} Position is: {}", index, u);

    }
    println!("---------\n");
    println!("### Enum Type: ");
    // ENUM TYPES
    let direct:Directions = Directions::Up;

    match direct{
        Directions::Up => println!("Direction is: UP"),
        Directions::Down => println!("Direction is: DOWN"),
        Directions::Left => println!("Direction is: LEFT"),
        Directions::Right => println!("Direction is: RIGHT")
    }

    println!("---------\n");
    println!("### Constants: ");
    // CONSTANTS
    println!("Max number is: {}", MAX_NUMBER);

    //TUPLES
    let tup1 = (20, "Code", 30, 40, 50.75, "Rust", false, (1, "Word", true));
    println!("Tuple Value: {}", tup1.6);
    println!("Tuple within a Value: {}", (tup1.7).2);

    let tup2 = (1.1, "Computer", false);
    let (h,j,k) = tup2;
    println!("Values of H, J and K are: {}, {} and {}",h, j, k);

    println!("---------\n");
    println!("### Functions: ");
    // FUNCTIONS

    let result = print_numbers(10);
    println!("Result is: {}", result);

    let even = is_even(10);
    println!("Is Even? {}", even);

    println!("---------\n");
    println!("### Codeblocks: ");
    //CODEBLOCKS

    let outside = 10;
    {
        //VARIABLES HERE CAN NOT BE ACCESS OUTSIDE THE CODEBLOCK
        //BUT IT HAS ACCESS TO THE OUTSIDE
        let inside = 15;
        println!("Value of Inside is: {}",inside);
        println!("Value of Outside is: {}",outside);
    }
    //THIS WONT WORK
    //println!("Value inside of the codeblock: {}",inside);

    println!("---------\n");
    println!("### Shadowing: ");
    //SHADOWING
    //Is kinda like redeclaring, but the original value of the variable, outside the codeblock is preserved.
    let shadow = 10;
    println!("Value of shadow outside the codeblock is: {}", shadow);

    {
        let shadow = 20;
        println!("Value of shadow inside the codeblock is: {}", shadow);
    }

    println!("Value of shadow outside and after the codeblock is: {}", shadow);

    //You can also use let to redefine the type of variable 
    let shadow = "String";
    println!("Last value of shadow: {}", shadow);

    println!("---------\n");
    println!("### References: ");
    //REFERENCES

    let mut original = 10;
    println!("Original is: {}", original); 
    {
        let mutref = &mut original;
        *mutref +=1;
        println!("Mutables reference +1 is: {}", mutref);
    }
    println!("Original is: {}", original); // THIS WONT WORK  IF mutref IS USED OUTSIDE OF A CODEBLOCK

    println!("---------\n");
    println!("### Structs: ");
    //STRUCTS
    let mut color = Color{red: 255, green: 70, blue: 15};
    println!("Red: {}  \nGrenn: {} \nBlue: {}",color.red, color.green, color.blue);

    color.blue = 255;
    println!("\nRed: {}  \nGrenn: {} \nBlue: {}",color.red, color.green, color.blue);

    println!("---------\n");
    println!("### Tuple Structs: ");
    //TUPLE STRUCTS
    let mut red = TpColor(255, 0, 0);

    println!("Red is: {} {} {}", red.0, red.1, red.2);

    red.2 = 1;

    println!("Red is: {} {} {}", red.0, red.1, red.2);
    
    println!("---------\n");
    println!("### Pass by reference: ");
    //PASS BY REFERENCES
    let blue = Color{red: 0, green: 0, blue: 255};

    print_color(&blue);

    println!("---------\n");
    println!("### Arrays: ");
    //ARRAYS

    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array value in position 0 is {}", array[0]);

    println!("Print within for (loop)");
    for c in 0..array.len(){
        println!("Value: {}",array[c]);
    }

    println!("---------\n");
    println!("### IMPL KEYWORDS: ");
    //IMPL KEYWORDS

    let my_rectangle = Rectangle {width: 15, height: 5};

    my_rectangle.print_description();
    let is_square = my_rectangle.is_square();
    println!("My rectangle is square: {}",is_square);

    println!("---------\n");
    println!("### STRINGS: ");
    //STRINGS

    let mut my_string = String::from("Hey, whazzap. My name is Viny");
    println!("Lenght: {}",my_string.len());
    println!("IsEmpty: {}",my_string.is_empty());
    println!("Original String: {}",my_string);

    println!("\nPrint by line (splite_whitespace):");
    for token in my_string.split_whitespace(){
        println!("{}",token);
    }

    println!("\nDoes the string contains 'Viny'? {}", my_string.contains("Viny"));

    my_string.push_str(" and I'm learning Rusty.");
    println!("String after push: {}",my_string);
    
    println!("---------\n");
    println!("### TRAITS: ");
    //TRAITS

    let me = Person{name: String::from("Viny"), age: 33};

    println!("{}",me.to_string());

    println!("---------\n");
    println!("### VECTORS: ");
    //VECTORS (ARRAYS IN STEROIDS)

    let mut my_vector1: Vec<i32> = Vec::new();
    let my_vector2 = vec![1,2,3,4];

    my_vector1.push(49);
    my_vector1.push(13);
    println!("My Vector 1 Before: {}",my_vector1[0]);
    my_vector1.remove(0);
    println!("My Vector 1 After: {}",my_vector1[0]);
    println!("My Vector 2: {}",my_vector2[0]);

    println!("---------\n");
    println!("### READ A FILE: ");
    //READ A FILE

    let mut file = File::open("info.txt").expect("Can't open file");
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Ops, can not read the file...");

    println!("File content: {}", content);

    println!("---------\n");
    println!("### COMMAND LINE ARGUMENTS: ");
    //COMMAND LINE ARGUMENTS (SIMILAR TO $ARGV[i])

    let args: Vec<String> = env::args().collect();

    for arguments in args.iter(){
        println!("Argument: {}", arguments);
    }

    println!("---------\n");
    println!("### NOT RELATED TANGENT: ");
    //HOW TO APPEND A TEXT TO THE END OF A FILE

    let path = "log.log";
    let txt = "Add a new line to a file.";
    let result = log_file(path,txt);

    if result.is_ok() {
        println!("Successfully logged to file!");
    } else {
        println!("Error appending to file: {:?}", result.unwrap_err());
    }

    println!("\x1b[89m Gray \x1b[0m");
    println!("\x1b[90m Gray \x1b[0m");
    println!("\x1b[91m Red \x1b[0m");
    println!("\x1b[92m Green \x1b[0m");
    println!("\x1b[93m Yellow \x1b[0m");
    println!("\x1b[94m Blue \x1b[0m");
    println!("\x1b[95m Purple \x1b[0m");
    println!("\x1b[96m Water \x1b[0m");
    println!("\x1b[97m White \x1b[0m");

    println!("---------\n");
    println!("### DEFINING TRAITS ");
    //DEFINING TRAITS

    let someone = Person{
        name: String::from("Bob"),
        age: 39
    };

    someone.speak();

    println!("Can {} person speak? {}", someone.name, someone.can_speak());

    println!("---------\n");
    println!("### MATCH ");
    //MATCH (SWITCH LIKE)

    let number = 6;

    match number {
        1 => println!("Number is 1."),
        2 => println!("Number is 2."),
        3 => println!("Number is 3."),
        4..=9 => println!("Number is between 4 and 9."),
        10 | 12 => println!("Number is either 10 or 12."),
        _ => println!("It doesn't match. {}", number)
    }

    let labbel = "shit";

    match labbel {
        "Can" => println!("It's just a can bro."),
        "Fork" => println!("You can use it to eat, except beans."),
        "Spork" => println!("Mother of all utensils."),
        _ => println!("Dunno what this shit is... {}", labbel)
    }
    println!("---------\n");
    println!("### READING USER INPUT ");
    //READING USER INPUT

    let mut input = String::new();
    println!("\x1b[92mHey bro, say something \x1b[0m: ");

    //GET FROM STANDARD INPUT
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Success. You said: {}", input.to_uppercase());
        },
        Err(e) => println!("Ops! Something went wrong: {}", e)   
    }
    println!("---------\n");
    println!("### HASH MAPS ");
    //HASH MAPS

    let mut marks = HashMap::new();

    //ADD VALUES
    marks.insert("Rust Programming", 80);
    marks.insert("Web Development", 60);
    marks.insert("C#", 80);
    marks.insert("PHP", 91);

    for (subject,mark) in &marks{
        println!("For {} you got {}%!", subject, mark);
    }

    //PRINT THE TOTAL SIZE 
    println!("How many subjects were studied: {}", marks.len());

    match marks.get("Web Development"){
        Some(marks) => println!("You got {} for Web Development\n",marks),
        None => println!("You do not study Web Development\n")
    }

    //REMOVE KEYS + VALUES
    marks.remove("Web Development");

    for (subject,mark) in &marks{
        println!("For {} you got {}%!", subject, mark);
    }

    // Check for a value
    println!("Did you study C#? {}", marks.contains_key("C#"));

    println!("---------\n");
    println!("### RANDOM NUMBERS AND BOOLEANS ");
    //RANDOM NUMBERS AND BOOLEANS

    let random_number = rand::thread_rng().gen_range(1,21); // 1 to 20 (21 not included)
    println!("Random number: {}", random_number);

    //Flip a coin
    let coin = rand::thread_rng().gen_weighted_bool(2); 
    //Arg that you have 1 in NUM chances to beeing true. So that means that with 2, you have a 1 in 2 chances to be true

    println!("Random Boolean: {}", coin);

    println!("---------\n");
    println!("### MORE STRING METHODS ");
    //MORE STRING METHODS

    //Replace
    {
        let my_string = String::from("Rust is good, rust is love.");
        println!("Before replace: {}", my_string);
        println!("After replace: {}", my_string.replace("good", "great"));
        println!("\n");
    }

    //Lines
    {
        let my_string = String::from("The wheather is nice \noutside \nmate.");
        for line in my_string.lines(){
            println!("[ {} ]",line);
        }
    }

    //Split
    {
        let my_string = String::from("Let+we+have+the+string+divided+by+plus+signs!");
        let explode: Vec<&str> = my_string.split("+").collect();

        println!("Full string: {}",my_string);
        println!("At index 2: {}",explode[2]);
        println!("\n");
    }

    //Trim
    {
        let my_string = String::from("   My name is Viny.   Okay... \n\r");
        println!("Before trim: {}", my_string);
        println!("After trim: {}", my_string.trim());
        println!("\n");

    }

    //Chars
    {
        let my_string = String::from("Learning rust");
        println!("The full string: {}", my_string);
        //Get Character at index
        match my_string.chars().nth(4) {
            Some(c) => println!("Character at index 4: {}", c),
            None => println!("No character at index 4")
        }
    }

    println!("---------\n");
    println!("### MULTIPLE SOURCE FILES ");
    //MULTIPLE SOURCE FILES

    learnrust::print_message();

    println!("---------\n");
    println!("### REGULAR EXPRESSIONS ");
    //REGULAR EXPRESSIONS

    //let re = Regex::new(r"\d").unwrap(); // Match a 1 digit
    let re = Regex::new(r"(\w{5})").unwrap(); // Match a 5 letter word  () -> is a capture
    let text = "abcdefghijk";

    println!("Found match {}", re.is_match(text));

    match re.captures(text){
        Some(caps) => println!("Found match: {}", caps.get(0).unwrap().as_str()), 
        None => println!("Could not found the match...")
    }
    println!("---------\n");
    println!("### MODULES ");
    //MODULES

    module::generic_message();
    module::water::generic_message();

    println!("---------\n");
    println!("### OPTION ENUN ");
    //OPTION ENUN

    let name = String::from("Vinicios");

    println!("Character at index 0: {}", match name.chars().nth(0){
        Some(c) => c.to_string(),
        None => "No character at index 0".to_string()
    });

    println!("Occupation is: {}", match get_occupation("Vinicios"){
        Some(o) => o,
        None => "No occupation found!"
    });

    println!("---------\n");
    println!("### HTTP REQUEST (NOT WORKING)");
    //HTTP REQUEST

    //FIRST WAY (NOT WORKING)
    /*
    match reqwest::get("http://localhost/OlaMundo"){
        Ok(mut response) => {
            //Check if 200 ok
            if response.status() == reqwest::StatusCode::Ok{
                match response.text() {
                    Ok(text) => println!("Response Text: {}", text),
                    Err(_) => println!("Could not read response text.")
                }
            }
            else{
                println!("Response was not 200 OK.")
            }
        }
        Err(_) => println!("Get request failed!")
    }
    */

    //SECONDARY WAY (NOT WORKING)
    /*
    let response_text = reqwest::get("http://localhost/OlaMundo")
        .expect("Could not make the request.")
        .text().expect("Could not read the response text.");
    
    println!("Response text: {}", response_text); 
    */

    //OTHER WAYS
    let api_response = http_get().await;
    println!("API Response (JSON) = {:#?}", api_response);

    println!("---------\n");
    println!("### ENUM METHODS");
    //ENUM METHODS

    let day = Day::Tuesday;
    println!("Is D a WeekDay? {}", day.is_weekday());

    println!("---------\n");
    println!("### EXECUTING/RUNNING COMMANDS");
    //EXECUTING/RUNNING COMMANDS

    let mut cmd = Command::new("python"); //Aplication/Executable
    cmd.arg("python.py"); // Argumento (name of the script)

    //Execute the command
    match cmd.output(){
        Ok(o) => {
            unsafe{
                println!("Output: {}", String::from_utf8_unchecked(o.stdout)); // THIS IS UNSAFE
            }
        },
        Err(e) => {
            println!("There was an Error! {}",e);
        }
    }

    println!("---------\n");
    println!("### JSON DECODE");
    //JSON DECODE

    //String in Json
    let json_string = r#"
        {
            "name": "Viny",
            "age": 33,
            "is_male": true,
            "phones": {
                "first": 999999999,
                "second": 333333333
            },
            "is_human": true
        }
    "#;

    //using simple string (#1)
    let json = serde_json::from_str(json_string);
    if json.is_ok(){
        let valid_json: JsonValue = json.unwrap();
        println!("(ORI) The name is: {}", valid_json["name"]);
        println!("(ORI) First Phone: {}", valid_json["phones"]["first"]);
        println!("(ORI) Second Phone: {}", valid_json["phones"]["second"]);
        println!("(STR) The name is: {}", valid_json["name"].as_str().unwrap()); //Using unwrap is not recommended
    }
    else{
        println!("Sorry, invalid json :(");
    }

    println!("\n");

    //using simple string + struct (#2)
    let json2 = serde_json::from_str(json_string);
    if json2.is_ok(){
        let valid_json2: PersonExample = json2.unwrap();
        println!("(Using Struct) The name is: {}", valid_json2.name);
        println!("(Using Struct) The age is: {}", valid_json2.age);
        println!("(Using Struct) Is male?: {}", valid_json2.is_male);
    }
    else{
        println!("Sorry, invalid json :(");
    }


    //Using API response (NOT WORKING YET)
/*
    let api_response_str = match api_response {
        Ok(data) => "Success",
        Err(e) => &format!("Error: {}", e)[..],
    };
    
    let json_api = serde_json::from_str(api_response_str);
    if json_api.is_ok(){
        let valid_json_api: JsonValue = json.unwrap();
        println!("(API) The name is: {}", valid_json_api["info_conta"]["IDCliente"]);
    }
    else{
        println!("Sorry, invalid json :(");
    }
*/
}