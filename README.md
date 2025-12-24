# Stock Market Analyst

A comprehensive Rust-based stock market analysis application that provides real-time stock data analysis, machine learning-powered predictions, technical indicators, and investor education.

## Features

### 🚀 Real-Time Data Analysis
- Live stock price updates from multiple data providers
- Historical data analysis with configurable time ranges
- Economic indicators and news sentiment analysis
- WebSocket connections for real-time updates

### 📊 Technical Analysis
- Moving averages (SMA, EMA, WMA)
- Momentum indicators (RSI, MACD, Stochastic)
- Volatility indicators (Bollinger Bands, ATR)
- Volume indicators (OBV, VWAP)
- Chart pattern recognition

### 🤖 Machine Learning Models
- LSTM networks for price forecasting
- Random Forest for volatility prediction
- Pattern recognition with CNN
- Sentiment analysis for news
- Portfolio optimization with reinforcement learning

### 💼 Portfolio Management
- Multi-asset portfolio tracking
- Risk metrics calculation (VaR, Sharpe ratio, beta)
- Performance attribution analysis
- Rebalancing recommendations
- Tax-loss harvesting suggestions

### 🎓 Investor Education
- Interactive tutorials on technical analysis
- Risk management simulations
- Virtual trading environment
- Educational content library
- Progress tracking and gamification

## Technology Stack

### Backend (Rust)
- **Web Framework**: Actix-web
- **Async Runtime**: Tokio
- **Machine Learning**: SmartCore, Linfa
- **Data Processing**: Polars (DataFrame library)
- **Database**: PostgreSQL with Diesel ORM
- **Caching**: Redis
- **HTTP Client**: Reqwest
- **WebSocket**: Tokio-tungstenite

### Frontend
- **Charting**: Chart.js
- **UI Framework**: Bootstrap 5
- **Real-time Updates**: WebSocket/SSE

## Quick Start

### Prerequisites
- Rust 1.70+ 
- PostgreSQL 12+
- Redis 6+
- Node.js 16+ (for frontend development)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/stock-analyst.git
cd stock-analyst
```

2. Set up the database:
```bash
# Create PostgreSQL database
createdb stock_analyst

# Run migrations
diesel migration run
```

3. Configure environment variables:
```bash
cp .env.example .env
# Edit .env with your API keys and database credentials
```

4. Install dependencies and build:
```bash
cargo build --release
```

5. Run the application:
```bash
cargo run --release
```

6. Open the web interface:
```
http://localhost:8080
```

## API Endpoints

### Stock Data
- `GET /stock/{symbol}` - Get current stock data
- `GET /analysis/{symbol}` - Get technical analysis
- `GET /predictions/{symbol}` - Get ML predictions

### Portfolio
- `GET /portfolio` - Get portfolio overview
- `POST /portfolio/holdings` - Add holding
- `PUT /portfolio/holdings/{id}` - Update holding
- `DELETE /portfolio/holdings/{id}` - Remove holding

### Education
- `GET /education/{topic}` - Get educational content
- `GET /education/simulations` - Get available simulations
- `POST /education/progress` - Update learning progress

## Configuration

The application can be configured through environment variables or configuration files:

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "postgres://user:password@localhost/stock_analyst"
max_connections = 10

[api_keys]
alpha_vantage = "your_alpha_vantage_key"
yahoo_finance = "your_yahoo_finance_key"
finnhub = "your_finnhub_key"
news_api = "your_news_api_key"
```

## Machine Learning Models

### Training
Models can be trained using historical data:

```bash
cargo run --bin train_models -- --symbol AAPL --period 1y
```

### Evaluation
Model performance can be evaluated:

```bash
cargo run --bin evaluate_models -- --symbol AAPL --test-period 3m
```

## Testing

Run the test suite:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test integration_tests
```

## Deployment

### Docker
```bash
docker build -t stock-analyst .
docker run -p 8080:8080 stock-analyst
```

### Docker Compose
```bash
docker-compose up -d
```

### Kubernetes
```bash
kubectl apply -f k8s/
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## API Keys

You'll need to obtain API keys from the following services:

- [Alpha Vantage](https://www.alphavantage.co/support/#api-key) - Free tier available
- [Yahoo Finance](https://www.yahoofinanceapi.com/) - Free tier available  
- [Finnhub](https://finnhub.io/) - Free tier available
- [News API](https://newsapi.org/) - Free tier available

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This application is for educational and informational purposes only. It should not be used as the sole basis for making investment decisions. Always conduct your own research and consult with qualified financial advisors before making investment choices.

## Support

For support, please open an issue on GitHub or contact support@stockanalyst.com