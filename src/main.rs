use flate2::write::GzEncoder; //Compresses into gzip format
use flate2::Compression; // An Enum of Compression Levels
use std::env::args; // To Access CLI Arguements
use std::fs::File; // To Access Files
use std::io::copy; // Copy Data from One I/O to Other
use std::io::BufReader; // Wrapper for File or Network Streams to Streamline
use std::time::Instant; // To measure Time Taken


fn main(){
    println!("Hello World");
}