//contstant variables
const MAX_POINTS: u32 = 100_000 ;


fn main() {
    // ----- Lecture 001 -----
    // how to define a variable
    let x = 5;
    let y = 10;
    // println!("{}*2 = {}", x,y);

    // if creating a variable you need to let compiler know 
    // that you are planing to changet
    let mut result: f32 = 0.0;
    result += 1.5;
    // println!("{}", result);

    // using "as" will just change type on the fly
    let x1 = 10;
    let y1 = 2.0;

    let result1 = x*(y as i32);
    // println!("{}", result1);

    //const variables need to be define type and cannot be 
    // modefied while program runing

    // println!("{}", MAX_POINTS);

    // ----- Lecture 002 [Jan-27-2026] -----
    //shadowing
    let num = "25";
    let result2 : Result<i32,_> = num.parse();
    // println!("{:?}", result2);
    let num2 = "25";
    let result3 : f32 = num.parse().expect("I am sure");
    // println!("{}", result3);

    // ----- review -----
    //shadow
    let x = 5;
    let x = x+1;
    //Mutation
    let mut y = 5;
    y = y + 1;
    // println!("x: {}, y = {}", x, y);

    // ----- Function -----
    get_number();
    println!("{}", get_number1(10));
    println!("{}", get_number2(10));

}

// ----- lecture003 ----
fn get_number(){
    let x = 5;
    println!("{}", x)
}
fn get_number1(y: i32) -> i32{
    let x = 5;
    return x + y; //since its the last line x+y will be run and the return the expresion(works kind of wird xD)
    // no semicolon, its an expresion
}
fn get_number2(y: i32) -> i32{
    let x = 5;
    let y = {
        let x = x + 3;
        y + x
    };
    y
}