use std::collections::BTreeMap;

macro_rules! my_macro_1 {
    ($e:literal) => {{
        let val: &str = $e;
        let result: String = format!("Hello {}", val);
        result
    }};
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

    // iterator usage
    let v = vec!["ana", "ars", "alu", "sel"];

    for element in &v {
        println!("{}", element);
    }

    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    let mut iterator = v.into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }

    capitalize_first("hello");

    let numbers = vec![27, 297, 38502, 81];
    let division_results: Result<Vec<_>, DivisionError> = Ok(numbers
        .into_iter()
        .map(|n| divide(n, 27))
        .filter_map(Result::ok)
        .collect());

    println!("{:?}", division_results);

    print_range(0);
    print_range(1);
    print_range(2);

    let data = populate_cities();
    println!("{:?}", data);
    println!("{:?}", data.values());

    let mut cities_1: Vec<&str> = Vec::new();
    data.values().fold(&mut cities_1, |accum, v| { for c in v {accum.push(c)}; accum });
    println!("{:?}", cities_1); 
    let mut cities_2: Vec<&str> = Vec::new();
    data.values().fold(&mut cities_2, |accum, v| { accum.extend(v); accum }); 
    println!("{:?}", cities_2); 
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

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let s = match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    };
    s
}

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    match a % b {
        0 => Ok(a / b),
        _ => Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        })),
    }
}

pub fn print_range(num: u64) {
    println!("{:?}", (1..num + 1).collect::<Vec<_>>());
}

fn populate_cities() -> BTreeMap<&'static str, Vec<&'static str>> {
    let mut cities = BTreeMap::new();

    cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    cities.insert("The United States", vec!["Portland", "Nashville"]);
    cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    cities
}
