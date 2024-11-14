use core::time;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use std::{collections::HashMap, thread::sleep};

fn main() -> Result<(), std::io::Error> {
    let numbers: HashMap<String, Box<[&str]>> = HashMap::from([
        (
            "0".to_string(),
            Box::from([" -- ", "|  |", "|  |", "|  |", " -- "]),
        ),
        (
            "1".to_string(),
            Box::from(["    ", "   |", "   |", "   |", "    "]),
        ),
        (
            "2".to_string(),
            Box::from([" -- ", "   |", " -- ", "|   ", " -- "]),
        ),
        (
            "3".to_string(),
            Box::from([" -- ", "   |", " -- ", "   |", " -- "]),
        ),
        (
            "4".to_string(),
            Box::from(["    ", "|  |", " -- ", "   |", "    "]),
        ),
        (
            "5".to_string(),
            Box::from([" -- ", "|   ", " -- ", "   |", " -- "]),
        ),
        (
            "6".to_string(),
            Box::from([" -- ", "|   ", " -- ", "|  |", " -- "]),
        ),
        (
            "7".to_string(),
            Box::from([" -- ", "   |", "    ", "   |", "    "]),
        ),
        (
            "8".to_string(),
            Box::from([" -- ", "|  |", " -- ", "|  |", " -- "]),
        ),
        (
            "9".to_string(),
            Box::from([" -- ", "|  |", " -- ", "   |", "    "]),
        ),
    ]);

    let mut seconds_left = 30;

    'clock: loop {
        execute!(
            std::io::stdout(),
            Hide,
            Clear(Purge),
            Clear(All),
            DisableLineWrap,
            MoveTo(0, 0)
        )?;

        let mut times = [
            (seconds_left / 3600).to_string(),
            (seconds_left % 3600 / 60).to_string(),
            (seconds_left % 60).to_string(),
        ];

        for i in 0..5 {
            let mut line = String::new();
            for time in &mut times {
                if time.len() == 1 {
                    time.insert_str(0, "0");
                }
                for char in time.chars() {
                    let num = numbers.get(&char.to_string()).unwrap()[i];
                    line.push_str(num);
                    line.push_str(" ");
                }
                if (i == 1 || i == 3) && line.len() <= 26 {
                    line.push_str(" * ");
                } else {
                    line.push_str("   ");
                }
            }

            println!("{}", line);
        }
        if seconds_left == 0 {
            println!("       * * * * BOOM * * * *");
            break 'clock;
        }
        println!("Press Ctrl-C to quit.");
        sleep(time::Duration::from_secs(1));
        seconds_left -= 1;
    }

    Ok(())
}
