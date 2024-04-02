fn main() {
    // let sites = vec![
    //     "https://www.cnbc.com/the-morning-report/",
    //     "https://www.coindesk.com/price/ethereum/",
    //     "https://discord.com/channels/708365137660215327/765540788162920480",
    //     "https://swaggystocks.com/dashboard/options-max-pain/spy",
    //     "https://www.cnn.com/markets/fear-and-greed",
    //     "http://www.maximum-pain.com/options/nvda",
    //     "http://www.maximum-pain.com/options/SPX",
    //     "http://www.maximum-pain.com/options/MSFT",
    //     "http://www.maximum-pain.com/options/TSLA",
    //     "http://www.maximum-pain.com/options/QQQ",
    //     "https://www.coindesk.com/price/bitcoin/",
    //     "https://www.coindesk.com/price/ethereum/",
    //     "https://www.coindesk.com/price/solana/",
    // ];

    let tickers = vec![
        // "AAPL", "AMC", "AMD", "AMZN", "ANET", "ARKF", "ARKK", "AVGO", "BABA", "BIDU", "C", "CBP",
        "CHK", "CHTR", "COF", "CRSP", "DAL", "DIS", "DJT", "EBAY", "FANG", "FDX", "GDDY", "GM",
        "GOOG", "GOOGL", "GPS", "HMC", "IBKR", "INTC", "IWM", "JPM", "KHC", "LLY", "LMT", "LYFT",
        // "MARA", "META", "MKL", "MSFT", "MSFT", "MU", "NFLX", "NKE", "NVDA", "ON", "ONON", "OSIS",
        // "PD", "PLTR", "PNC", "PVH", "PYPL", "QCOM", "QQQ", "QQQ", "SMH", "SONY", "SOXL", "T",
        // "TGT", "TM", "TSLA", "TSLA", "TSM", "U", "UAL", "UBS", "ULTA", "V", "VOD", "WBA", "WWW",
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
