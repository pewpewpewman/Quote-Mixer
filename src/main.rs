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

    let args : Vec<String> = std::env::args().collect();

    if (args.len() < 3)
    { panic!("At least 2 strings are needed for mixing"); }

    let from : &str = &args[1];
    let into : &str = &args[2];
    let insertion_seperator : &str =
        if (args.len() < 4) { "`" }
        else { &args[3] };
    
    if (args.contains(&String::from("-l")))
    {
        let lcs : String = lcs(from, into);
        println!("{lcs}");
    }
    else
    {
        let mixed : String = mix_quotes(from, into, insertion_seperator);
        println!("{mixed}");
    }

    Ok(())
}


