fn main() {
    let temperature: f64= 64.0;
    let c = fahrenheit_to_celsius(temperature);
    println!("{} degrees F is {} deegrees C", temperature, c);
    let f = celsius_to_fahrenheit(temperature);
    println!("{} degrees C is {} deegrees F", temperature, f);
}

fn fahrenheit_to_celsius(f: f64) -> f64{
    return (f - 32.0) * (0.5555);
}
fn celsius_to_fahrenheit(c: f64) -> f64{
    return (c * 1.8) + 32.0;
}