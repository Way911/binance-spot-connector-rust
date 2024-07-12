use crate::market::klines::KlineInterval;
use crate::websocket::Stream;

/// Kline/Candlestick Stream
///
/// The Kline/Candlestick Stream push updates to the current klines/candlestick every second.
///
/// Update Speed: 2000ms
///
/// [API Documentation](https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-streams)
///
/// # Example
///
/// ```
/// use binance_spot_connector_rust::{ market::klines::KlineInterval, market_stream::kline::KlineStream };
///
/// let stream = KlineStream::new("BTCUSDT", KlineInterval::Minutes1);
/// ```
pub struct KlineStream {
    symbol: String,
    interval: KlineInterval,
    timezone: Option<String>,
}

impl KlineStream {
    pub fn new(symbol: &str, interval: KlineInterval, timezone: Option<String>) -> Self {
        Self {
            symbol: symbol.to_lowercase(),
            interval,
            timezone,
        }
    }
}

impl From<KlineStream> for Stream {
    /// Returns stream name as `<symbol>@kline_interval`
    fn from(stream: KlineStream) -> Stream {
        if let Some(timezone) = stream.timezone {
            Stream::new(&format!(
                "{}@kline_{}@{}",
                stream.symbol, stream.interval, timezone
            ))
        } else {
            Stream::new(&format!("{}@kline_{}", stream.symbol, stream.interval))
        }
    }
}
