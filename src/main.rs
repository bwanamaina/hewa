#[path = "functions/table.rs"]
mod table;

#[path = "structs/weather.rs"]
mod weather;

use dotenv_codegen::dotenv;

use exitfailure::ExitFailure;
use reqwest::Url;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    city: String,
    country_code: String,
}

impl weather::Forecast {
    async fn get(city: &str, country_code: &str) -> Result<Self, ExitFailure> {
        let api_key = dotenv!("WEATHER_API_KEY");

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}",
            city, country_code, api_key
        );

        let url = Url::parse(&*url)?;

        let response = reqwest::get(url).await?.json::<weather::Forecast>().await?;

        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let response = weather::Forecast::get(&args.city, &args.country_code).await?;

    table::draw_table(response);

    Ok(())
}
