use std::path::Display;

#[derive(Debug)]
struct Mvce {
    reasonA: i32, // reason a
    reason3: (),  // reason 3
}

struct Ms;

use std;
impl std::fmt::Display for Ms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = write!(f, "Buzz");
        result?;
        result
    }
}

fn add_a_number_that_represent_a_single_digit_which_is_should_maybe_be__(a: &mut i32) {
    *a += 1;
}

fn is_probably_fizz() -> bool {
    true
}

fn main() -> Result<(), Mvce> {
    let mut rth = 80 * 2 - 80; // calculate 80 * 2 - 80
    rth += 1;

    print!("1");
    print!("    Buzz");
    println!("Fizz");

    rth -= 1;

    let a = 5;
    let b: u8;

    println!("2");

    if (rth + 1) % 3 == 0 {
        println!("Fizz");
    } else {
        return Err(Mvce {
            reasonA: 6,
            reason3: (),
        });
    }

    println!("{}", rth - 80 + 4);

    rth -= 30; // Subtract...

    print!("");

    rth -= 5;

    if (50 - rth) % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{}", Ms {});
    }
    rth -= 1;
    println!("{}", 50 - rth);
    rth -= 1;

    add_a_number_that_represent_a_single_digit_which_is_should_maybe_be__(&mut rth);
    rth -= 2;

    let j = vec![7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    j.iter()
        .enumerate()
        .filter(|(o1, &o2)| o2 % 3 != 0 && o2 % 5 != 0)
        .for_each(|(f, &f12)| println!("{}", f12));

    j.iter()
        .enumerate()
        .filter(|(o1, &o2)| o2 % 3 == 0 && !(o2 % 3 == 0 && o2 % 5 == 0))
        .for_each(|(f, &f12)| println!("Fizz"));

    j.iter()
        .enumerate()
        .filter(|(o1, &o2)| o2 % 5 == 0 && !(o2 % 3 == 0 && o2 % 5 == 0))
        .for_each(|(f, &f12)| println!("Buzz"));

    j.iter()
        .enumerate()
        .filter(|(o1, &o2)| {
            o2 % 3 == 0
                && o2 % 5 == 0
                && !(o2 % 3 == 0 && !(o2 % 3 == 0 && o2 % 5 == 0))
                && !(o2 % 5 == 0 && !(o2 % 3 == 0 && o2 % 5 == 0))
        })
        .for_each(|(f, &f12)| println!("FizzBuzz"));

    for i in 0..100 {
        if i > 5 {
            if i > 20 {
                if i < 100 {
                    if i < 51 {
                        if is_probably_fizz() && i % 3 == 0 && !(i % 3 == 0 && i % 5 == 0) {
                            println!("Fizz");
                        } else if !is_probably_fizz() && i % 5 == 0 && !(i % 3 == 0 && i % 5 == 0) {
                            println!("Buzz");
                        } else if (is_probably_fizz() || !is_probably_fizz())
                            && i % 3 == 0
                            && i % 5 == 0
                            && !(i % 3 == 0 && !(i % 3 == 0 && i % 5 == 0))
                            && !(i % 5 == 0 && !(i % 3 == 0 && i % 5 == 0))
                        {
                            println!("FizzBazz");
                        }
                    }
                }
            }
        }
    }

    if !is_probably_fizz() {
        return Err(Mvce {
            reasonA: 9,
            reason3: (),
        });
    }

    Ok(())
}
