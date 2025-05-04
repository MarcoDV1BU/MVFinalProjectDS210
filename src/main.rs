use std::error::Error;
use csv::Reader;
use serde::Deserialize;
use std::collections::HashMap;
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
#[derive(Debug)]
struct InvestmentResult {
    total_invested: f64,
    final_value: f64,
    pct_yield: f64,
    days_invested: usize,
}
fn calc_type<F>(records: &[BitcoinData], filter: F, final_price: f64) -> InvestmentResult 
where
    F: Fn(&BitcoinData) -> bool,
{
    let mut total_bct = 0.0;
    let mut total_invested = 0.0;
    let mut days_invested = 0;
    for record in records {
        if filter(record) {
            let bought = 5.0 / record.Open;
            total_bct += bought;
            total_invested += 5.0;
            days_invested += 1;
        }
    }
    let final_value = total_bct * final_price;
    let pct_yield = if total_invested > 0.0 {
        ((final_value - total_invested) / total_invested) * 100.0
    } else {
        0.0
    };
    InvestmentResult {
        total_invested,
        final_value,
        pct_yield,
        days_invested,
    }
}
fn print_result(type_name: &str, result: &InvestmentResult) {
    println!("\n{}", type_name);
    println!("Days Invested: {}", result.days_invested);
    println!("Total Invested: ${:.2}", result.total_invested);
    println!("Final Value: ${:.2}", result.final_value);
    println!("Percentage Yield: {:.2}%", result.pct_yield);
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut db = Reader::from_path("bitcoin_fear_greed_2018_2024.csv")?;
    let mut records = Vec::new();
    for result in db.deserialize() {
        let record: BitcoinData = result?;
        records.push(record);
    }
    let final_price = records.last().unwrap().Close;
    let daily_result = calc_type(&records, |_| true, final_price);
    let extreme_fear_result = calc_type(&records, |r| r.Sentiment == "Extreme Fear", final_price);
    let fear_result = calc_type(&records, |r| r.Sentiment == "Fear", final_price);
    let neutral_result = calc_type(&records, |r| r.Sentiment == "Neutral", final_price);
    let greed_result = calc_type(&records, |r| r.Sentiment == "Greed", final_price);
    let extreme_greed_result = calc_type(&records, |r| r.Sentiment == "Extreme Greed", final_price);
    println!("Based From Bitcoin Fear Greed Index (2018-2024)");
    println!("Final Bitcoin Price: ${:.2}", final_price);
    println!("What would happen if you were to invest $5 per day into bitcoin?");
    print_result("Investing Every Day", &daily_result);
    print_result("Investing on Extreme Fear Days Only", &extreme_fear_result);
    print_result("Investing on Fear Days Only", &fear_result);
    print_result("Investing on Neutral Days Only", &neutral_result);
    print_result("Investing on Greed Days Only", &greed_result);
    print_result("Investing on Extreme Greed Days Only", &extreme_greed_result);
    let types = vec![
        ("Every Day", daily_result),
        ("Extreme Fear", extreme_fear_result),
        ("Fear", fear_result),
        ("Neutral", neutral_result),
        ("Greed", greed_result),
        ("Extreme Greed", extreme_greed_result),
    ];
    let best_idea = types.iter()
        .max_by(|a, b| a.1.pct_yield.partial_cmp(&b.1.pct_yield).unwrap())
        .unwrap();
    let worst_idea = types.iter()
        .min_by(|a, b| a.1.pct_yield.partial_cmp(&b.1.pct_yield).unwrap())
        .unwrap();
    println!("\nBest Idea: Investing only when there is {}", best_idea.0);
    println!("Percentage Yield: {:.2}%", best_idea.1.pct_yield);
    println!("\nWorst Idea: Investing only when there is {}", worst_idea.0);
    println!("Percentage Yield: {:.2}%", worst_idea.1.pct_yield);
    Ok(())
}

