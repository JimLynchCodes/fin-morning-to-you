fn main() {
    let tickers = vec![
        "AAPL", "AMD", "AMZN", "ANET", "ARKF", "ARKK", "AVGO", "BABA", "BIDU", "C", "CBP",
        "CHK", "CHTR", "COF", "CRSP", "DAL", "DIS", "DJT", "EBAY", "FANG", "FDX", "GDDY", "GM",
        "GOOG", "GOOGL", "GPS", "HMC", "IBKR", "INTC", "IWM", "JPM", "KHC", "LLY", "LMT", "LYFT",
        "MARA", "META", "MKL", "MSFT", "MU", "NFLX", "NKE", "NVDA", "ON", "ONON", "OSIS",
        "PD", "PLTR", "PNC", "PVH", "PYPL", "RDDT", "QCOM", "QQQ", "SMH", "SONY", "SOXL", "T",
        "TGT", "TM", "TSLA", "TSM", "U", "UAL", "UBS", "ULTA", "V", "VOD", "WBA", "WWW",
        "XLE",
    ];

    for ticker in tickers.iter() {
        open::that(format!("http://www.maximum-pain.com/options/{ticker}")).unwrap();
        open::that(format!(
            "https://swaggystocks.com/dashboard/options-max-pain/{ticker}"
        ))
        .unwrap();
    }
}
