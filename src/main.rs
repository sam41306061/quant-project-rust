use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::{Duration, UNIX_EPOCH};
use tokio_test;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::time::OffsetDateTime;
use yahoo_finance_api::Quote;

fn write_to_csv(data: &Vec<Quote>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(Path::new(filename))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Timestamp,Open,High,Low,Close,Volume")?;

    for quote in data {
        writeln!(
            writer,
            "{},{},{},{},{},{}",
            quote.timestamp, quote.open, quote.high, quote.low, quote.close, quote.volume
        )?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = yahoo::YahooConnector::new()?;
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("ABBV", "1mo"))?;
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote()?;
    let time: OffsetDateTime =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));

    // Write the data to a CSV file
    write_to_csv(&vec![quote], "abbvie_quotes.csv")?;

    println!("Data written to abbvie_quotes.csv");

    Ok(())
}
