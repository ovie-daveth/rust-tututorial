
mod ownership;
mod learn;
mod guessing;

fn main() {
    guessing::guessing();
    // let num: i32 = 23;
    // let num_2: i32 = 54;
    // let number: [i32; 5] = [1,2,3,4,5];
    // let array_string: [&str; 3] = ["aArrow", "Ovie", "David"];
    // let my_tuple: (i32, f64, &str) = (42, 3.14, "hello");

    // let sum: i32;

    // sum = num + num_2;

    // let mut speech: String = String::from("Hello, ");
    // speech.push_str("world");
    // let slice: &str = &speech[0..5];
    // println!("{}", slice);

    // println!("The result from debug is: {:?}, {}, {:?}, {:?}", number, debug(sum), array_string[1], my_tuple );

    let height_cm = 155;
    let weight_kg = 72;
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 500.54
    };

    //mutable withdrawal
    account.withdraw(56.54);

    //immutable borrow
    account.check_balance();

    let bmi = bmi_calculate(weight_kg, height_cm);
    println!("The BMI is {:?}", bmi);
    borrow_ref();
    ownership::ownership();
    learn::learn();

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
}

// fn debug(num: i32) -> i32 {
//     num * num
// }

fn bmi_calculate(weight: i32, height: i32) -> f32 {
    // Convert height from centimeters to meters
    let height_m: f32 = height as f32 / 100.0;

    // Calculate BMI with floating-point division
    let bmi: f32 = weight as f32 / (height_m * height_m);
    return bmi;
}

// //ownership
// fn owner_ship(){
//     let s1 = String::from("Oviet");
//     let s2 = s1;
//     let len = calculate_length(&s2);
//     println!("The size of  '{}' is {}", s2, len);
// }

fn calculate_length(s: &String) -> usize {
    return s.len();
}

//borrowing and references
fn borrow_ref(){
    //mutable refer
    let mut x = 5;
    let r = &mut x; // referencing. without the & x would no longer exist but eith the & both exist using the same memory but x is the owner


    *r += 1;
// you cannot print both at the same time if you ref one mutably
    println!("{}", x);

    let p = 12;
    let g = &p;

    // you can print infinite numbers of immutable reference
    println!("{}, {}", p, g)

}

//struct



struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount{
    fn withdraw(&mut self, amount: f64){
        println!("withdrawing {} from account ownedby {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("account owned by {}, has a balance of {}", self.owner, self.balance)
    }
}