use rand::Rng;
use regex::Regex;
use log::{error};
use serenity::framework::standard::{macros::command, Args, CommandError, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;


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
        result.pop();
        result.pop();
        result += ")";
        result
    }
}

#[command]
pub fn roll(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let input = args.rest().to_string();
    // TODO get this only once and pass it to the functions
    //      each call to env would be expensive
    let re = Regex::new(r"(\d{1,10}\s*[wWdD]?\d{0,10})\s*([+\-2/\*]?)").unwrap();
    //print!("Rolling {}\n", input);
    let result = create_result(re, input);
    let rolls = &result.0;
    let operators = &result.1;
    let mut roll_result_string = String::new();
    for (n,roll) in rolls.iter().enumerate() {
        roll_result_string += &roll.generate_roll_result_string();
        roll_result_string += " ";
        if n < operators.len() {
            roll_result_string += &operators[n];
            roll_result_string += " ";
        }
    }
    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.content(format!("Your roll result was {}", sum_rolls(&result)));
        m.embed( |e| {
            e.title("Detailed Result:");
            e.description("See how the result was generated");
            e.fields(vec![
                ("Rolls:", roll_result_string, false),
            ]);
            e
        });
        m
    });
    if let Err(why) = msg {
        error!("Error sending message: {:?}", why);
        return Err(CommandError::from(why));
    }
    Ok(())
    
}

fn sum_rolls(result: &(Vec<Roll>, Vec<String>)) -> u32 {
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

fn create_result(re: Regex, input: String) -> (Vec<Roll>, Vec<String>) {
    let mut operations = vec![];
    let mut rolls = vec![];
    for token in re.captures_iter(&input.trim()) {
        match &token[2] {
            "" => (),
            "+" | "-" | "/" | "*" => operations.push(token[2].to_string()),
            _ => continue,
        };
        let roll = Roll {
            roll_string: token[1].trim().to_string(),
            roll_values: vec![],
            roll_result: 0,
        };
        let dice_in_roll = Regex::new(r"[dDwW]").unwrap();
        if !dice_in_roll.is_match(&token[1]) {
            rolls.push(Roll {
                roll_string: token[1].to_string(),
                roll_values: vec![token[1].parse().expect("no number")],
                roll_result: token[1].parse().expect("no number"),
            });
        } else {
            rolls.push(evaluate_roll(roll));
        }
    }
    return (rolls, operations);
}

fn evaluate_roll(mut roll: Roll) -> Roll {
    let re = Regex::new(r"(\d{1,10})[dDwW](\d{1,10})").unwrap();
    let m = re.captures(&roll.roll_string.trim()).unwrap();
    roll.roll_values = roll_dice(
        m.get(1)
            .unwrap()
            .as_str()
            .trim()
            .parse()
            .expect("Not a number"),
        m.get(2).unwrap().as_str().parse().expect("not a Number"),
    );
    roll.calculate_roll_result();
    roll
}

fn roll_dice(amount: u32, eyes: u32) -> Vec<u32> {
    let mut results = vec![];
    for _ in 0..amount {
        let result = rand::thread_rng().gen_range(1, eyes + 1); // high end of range is exclusive!
        results.push(result);
    }
    results
}
