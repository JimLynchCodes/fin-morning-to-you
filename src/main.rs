fn main() {

    let sites = vec![
        "https://www.cnbc.com/the-morning-report/",
        "https://www.coindesk.com/price/ethereum/",
        "https://discord.com/channels/708365137660215327/765540788162920480",
        "https://swaggystocks.com/dashboard/options-max-pain/spy",
        "https://www.cnn.com/markets/fear-and-greed",
        "http://www.maximum-pain.com/options/nvda",
        "http://www.maximum-pain.com/options/SPX",
        "http://www.maximum-pain.com/options/MSFT",
        "http://www.maximum-pain.com/options/TSLA",
        "http://www.maximum-pain.com/options/QQQ",
        "https://www.coindesk.com/price/bitcoin/",
        "https://www.coindesk.com/price/ethereum/",
        "https://www.coindesk.com/price/solana/",
    ];

    for site in sites.iter() {
        open::that(site).unwrap();
    }

}
