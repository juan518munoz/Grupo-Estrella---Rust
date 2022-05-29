fn main() {
    let x = 5;
    let x = x + 1;
    let five = five();

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    statement();
    println!("Print {}", five);
}

fn shadow() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn mutability() {
    let mut spaces = "   ";
    //spaces = spaces.len();
}

fn statement() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}
