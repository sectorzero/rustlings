macro_rules! my_macro_1 {
    ($e:literal) => {
        {
            let val: &str = $e;
            let result: String = format!("Hello {}", val);
            result
        }
    };
}

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    // strings
    string_slice("blue");

    let s: String = String::from("abc"); 
    let t: &str = &s[0..1];
    string_slice(t);

    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());

    let f: String = format!("Interpolation {}", "Station");
    string(f);

    let h: String = "Happy Monday!".to_string();
    let i = h.replace("Mon", "Tues");
    string(i);

    let x: &str = "mY sHiFt KeY iS sTiCkY";
    let y: String = x.to_lowercase();
    string(y);

    let m: &str = "  hello there ";
    let n: &str = m.trim();
    string_slice(n);

    // macros
    let sample: String = my_macro_1!("Mundo");
    println!("{:?}", sample);
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}

fn string(arg: String) {
    println!("{}", arg);
}