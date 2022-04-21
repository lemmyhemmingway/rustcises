use std::io::{stdin, stdout, Write};
use clap::Parser;
use csv::{Reader, StringRecord};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, default_value_t = 30)]
    limit: u32,
    #[clap(short, long, default_value = "problems.csv")]
    filename: String,
}

#[derive(Debug)]
struct Question {
    question: String,
    answer: String,
}


fn main() {
    let args = Args::parse();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(args.filename)
        .unwrap();

    let mut questions: Vec<Question> = vec![];

    for record in reader.records() {
        let mut question = Question{ question: String::new(), answer: String::new()};
        match record.as_ref().unwrap().get(0) {
            Some(value) => question.question = value.to_string(),
            None => ()
        }
        match record.as_ref().unwrap().get(1) {
            Some(value) => question.answer = value.to_string(),
            None => ()
        }
        questions.push(question);
    }


    let mut total = 0;
    for question in &questions {
        let mut answer = String::new();
        print!("{} = ", question.question);
        stdout().flush();
        stdin().read_line(&mut answer).unwrap();

        if answer.trim() == question.answer {
            total += 1;
        }

    }

    println!("Correct Answer: {} Wrong Answer: {}", total, questions.len() - total);

}
