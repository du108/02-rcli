use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Args, Base64SubCommand,
    SubCommand,
};

fn main() -> Result<()> {
    let args = Args::parse();
    // println!("{:?}", args);
    match args.cmd {
        SubCommand::CSV(csvopts) => {
            let output = if let Some(output) = csvopts.output {
                output
            } else {
                format!("format.{}", csvopts.format)
            };
            process_csv(&csvopts.input, output, Into::<&str>::into(csvopts.format))?;
        }
        SubCommand::GenPass(genopts) => {
            process_genpass(
                genopts.length,
                genopts.uppercase,
                genopts.lowercase,
                genopts.numbers,
                genopts.symbols,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(encode) => {
                process_encode(&encode.input, encode.format)?;
                println!("{:?}", encode.format);
            }
            Base64SubCommand::Decode(decode) => {
                process_decode(&decode.input, decode.format)?;
                println!("{:?}", decode.format);
            }
        },
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
