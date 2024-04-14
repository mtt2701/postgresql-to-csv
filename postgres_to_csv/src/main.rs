use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use clap::Parser;

#[derive(Parser)]
#[clap(about)]
struct Cli {
    // The path to the input file
    input: std::path::PathBuf,
    // The path to the output file
    output: std::path::PathBuf,
    // Include header or not
    #[clap(long, short, action = clap::ArgAction::SetTrue)]
    add_header: bool,
    // Delimiter of input file, default "\t"
    #[clap(long, short, default_value = "\t")]
    input_delimiter: String,
    // Delimiter of output file, default ","
    #[clap(long, short, default_value = ",")]
    output_delimiter: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    // Open input and output files
    let input_file = File::open(args.input)?;
    let mut output_file = File::create(args.output)?;

    // Iterate through the PostgreSQL dump file
    let reader = BufReader::new(input_file);
    let mut in_copy = false;
    for read_line in reader.lines() {
        let line = read_line?;

        // '\.' indicates the end of a COPY block
        if line.starts_with("\\.") {
            in_copy = false;
        }

        // Process the line if it is in a COPY block
        if in_copy {
            let line = line.replace(&args.input_delimiter, &args.output_delimiter);
            let line = line.replace("\\N", "");
            writeln!(output_file, "{}", line)?;
        }
        
        // 'COPY' indicates the start of a COPY block
        if line.starts_with("COPY") {
            in_copy = true;
            if args.add_header {
                // Variable names are enclosed by "()"
                let start = match line.find('(') {
                    Some(index) => index,
                    None => continue,
                };
                let end = match line.find(')') {
                    Some(index) => index,
                    None => continue,
                };
                
                // Process the line if "()" is found
                if start < end {
                    let line = &line[start + 1 .. end];
                    let line = line.replace(" ", "");
                    writeln!(output_file, "{}", line)?;
                }
            }
        }
    }

    Ok(())
}