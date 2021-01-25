use clap::{App, Arg, SubCommand};
use ncurses::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead, Read};

const QUESTION_PATH: &'static str = "questions.json";

#[derive(Debug, Serialize, Deserialize)]
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

    let mut questions = load_questions(QUESTION_PATH)?;

    questions.shuffle(&mut thread_rng());

    initscr();

    let mut correct: u64 = 0;
    let mut incorrect: u64 = 0;

    for (i, question) in questions.iter().enumerate() {
        clear();

        let percent: f32 = i as f32 / questions.len() as f32;
        let width: u32 = (percent * 40.0) as u32;
        let width_left: u32 = ((1.0 - percent) * 40.0) as u32;

        addstr(&(0..width).map(|_| "#").collect::<String>());
        addstr(&(0..width_left).map(|_| "-").collect::<String>());
        addstr("\n");

        addstr(&format!("{}\n", question.text));

        if question.code.is_some() {
            addstr(&format!("\n{}\n", question.code.as_ref().unwrap()));
        }

        refresh();

        let answer = getch() == 'f' as i32;

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
        correct as f32 / (correct + incorrect) as f32
    );

    Ok(())
}
