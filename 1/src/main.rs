use rprompt;
use std::io;

fn main() -> io::Result<()> {
    let contents: Vec<_> = std::include_str!("../input").split("\n").collect();

    let entries: usize = rprompt::prompt_reply_stdout("Number of entries: ")?
        .parse()
        .unwrap();

    let locations: Vec<usize> = vec![0; entries];

    loop {
        // Check if any of the numbers in locations are the same
        if (1..locations.len()).any(|i| locations[i..].contains(&locations[i - 1])) {
            print!("Yo");
        }
        // if locations.iter().all(|location|)
        break;
    }

    Ok(())
}
