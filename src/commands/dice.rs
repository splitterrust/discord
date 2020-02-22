use cfg_if;
use log::error;
use regex::Regex;
use serenity::framework::standard::{
    macros::command,
    Args,
    CommandError,
    CommandResult,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
cfg_if::cfg_if! {
    if #[cfg(test)] {
        use std::io;
    } else {
        use rand::Rng;
    }
}

#[derive(Debug)]
struct Roll {
    roll_string: String,
    roll_values: Vec<u32>,
    roll_result: u32,
}
impl Roll {
    fn calculate_roll_result(&mut self) {
        self.roll_result += self.roll_values.iter().sum::<u32>()
    }
    fn generate_roll_result_string(&self) -> String {
        let mut result = String::from("(");
        for value in &self.roll_values {
            result += &value.to_string();
            result += ", ";
        }
        // remove trailing ','
        result.pop();
        result.pop();
        result += ")";
        result
    }
}

#[command]
#[bucket = "basic"]
pub fn roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let input = args.rest().to_string();
    // TODO get this only once and pass it to the functions
    //      each call to env would be expensive

    if input.contains(|c| (c == '.' || c == '(' || c == ')' || c == '^')) {
    } else {
        let result = create_result(&input);
        let result = match result {
            Some(r) => r,
            None => return Ok(()),
        };
        let roll_result_string = create_result_string(&result);
        let msg = msg.channel_id.send_message(&ctx.http, |m| {
            m.content(format!("Your roll result was {}", sum_rolls(&result)));
            m.embed(|e| {
                e.title("Detailed Result:");
                e.description("See how the result was generated");
                e.fields(vec![("Rolls:", roll_result_string, false)]);
                e
            });
            m
        });
        if let Err(why) = msg {
            error!("Error sending message: {:?}", why);
            return Err(CommandError::from(why));
        }
    }
    Ok(())
}

fn sum_rolls<'a>(result: &(Vec<Roll>, Vec<&'a str>)) -> u32 {
    let mut rolls_summed: u32 = result.0[0].roll_result;
    for (i, operator) in result.1.iter().enumerate() {
        match operator.trim() {
            "+" => rolls_summed += result.0[i + 1].roll_result,
            "-" => rolls_summed -= result.0[i + 1].roll_result,
            "/" => rolls_summed /= result.0[i + 1].roll_result,
            "*" => rolls_summed *= result.0[i + 1].roll_result,
            _ => (),
        };
    }
    rolls_summed
}

fn create_result_string(result: &(Vec<Roll>, Vec<&str>)) -> String {
    let rolls = &result.0;
    let operators = &result.1;
    let mut roll_result_string = String::new();
    for (n, roll) in rolls.iter().enumerate() {
        roll_result_string += &roll.generate_roll_result_string();
        roll_result_string += " ";
        if n < operators.len() {
            roll_result_string += &operators[n];
            roll_result_string += " ";
        }
    }
    roll_result_string
}

fn create_result<'a>(input: &'a str) -> Option<(Vec<Roll>, Vec<&'a str>)> {
    let operations: Vec<&str> = input
        .matches(|c| (c == '+' || c == '-' || c == '*' || c == '/'))
        .collect();
    let input_split: Vec<&str> = input
        .split(|c| c == '+' || c == '-' || c == '/' || c == '*')
        .collect();
    if operations.len() != input_split.len() - 1 {
        error!(
            "Operator length missmatch!\nOperator length: {} \nToken length: {}",
            operations.len().to_string(),
            input_split.len().to_string()
        );
        return None;
    }
    let mut rolls: Vec<Roll> = vec![];
    for token in input_split {
        let token = token.trim();
        //if token.contains(|c| (c == 'd' || c == 'D' || c == 'w' || c == 'W')) {
        if token
            .chars()
            .all(|c| (c == 'd' || c == 'D' || c == 'w' || c == 'W' || c.is_numeric()))
            && token.contains(|c| (c == 'd' || c == 'D' || c == 'w' || c == 'W'))
        {
            let roll = match evaluate_roll(Roll {
                roll_string: token.to_string(),
                roll_result: 0,
                roll_values: vec![],
            }) {
                Some(roll) => roll,
                None => return None,
            };
            rolls.push(roll);
        } else if token.parse::<u32>().is_ok() {
            let value = token.trim().parse::<u32>().unwrap(); // should be okay since is_okay check
            rolls.push(Roll {
                roll_string: token.to_string(),
                roll_values: vec![value],
                roll_result: value,
            });
        } else {
            error!("Not matching token in input!");
            return None;
        }
    }
    Some((rolls, operations))
}

fn evaluate_roll(mut roll: Roll) -> Option<Roll> {
    let re = Regex::new(r"(\d{1,3})[dDwW](\d{1,3})").unwrap();
    let m = re.captures(&roll.roll_string.trim()).unwrap();
    roll.roll_values = match roll_dice(
        m.get(1)
            .unwrap()
            .as_str()
            .trim()
            .parse()
            .expect("Not a number"),
        m.get(2).unwrap().as_str().parse().expect("not a Number"),
    ) {
        Some(r) => r,
        None => return None,
    };
    roll.calculate_roll_result();
    Some(roll)
}

//replace rand function with a deterministic one for tests
cfg_if::cfg_if! {
    if #[cfg(test)] {
        fn roll_dice(amount: u32 , eyes: u32) -> Option<Vec<u32>> {
            let mut results = vec![];
            if amount > 64 ||  eyes > 64 {
                return None;
            }
            for _ in 0..amount {
                let result = eyes / 2; // high end of range is exclusive!
                results.push(result);
            }
            Some(results)
        }
    } else {
        fn roll_dice(amount: u32, eyes: u32) -> Option<Vec<u32>> {
        let mut results = vec![];
        if amount > 64 ||  eyes > 64 {
            return None;
        }
        for _ in 0..amount {
            let result = rand::thread_rng().gen_range(1, eyes + 1); // high end of range is exclusive!
            results.push(result);
        }
        Some(results)
        }
    }
}

#[test]
#[ignore]
fn test_roll_interactive() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Line could not be read!");
    let test = create_result(&input);
    let test = match test {
        Some(r) => r,
        None => return,
    };
    let value = sum_rolls(&test);
    let result = create_result_string(&test);
    println!("Value {}", value);
    println!("Result {}", result);
}

#[test]
fn test_roll() {
    let input = vec!["2w10", "5W6 + 3", "3 + 4D6", "2d10+5"];
    let output: Vec<u32> = vec![10, 18, 15, 15];
    for (i, token) in input.iter().enumerate() {
        let result = match create_result(token) {
            Some(r) => r,
            None => panic!("Create result failed in test {}", token),
        };
        let value = sum_rolls(&result);
        if value != output[i] {
            panic!("Wrong calculation");
        }
    }
}
#[test]
fn test_should_fail() {
    let input = vec!["2.5w10", "(5+4)W6 + 3", "3 + 4^3D6", "2d100+5"];

    for token in input {
        match create_result(token) {
            Some(r) => panic!("Returned value {:?} for invalid input {}", r, token),
            None => (),
        };
    }
}
