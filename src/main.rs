use alphavantage::time_series::TimeSeries;
use std::path::Path;
// path to correct api key
let path = Path::new("./env");
// path to open key in the api
let mut file = File::open(path).unwrap();

// set data value coming for get_daily value
let ts = TimeSeries::Daily;
// set your data columns

// all the tickers I would like to grab data from
// close price on the day

// loop to go through all tickers
    // set a delay of 5 seconds between calls to the api
    // cover the Timeseries to JSON format or something else for CSV export?
    // get data from the get_intraday function
    // set intraday columns for export

    // validation check for empty data rows 
        // new valriable to drop empty 
        // remove empty rows 
        // print messeage 

    // gather close price per ticker 
    // print successful fetch 

    // Execption failure message for failed data fetch 

// export data to csv file 

// print success message

fn main() {
    println!("Data has been written to alpha_vantage_close_prices.csv")
}
