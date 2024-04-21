use anyhow::Result;
use rcli::{process_csv, Args, SubCommand};

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);
    match args.cmd {
        Some(SubCommand::CSV(csvopts)) => process_csv(&csvopts.input, &csvopts.output)?,
        None => println!("nothing"),
    }

    Ok(())
}

// fn process_csv(filename: &str) {
//     let file =  File::open(filename).expect("filename can not open");
//     let mut rdr = csv::Reader::from_reader(file);
//     {
//         let headers = rdr.headers().expect("except headers wrong!");
//         println!("headers: {:?}", headers);
//     }
//     for result in rdr.records() {
//         let record = result.expect("a CSV record");
//         println!("{:?}", record);
//     }
// }
