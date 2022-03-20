use std::fs::File;
use clap::Parser;
use csv::{Reader, StringRecord};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, default_value_t = 30)]
    limit: u32,
    #[clap(short, long, default_value = "problems.csv")]
    filename: String
}
#[derive(Debug)]
struct Problem {
    question: *str,
    answer: *str
}

fn main() {
    let args = Args::parse();

    let mut reader = csv::Reader::from_path(args.filename)
                                                .expect("a csv file");

    let mut problems: Vec<Problem>;
    for result in reader.records() {
        let record = result?;
        let mut problem = Problem{
            question: &record[0],
            answer: &record[1]
        };
        println!("{:?}", problem);
        problems.append(*problem)
    }

}

