//Rust program to understand lifetimes

fn borrow_and_return(x : String) -> String {
    println!("Value borrowed is {}", x);
    x
}
fn borrow_and_return_ptr(x : &str) -> &str {
    println!("Value borrowed is {}", x);
    x
}


fn correct_borrow() {
    let my_string = String::from("Hello!");
    let returned_string = borrow_and_return(my_string);
    println!("Returned value is {}", returned_string); //can only access this and not my_string
}
fn pointer_borrow() {
    let my_string = "Hello!";
    let returned_string = borrow_and_return_ptr(my_string); // No need to store it in returned_string
    println!("Returned value is {}", returned_string); //This works as we are accessing string pointer and not string
    println!("Returned value is {}", my_string);
    if my_string==returned_string {
        println!("They are the same pointers!");
    }
}
fn incorrect_borrow() {
    let my_string = String::from("Hello!");
    let returned_string = borrow_and_return(my_string);
    //println!("Original value is {}", my_string); //can't access as my_string doesn't belong to it anymore
    //borrow of moved value
    //another solution : let returned_string = borrow_and_return(my_string.clone());
}
fn main() {
    //Uncomment one at a time to observe
    correct_borrow();
    //pointer_borrow();
    //incorrect_borrow();   
}
