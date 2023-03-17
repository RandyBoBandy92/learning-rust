fn greet(name: &str, title: Option<&str>) {
    let title = title.unwrap_or("");
    // if title is an empty string, do a different println
    if title == "" {
        println!("Hello, {}!", name);
        return;
    }
    println!("Hello, {} {}!", title, name);
}

fn main() {
    greet("Alice", Some("Ms."));
    greet("Bob", None);
}
