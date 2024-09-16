// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue"); // reference

    string("red".to_string()); // creates a new owned copy

    string(String::from("hi")); // creates a new owned copy

    string("rust is fun!".to_owned()); // creates a new owned copy

    string("nice weather".into()); // creates a new owned copy

    string(format!("Interpolation {}", "Station")); // creates a new owned copy

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues")); // creates a new owned copy

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // creates a new owned copy
}
