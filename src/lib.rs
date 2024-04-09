/*
Introduction
Your friend Eliud inherited a farm from her grandma Tigist. Her granny was an inventor and had a tendency to build things in an overly complicated manner. The chicken coop has a digital display showing an encoded number representing the positions of all eggs that could be picked up.

Eliud is asking you to write a program that shows the actual number of eggs in the coop.

The position information encoding is calculated as follows:

Scan the potential egg-laying spots and mark down a 1 for an existing egg or a 0 for an empty spot.
Convert the number from binary to decimal.
Show the result on the display.
*/

pub fn egg_count(display_value: u32) -> usize {
    let mut counter: usize = 1;
    let mut decimals: Vec<usize> = Vec::new();
    let mut result: usize = 0;
    let mut display_value: usize = display_value as usize;
    loop {
        decimals.push(counter);
        if counter >= display_value {
            break
        }
        counter += counter;
    }
    decimals.reverse();
    for i in &decimals {
        if i <= &display_value {
            display_value -= i;
            result += 1;
        }
        if display_value == 0 {
            break;
        }
    }
    result
}
#[test]
fn test_0_eggs() {
    let input = 0;
    let output = egg_count(input);
    let expected = 0;
    assert_eq!(output, expected);
}
#[test]
fn test_1_egg() {
    let input = 16;
    let output = egg_count(input);
    let expected = 1;
    assert_eq!(output, expected);
}
#[test]
fn test_4_eggs() {
    let input = 89;
    let output = egg_count(input);
    let expected = 4;
    assert_eq!(output, expected);
}
/*
#[test]
fn test_13_eggs() {
    let input = 2000000000;
    let output = egg_count(input);
    let expected = 13;
    assert_eq!(output, expected);
}
*/