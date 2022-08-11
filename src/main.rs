use std::fs;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args{
    #[clap(short, long)]
    loc: String,
}

fn main() {
    let args = Args::parse();
    if let Ok(entries) = fs::read_dir(args.loc){
        for entry in entries{
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata(){
                    let mut path = entry.path().into_os_string().into_string().unwrap();
                    let chars: Vec<char> = path.chars().collect();
                    for i in chars{
                        if i == '.' || i == '/'{
                            path.remove(0);
                        }
                    }
                    println!("{:?}: {:?}", path, metadata.permissions());
                } else {
                    println!("Error from {:?}", entry.path());
                }
            }
        }
    }
}
