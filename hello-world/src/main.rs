fn fizzbuzz(i: u32) -> String {

    // things can be returned from an if block as if it is a function call
    let x = if i % 3 == 0 && i % 5 == 0 {
        String::from("FizzBuzz")
    } else if i % 3 == 0 {
        String::from("Fizz")
    } else if i % 5 == 0 {
        String::from("Buzz")
    } else {
        i.to_string()
    };
    x
}

#[test]
fn my_test() {
    assert_eq!(1, 1);
}

fn main() {
    for i in 1..101 {
        let fizz = fizzbuzz(i);
        println!("{}", fizz);
    }
}
