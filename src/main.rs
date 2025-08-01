#!
[
    allow
    (
        unused_variables,
        unused_parens
    )
]

fn main() -> Result<(), std::io::Error>
{

    use quote_mixer::{lcs, mix_quotes};

    let mut args : &[String] = &std::env::args().collect::<Vec<String>>();

    if (args.len() <= 3)
    { panic!("At least 3 strings are needed for mixing"); }

    let from : &str = &args[1];
    let into : &str = &args[2];
    let insertion_seperator : &str = &args[3];
    
    args = &args[4..]; //Forget about first arguments
    let only_lcs : bool = args.contains(&String::from("-l")) || args.contains(&String::from("--lcs"));
    let use_files : bool = args.contains(&String::from("-f")) || args.contains(&String::from("--from-files"));

    if (use_files)
    {
        use std::fs;
        let from : String = fs::read_to_string(from)?;
        let into : String = fs::read_to_string(into)?;

        if (only_lcs)
        {
            let lcs : String = lcs(&from, &into);
            println!("{lcs}");
        }
        else
        {
            let mixed : String = mix_quotes(&from, &into, insertion_seperator);
            println!("{mixed}");
        }
    }
    else
    {
        if (only_lcs)
        {
            let lcs : String = lcs(from, into);
            println!("{lcs}");
        }
        else
        {
            let mixed : String = mix_quotes(from, into, insertion_seperator);
            println!("{mixed}");
        }
    }

    Ok(())
}


