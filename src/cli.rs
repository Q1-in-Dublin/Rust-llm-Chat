//Command Line Argument Parser

#[derive(clap::Parser)]
pub struct Cli{

    // Question -> answer and exit
    pub prompt: Option<String>,

    //Model 
    #[arg(long)]
    pub model : Option<String>,

    //print Token usage
    #[arg(long)]
    pub usage: bool,
}