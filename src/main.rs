use structopt::StructOpt;

#[derive(StructOpt)]
struct CLI {
    city: String,
    country_code: String,
}

fn main() {
    let args = CLI::from_args();

    println!("Getting weather for country : {} code : {}", args.city, args.country_code);
}
