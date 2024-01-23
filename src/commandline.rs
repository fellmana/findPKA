use clap::Parser;

#[derive(Parser)]
pub struct Arguments {
    #[arg(help = "input lammps data file", required = true)]
    pub filename: String,

    #[arg(short, help = "number of PKAs", default_value_t = 1)]
    pub n: usize,

    #[arg(
        short,
        help = "tolerance to find PKA (Å).",
        long,
        default_value_t = 5.0
    )]
    pub tolerance: f64,

    #[arg(long, help = "seed random number generator.", default_value_t = 1234)]
    pub seed: u64,

    #[arg(short, help = "PKA distance from center (Å).", long, required = true)]
    pub rPKA: f64,

    #[arg(short, long, help = "PKA energy (eV).", required = true)]
    pub energy: f64,

    #[arg(short, long, value_parser,
          help="Map elements ('Al,W...') in same order as type in data file",
          required=true,
          num_args = 1..)]
    pub map: Vec<String>,

    #[arg(long,value_parser,
        help="Give masses in same order as in element mapping (if not given mass infered)",
        required=false,
        num_args = 1..)]
    pub mass: Vec<f64>,

    #[arg(
        short,
        long,
        help = "verbose output",
        default_value_t = false,
        required = false
    )]
    pub verbose: bool,

    #[arg(
        short,
        long,
        help = "print lammps ready string",
        default_value_t = false,
        required = false
    )]
    pub lammps: bool,
}
