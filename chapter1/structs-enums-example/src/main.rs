// Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Enum
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Methods (impl block)
impl User {
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
}

fn main() {
    // Create a user using the constructor
    let user1 = User::new(
        String::from("alice"),
        String::from("alice@example.com"),
    );

    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-in count: {}", user1.sign_in_count);

    // Create some enum variants
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 128),
    ];

    println!("\nMessages:");
    for (i, msg) in messages.iter().enumerate() {
        match msg {
            Message::Quit => println!("  {}: Quit", i),
            Message::Move { x, y } => println!("  {}: Move to ({}, {})", i, x, y),
            Message::Write(text) => println!("  {}: Write '{}'", i, text),
            Message::ChangeColor(r, g, b) => println!("  {}: ChangeColor({}, {}, {})", i, r, g, b),
        }
    }
}
