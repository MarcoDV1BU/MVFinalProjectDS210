use std::error::Error;
use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;


//Sets up loading in the database
#[derive(Debug, Deserialize)]
struct BitcoinData {
    Date: String,
    Open: f64,
    High: f64,
    Low: f64,
    Close: f64,
    Volume: f64,
    Fear_Greed_Index: i64,
    Sentiment: String,
}


//Contains all variables I will be finding and assigns them as floats since they have decimals and so I can preform operations with them
#[derive(Debug)]
struct InvestmentResult {
    total_invested: f64,
    final_value: f64,
    pct_yield: f64,
    days_invested: f64,
    total_bct: f64,
    profit: f64,
    needed_investment_10k: f64,
    avg_buyin: f64,
    multi: f64,
    needed_investment_5x: f64,
}


//This is where I run the calculations that get returned for each way of investing
fn calc_type<F>(records: &[BitcoinData], filter: F, final_price: f64, investment: f64, every_other: bool) -> InvestmentResult
where
    F: Fn(&BitcoinData) -> bool,
{
    let mut total_bct = 0.0;
    let mut total_invested = 0.0;
    let mut days_invested = 0.0;
    let mut total_open = 0.0;
    let mut day_count = 0;


// This for loop goes through every day and adds a counter. Mostly important for the $10 every other day investment
    for record in records {
        let invest = if every_other {
            day_count % 2 == 0
        } else {
            true
        };
        if invest && filter(record) {
            let bought = investment / record.Open;
            total_bct += bought;
            total_invested += investment;
            days_invested += 1.0;
            total_open += record.Open;
        }
        day_count += 1;
    }


    let final_value = total_bct * final_price;
    let pct_yield = if total_invested > 0.0 {
        ((final_value - total_invested)
        / total_invested) * 100.0
    } else {
        0.0
    };
    let desired_profit = 10000.0;
    let profit = final_value - total_invested;
    let needed_multi = desired_profit
        / profit;
    let desired_multi = 5.0;
    let multi = final_value
        / total_invested;
    let multi_dif = desired_multi
        / multi;
    let needed_investment_5x = investment * multi_dif;
    let needed_investment_10k = needed_multi * investment;
    let avg_buyin = total_open
        / days_invested;


    //What gets returned
    InvestmentResult {
        total_invested,
        final_value,
        pct_yield,
        days_invested,
        total_bct,
        profit,
        needed_investment_10k,
        avg_buyin,
        multi,
        needed_investment_5x,
    }
}

//Collects what comes from calc_type() and prints the data for each investment
fn print_result(type_name: &str, result: &InvestmentResult) {
    println!("\n{}", type_name);
    println!("Days Invested: {:.0}", result.days_invested);
    println!("Average Price Per BTC: ${:.2}", result.avg_buyin);
    println!("Total Invested: ${:.2}", result.total_invested);
    println!("Total Bitcoin: {:.5} BCT",result.total_bct);
    println!("Final Value: ${:.2}", result.final_value);
    println!("Percentage Yield: {:.2}%", result.pct_yield);
    println!("Total Profit: ${:.2}", result.profit);
    println!("To Profit 10k You Needed To Invest: ${:.2} Per Day", result.needed_investment_10k);
    println!("Multiplier: {:.2}x", result.multi);
    println!("To Multiply Your Earnings By 5x You Needed To Invest ${:.2} Per Day", result.needed_investment_5x);
}

//Runs the entire process
fn main() -> Result<(), Box<dyn Error>> {


    //Loads Database
    let mut db = Reader::from_path("bitcoin_fear_greed_2018_2024.csv")?;
    let mut records = Vec::new();


    for result in db.deserialize() {
        let record: BitcoinData = result?;
        records.push(record);
    }

    // Runs a few calculations to set up each printing investment
    let starting_price = records.first()
    .unwrap().Open;
    let final_price = records.last()
    .unwrap().Close;
    let price_dif = (final_price - starting_price);
    let daily_result = calc_type(&records, |_| true, final_price, 5.0, false);
    let extreme_fear_result = calc_type(&records, |r| r.Sentiment == "Extreme Fear", final_price, 5.0, false);
    let fear_result = calc_type(&records, |r| r.Sentiment == "Fear", final_price, 5.0, false);
    let neutral_result = calc_type(&records, |r| r.Sentiment == "Neutral", final_price, 5.0, false);
    let greed_result = calc_type(&records, |r| r.Sentiment == "Greed", final_price, 5.0, false);
    let extreme_greed_result = calc_type(&records, |r| r.Sentiment == "Extreme Greed", final_price, 5.0, false);
    let every_other_result = calc_type(&records, |_| true, final_price, 10.0, true);

    //Uses print_result() to print output for all investments
    println!("Based From Bitcoin Fear Greed Index (2018-2024)");
    println!("Starting Bitcoin Price: ${:.2}", starting_price);
    println!("Final Bitcoin Price: ${:.2}", final_price);
    println!("Total Profit Per Share: ${:.2}", price_dif);
    println!("What would happen if you were to invest $5 into bitcoin every day?");
    print_result("Investing Every Day", &daily_result);
    print_result("Investing Every Other Day", &every_other_result);
    print_result("Investing on Extreme Fear Days Only", &extreme_fear_result);
    print_result("Investing on Fear Days Only", &fear_result);
    print_result("Investing on Neutral Days Only", &neutral_result);
    print_result("Investing on Greed Days Only", &greed_result);
    print_result("Investing on Extreme Greed Days Only", &extreme_greed_result);


    //Stores each type of investment I am calculating
    let types = vec![
        ("$5 Every Day", daily_result),
        ("Extreme Fear", extreme_fear_result),
        ("Fear", fear_result),
        ("Neutral", neutral_result),
        ("Greed", greed_result),
        ("Extreme Greed", extreme_greed_result),
        ("$10 Every Other Day", every_other_result),
    ];


    //Lets you know best investment
    let best_idea = types.iter()
        .max_by(|a, b| a.1.pct_yield.partial_cmp(&b.1.pct_yield)
        .unwrap())
        .unwrap();


    //Lets you know worst investment
    let worst_idea = types.iter()
        .min_by(|a, b| a.1.pct_yield.partial_cmp(&b.1.pct_yield)
        .unwrap())
        .unwrap();


    println!("\nBest Idea: Investing only when there is {}", best_idea.0);
    println!("Percentage Yield: {:.2}%", best_idea.1.pct_yield);
    println!("\nWorst Idea: Investing only when there is {}", worst_idea.0);
    println!("Percentage Yield: {:.2}%", worst_idea.1.pct_yield);


    Ok(())
}


#[test]


//Sees if the code is actually filtering based on Sentiment
fn does_filter_work() {
    let records = vec![
        //Random days with easy data
        BitcoinData {
            Date: "2020-12-21".to_string(),
            Open: 100.0,
            High: 110.0,
            Low: 90.0,
            Close: 105.0,
            Volume: 1000.0,
            Fear_Greed_Index: 37,
            Sentiment: "Fear".to_string(),
        },
        BitcoinData {
            Date: "2021-12-21".to_string(),
            Open: 105.0,
            High: 115.0,
            Low: 95.0,
            Close: 110.0,
            Volume: 1100.0,
            Fear_Greed_Index: 71,
            Sentiment: "Greed".to_string(),
        },
        BitcoinData {
            Date: "2023-12-21".to_string(),
            Open: 110.0,
            High: 120.0,
            Low: 100.0,
            Close: 115.0,
            Volume: 1200.0,
            Fear_Greed_Index: 41,
            Sentiment: "Fear".to_string(),
        },
    ];
   
    let fear_result = calc_type(&records, |r| r.Sentiment == "Fear", 200.0, 5.0, false);


    assert_eq!(fear_result.days_invested, 2.0);
    assert_eq!(fear_result.total_invested, 10.0);
    assert_eq!(fear_result.total_bct, 5.0/100.0 + 5.0/110.0);
   
    let greed_result = calc_type(&records, |r| r.Sentiment == "Greed", 200.0, 5.0, false);
    assert_eq!(greed_result.days_invested, 1.0);
    assert_eq!(greed_result.total_invested, 5.0);
    assert_eq!(greed_result.total_bct, 5.0/105.0);
}


#[test]


//Sees if the filter of investing $10 every other day works
fn does_every_other_work() {
    let records = vec![
        //Random concurrent days with easy data
        BitcoinData {
            Date: "2018-02-01".to_string(),
            Open: 100.0,
            High: 110.0,
            Low: 90.0,
            Close: 105.0,
            Volume: 1000.0,
            Fear_Greed_Index: 30,
            Sentiment: "Fear".to_string(),
        },
        BitcoinData {
            Date: "2018-02-02".to_string(),
            Open: 105.0,
            High: 115.0,
            Low: 95.0,
            Close: 110.0,
            Volume: 1100.0,
            Fear_Greed_Index: 15,
            Sentiment: "Extreme Fear".to_string(),
        },
        BitcoinData {
            Date: "2018-02-03".to_string(),
            Open: 110.0,
            High: 120.0,
            Low: 100.0,
            Close: 115.0,
            Volume: 1200.0,
            Fear_Greed_Index: 40,
            Sentiment: "Fear".to_string(),
        },
        BitcoinData {
            Date: "2018-02-04".to_string(),
            Open: 115.0,
            High: 125.0,
            Low: 105.0,
            Close: 120.0,
            Volume: 1300.0,
            Fear_Greed_Index: 24,
            Sentiment: "Extreme Fear".to_string(),
        },
    ];


    let result = calc_type(&records, |_| true, 200.0, 10.0, true);
   
    assert_eq!(result.days_invested, 2.0);
    assert_eq!(result.total_invested, 20.0);
    assert_eq!(result.total_bct, 10.0/100.0 + 10.0/110.0);
    }

