#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        //field init shorthand, when parameters are the same as fields so its automatically assigned
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1: User = build_user(String::from("bob@email.com"), String::from("bob"));
    println!("User details:{:#?}", user1);

    //this moves ownership of "bob" from user1.username to original (pointer re-allocation)
    let original_name = user1.username;
    // println!("{}", user1.username);//THIS WILL CAUSE ERROR, trying to access moved value
    user1.username = String::from("bruh");
    println!("{} name is changed to {:#?}", original_name, user1.username);
    println!("New user details:{:#?}", user1);

    println!("--------USING STRUCT TUPLES FOR MATH------");
    let rect = Rectangle {
        width: dbg!(30 * 2), //takes ownership but returns it to width
        height: 50,
    };
    dbg!(&rect); //allowed because borrow
    // dbg!(rect); //moves ownership which would invalidate the next line
    let rect = dbg!(rect); //takes ownership and returns to rect (shadowing)
    println!("Area of rectangle: {}", area(&rect));
    //We're allowed to use rect again because &rect is an immutable borrow, so ownership isn't moved
    println!("Rectangle definition:{:?}", rect);
}
