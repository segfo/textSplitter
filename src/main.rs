use std::fs::File;
use std::io::{BufRead,BufReader,Read,BufWriter,Write};
use std::{env,process};

trait BaseNConvert{
    fn base36(self)->String;
}

fn fileSizeBaseSplitter(args:&Vec<String>)->Result<(),std::io::Error>{
    let fs=File::open(&args[0])?;
    let mut fs=BufReader::new(fs);
    let maxBytes=(*args[1]).parse::<usize>().unwrap_or(1*1024*1024);
    let mut idx=0;
    'newFile: loop{
        let mut readBytes=0;
        let ofileName=format!("{}{}",args[2],idx);
        let ofs = File::create(&ofileName)?;
        let mut ofs = BufWriter::new(ofs);
        for line in fs.by_ref().lines().map(|l| l.unwrap()){
            ofs.write(&*line.as_bytes());
            ofs.write("\n".as_bytes());
            readBytes+=line.len();
            if readBytes>maxBytes{
                idx+=1;
                continue 'newFile;
            }
        }
        break;
    }
    Ok(())
}

fn main() {
    let prog:String=env::args().next().unwrap_or("".to_owned());
    let args:Vec<String> = env::args().skip(1).collect();
    if args.len()<3{
        println!("{} <file> <bytes> <file prefix>",prog);
        process::exit(1);
    }
    fileSizeBaseSplitter(&args).unwrap_or_else(|e|{println!("{}",e)});
}