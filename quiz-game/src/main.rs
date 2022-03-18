
fn main() {
    let mut reader = csv::Reader::from_path("./src/problems.csv")
                                                .expect("a csv file");
    for result in reader.records() {
        println!("{:?}", result)
    }
}