fn main() {
    
    let mut x: i32 = 55;
    let mut y: i32 = 33;

    println!("The First value is {} and The Second value is {}", x, y); // 1st

    // Value Changed
    x = 60;
    y = 75;

    println!("The First value is {} and The Second value is {}", x, y); // 2nd

    test();


}

fn test() {

    let name: &str = "Shankar";
    let age: &str = "22";

    println!("My name is {} and my age is {}", name, age);

}

