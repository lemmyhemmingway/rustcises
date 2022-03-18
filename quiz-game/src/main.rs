use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, default_value_t = 30)]
    limit: u32,
    #[clap(short, long, default_value = "problems.csv")]
    filename: String
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args.limit);
    println!("{:?}", args.filename);

    /*let mut reader = csv::Reader::from_path("./src/problems.csv")
                                                .expect("a csv file");
    for result in reader.records() {
        println!("{:?}", result)
    }*/
}
