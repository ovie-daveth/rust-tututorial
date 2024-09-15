pub fn ownership(){
    const PI: f32 = 3.14;
    println!("Hello world from Ownsership. {}", PI);
    shadow_example();
    ifelse();
    loop_example();
    label_loop();
}

//SHADOWING
fn shadow_example(){
    let x = 5;
    let x = x + 1;
    println!("{}", x); // 30

    {
        let x = x * 5;
        println!("{}", x); // 30
    }
}

//control flow

fn ifelse(){
    let age = 17;
    if age >= 18{
        println!("You can drive a car");
    } else {
        println!("You cannot drive a car");
    }
}

fn loop_example(){

    let mut counter = 0;

    loop {
        counter += 1;
        println!("Hello world, {}", counter);

        if counter == 10 {
            break;
        }

    }
}

fn label_loop(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");
}