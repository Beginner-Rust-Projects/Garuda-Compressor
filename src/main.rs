use flate2::write::GzEncoder; //Compresses into gzip format
use flate2::Compression; // An Enum of Compression Levels
use std::env::args; // To Access CLI Arguements
use std::fs::File; // To Access Files
use std::io::copy; // Copy Data from One I/O to Other
use std::io::BufReader;// Wrapper for File or Network Streams to Streamline
use std::time::Instant; // To measure Time Taken

fn main() {
    if args().len() != 3 {
        eprintln!("Invalid Input\nExpected Use: 'Source' 'Target'");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
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
