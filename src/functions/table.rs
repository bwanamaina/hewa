use crate::weather::Forecast;

use term_table::row::Row;
use term_table::table_cell::Alignment;
use term_table::table_cell::TableCell;
use term_table::Table;
use term_table::TableStyle;

pub fn draw_table(response: Forecast) {
    let mut table = Table::new();

    table.max_column_width = 50;

    table.style = TableStyle::extended();

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!(
            "Current Weather For {} {}",
            response.name, response.sys.country
        ),
        2,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new("Timezone"),
        TableCell::new_with_alignment(response.timezone, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Sunrise"),
        TableCell::new_with_alignment(response.sys.sunrise, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Sunset"),
        TableCell::new_with_alignment(response.sys.sunset, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Wind Speed"),
        TableCell::new_with_alignment(response.wind.speed, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Wind Deg"),
        TableCell::new_with_alignment(response.wind.deg, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Wind Gust"),
        TableCell::new_with_alignment(response.wind.gust, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Temperature"),
        TableCell::new_with_alignment(response.main.temp, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Humidity"),
        TableCell::new_with_alignment(response.main.humidity, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Pressure"),
        TableCell::new_with_alignment(response.main.pressure, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Maximum Temperature"),
        TableCell::new_with_alignment(response.main.temp_max, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Minimum Temperature"),
        TableCell::new_with_alignment(response.main.temp_min, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Feels Like"),
        TableCell::new_with_alignment(response.main.feels_like, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Latitude"),
        TableCell::new_with_alignment(response.coord.lat, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Longitude"),
        TableCell::new_with_alignment(response.coord.lon, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new(response.weather.details.main),
        TableCell::new_with_alignment(response.weather.details.description, 1, Alignment::Right),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new("Visibility"),
        TableCell::new_with_alignment(response.visibility, 1, Alignment::Right),
    ]));

    println!("{}", table.render());
}
