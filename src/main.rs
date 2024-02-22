use flate2::write::GzEncoder; //Compresses into gzip format
use flate2::Compression; // An Enum of Compression Levels
use std::env::args; // To Access CLI Arguements
use std::fs::File; // To Access Files
use std::io::copy; // Copy Data from One I/O to Other
use std::io::BufReader;// Wrapper for File or Network Streams to Streamline
use std::time::Instant; // To measure Time Taken

fn main() {
    // We must get 2 inputs in CLI (One is the path to Original File and Second is the Target Location)
    if args().len() != 3 {
        // eprintln! to print to error panel
        eprintln!("Invalid Input\nExpected Use: 'Source' 'Target'");
        return;
    }
    // Read Input File
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    // Create a Empty Output File
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    // Starting the Timer
    let timer = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:#?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:#?}", output.metadata().unwrap().len());
    println!("Time Elapsed: {:#?}", timer.elapsed());
}
