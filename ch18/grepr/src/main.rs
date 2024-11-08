use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::{Arg, Command};


fn grep<R>(target: &str, reader: R) -> io::Result<()>
where R: BufRead
{
    reader.lines().try_for_each(|line_result| {
        let line = line_result?;
        line.contains(target).then(|| println!("{}", line));

        Ok(())
    })
}

fn grep_main() -> Result<(), anyhow::Error> {
    let matches = Command::new("grepr")
    .version("0.1.0")
    .about("'프로그래밍 러스트'의 예제 코드")
    .arg(
        Arg::new("pattern")
            .value_name("PATTERN")
            .num_args(1)
            .required(true)
    )
    .arg(
        Arg::new("file")
            .value_name("FILE")
            .num_args(0..)
            .required(false)
    )
    .get_matches();

    let target: String = matches.get_one("pattern").cloned().ok_or(anyhow::anyhow!("NO pattern!"))?;

    match matches.get_many("file") {
        None => grep(&target, io::stdin().lock())?,
        Some(files) => {
            let files  = files.cloned().collect::<Vec<String>>();
            files.into_iter().try_for_each(|file| {
                let f = File::open(file)?;
                grep(&target, BufReader::new(f))})?;
        }
    }

    Ok(())
}

fn main() {
    let result = grep_main();
    if let Err(err) = result {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
