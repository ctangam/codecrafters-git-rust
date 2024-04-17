#[allow(unused_imports)]
use std::env;
use std::ffi::CStr;
#[allow(unused_imports)]
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;
use std::process::Stdio;

use clap::Parser;
use flate2::bufread::ZlibDecoder;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,

        path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory")
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            let content = fs::read(format!(
                ".git/objects/{}/{}",
                &object_hash[..2],
                &object_hash[2..]
            ))
            .unwrap();
            let z = ZlibDecoder::new(&content[..]);
            let mut z = BufReader::new(z);
            let mut buf = Vec::new();
            z.read_until(0, &mut buf).unwrap();
            let header = CStr::from_bytes_with_nul(&buf).unwrap();
            let header = header.to_str().unwrap();
            let Some((kind, size)) = header.split_once(' ') else {
                panic!("wrong header: {}", header)
            };
            let size = size.parse::<u64>().unwrap();
            let mut z = z.take(size);

            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let n = std::io::copy(&mut z, &mut stdout).unwrap();
        }
        Command::HashObject { 
            write, 
            path 
        } => {
            
        }
    }

    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     fs::create_dir(".git").unwrap();
    //     fs::create_dir(".git/objects").unwrap();
    //     fs::create_dir(".git/refs").unwrap();
    //     fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    //     println!("Initialized git directory")
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
}
