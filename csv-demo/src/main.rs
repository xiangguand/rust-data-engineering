use std::{error::Error, io, process};
use csv::Writer;

fn example_read_txt() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn example_read_csv() -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(io::stdout());
    let fruits = [
        ("Apple", 1.25),
        ("Orange", 0.99),
        ("Banana", 0.79),
        ("Pear", 0.89),
        ("Kiwi", 1.99),
        ("Lemon", 0.99),
        ("Lime", 0.89),
    ];

    let mut wtr = Writer::from_path("data/output.csv")?;
    wtr.write_record(&["Fruit", "Price"])?;
    for (fruit, price) in fruits {
        wtr.write_record([fruit, &price.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}

fn main() {
    if let Err(err) = example_read_txt() {
        println!("{}", err);
        process::exit(1);
    }

    if let Err(err) = example_read_csv() {
        println!("{}", err);
        process::exit(1);
    }
}
