use clap::{App, Arg};
use ncurses::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::io::{self};

const QUESTION_PATH: &'static str = "questions.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Question {
    pub text: String,
    pub code: Option<String>,
    pub answer: bool,
}

fn load_questions(path: &str) -> io::Result<Vec<Question>> {
    let contents = std::fs::read_to_string(path)?;

    let questions: Vec<Question> = serde_json::from_str(&contents)?;

    println!("Loaded {} questions", questions.len());

    Ok(questions)
}

fn read_string(prompt: &str) -> io::Result<String> {
    use std::io::{stdin, stdout};

    let mut s = String::new();
    print!("{}: ", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    println!("{}", s);

    Ok(s)
}

fn main() -> io::Result<()> {
    let matches = App::new("Question randomizer")
        .version("1.0")
        .arg(
            Arg::with_name("input")
                .short("i")
                .help("Starts the application in input mode")
                .takes_value(false),
        )
        .get_matches();

    let mut questions: Vec<Question> = load_questions(QUESTION_PATH)?;

    if matches.is_present("input") {
        loop {
            let text = read_string("Text")?;

            if text.trim().is_empty() {
                break;
            }

            let mut code = String::new();

            print!("Code : ");
            let _ = std::io::stdout().flush();
            while !code.contains("@") {
                let mut buffer = String::new();

                io::stdin().read_line(&mut buffer)?;

                code.push_str(&buffer);
            }

            code = code.replace("@", "");

            let answer = read_string("Answer")?;

            let mut code_option = None;

            if !code.trim().is_empty() {
                code_option = Some(code.trim().to_string());
            }

            let question = Question {
                text,
                code: code_option,
                answer: answer.to_lowercase().contains("true"),
            };

            questions.push(question);

            let question_text = serde_json::to_string(&questions).unwrap();

            File::create(QUESTION_PATH)?.write_all(question_text.as_bytes())?;
        }

        return Ok(());
    }

    questions.shuffle(&mut thread_rng());

    initscr();

    let mut correct: u64 = 0;
    let mut incorrect: u64 = 0;

    for (i, question) in questions.iter().enumerate() {
        clear();

        let percent: f32 = i as f32 / questions.len() as f32;
        let width: u32 = (percent * 80.0) as u32;
        let width_left: u32 = 80 - width;

        addstr(&(0..width).map(|_| "#").collect::<String>());
        addstr(&(0..width_left).map(|_| "-").collect::<String>());
        addstr("\n");

        addstr(&format!("{}\n", question.text));

        if question.code.is_some() {
            addstr(&format!("\n{}\n", question.code.as_ref().unwrap()));
        }

        refresh();

        let answer = getch() == 't' as i32;

        if answer == question.answer {
            correct += 1;
        } else {
            incorrect += 1;

            addstr(&format!(
                "\nWRONG: Answer was {}, but you picked {}\n",
                question.answer, answer
            ));
            getch();

            getch();
        }
    }

    endwin();

    println!("Finished all questions!");
    println!("Final score: {}/{}", correct, correct + incorrect);
    println!(
        "Thats a score of {}%!!!",
        correct as f32 / (correct + incorrect) as f32 * 100.0
    );

    Ok(())
}
