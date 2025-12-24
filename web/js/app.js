// Global variables
let priceChart = null;
let currentSymbol = 'AAPL';

// API base URL
const API_BASE_URL = 'http://localhost:8080';

// Initialize the application
document.addEventListener('DOMContentLoaded', function() {
    console.log('Stock Market Analyst loaded');
    fetchStockData();
    fetchPortfolioData();
    
    // Add enter key support for stock symbol input
    document.getElementById('stockSymbol').addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            fetchStockData();
        }
    });
});

// Fetch stock data
async function fetchStockData() {
    const symbol = document.getElementById('stockSymbol').value.toUpperCase();
    if (!symbol) {
        alert('Please enter a stock symbol');
        return;
    }
    
    currentSymbol = symbol;
    
    try {
        // Show loading state
        showLoading('stockInfo');
        
        // Fetch basic stock data
        const stockResponse = await fetch(`${API_BASE_URL}/stock/${symbol}`);
        const stockData = await stockResponse.json();
        
        // Fetch technical analysis
        const analysisResponse = await fetch(`${API_BASE_URL}/analysis/${symbol}`);
        const analysisData = await analysisResponse.json();
        
        // Fetch predictions
        const predictionsResponse = await fetch(`${API_BASE_URL}/predictions/${symbol}`);
        const predictionsData = await predictionsResponse.json();
        
        // Update UI
        updateStockInfo(stockData);
        updateTechnicalAnalysis(analysisData);
        updatePredictions(predictionsData);
        
        // Create price chart
        createPriceChart();
        
        // Show results
        document.getElementById('stockInfo').style.display = 'block';
        
    } catch (error) {
        console.error('Error fetching stock data:', error);
        alert('Error fetching stock data. Please try again.');
    }
}

// Update stock information
function updateStockInfo(data) {
    document.getElementById('stockTitle').textContent = `${data.symbol} - Stock Information`;
    document.getElementById('currentPrice').textContent = `$${data.price.toFixed(2)}`;
    document.getElementById('priceChange').textContent = `$${data.change.toFixed(2)}`;
    document.getElementById('percentChange').textContent = `${data.change_percent.toFixed(2)}%`;
    document.getElementById('volume').textContent = formatNumber(data.volume);
    
    // Set color based on change
    const changeElement = document.getElementById('priceChange');
    const percentElement = document.getElementById('percentChange');
    
    if (data.change >= 0) {
        changeElement.className = 'text-success';
        percentElement.className = 'text-success';
    } else {
        changeElement.className = 'text-danger';
        percentElement.className = 'text-danger';
    }
}

// Update technical analysis
function updateTechnicalAnalysis(data) {
    document.getElementById('rsiValue').textContent = data.rsi ? data.rsi.toFixed(2) : 'N/A';
    document.getElementById('macdValue').textContent = data.macd ? data.macd.macd.toFixed(2) : 'N/A';
    document.getElementById('signalValue').textContent = data.macd ? data.macd.signal.toFixed(2) : 'N/A';
    
    const recommendation = data.recommendation || 'HOLD';
    const recommendationElement = document.getElementById('recommendation');
    recommendationElement.textContent = recommendation;
    
    // Set recommendation color
    switch (recommendation.toUpperCase()) {
        case 'BUY':
        case 'STRONGBUY':
            recommendationElement.className = 'badge bg-success';
            break;
        case 'SELL':
        case 'STRONGSELL':
            recommendationElement.className = 'badge bg-danger';
            break;
        default:
            recommendationElement.className = 'badge bg-warning';
    }
}

// Update predictions
function updatePredictions(data) {
    document.getElementById('prediction1d').textContent = `$${data.predictions['1_day'].toFixed(2)}`;
    document.getElementById('prediction1w').textContent = `$${data.predictions['1_week'].toFixed(2)}`;
    document.getElementById('prediction1m').textContent = `$${data.predictions['1_month'].toFixed(2)}`;
    
    document.getElementById('confidence1d').textContent = `${(data.confidence * 100).toFixed(0)}%`;
    document.getElementById('confidence1w').textContent = `${(data.confidence * 100).toFixed(0)}%`;
    document.getElementById('confidence1m').textContent = `${(data.confidence * 100).toFixed(0)}%`;
}

// Create price chart
function createPriceChart() {
    const ctx = document.getElementById('priceChart').getContext('2d');
    
    // Destroy existing chart if it exists
    if (priceChart) {
        priceChart.destroy();
    }
    
    // Generate sample data for demonstration
    const labels = [];
    const data = [];
    const basePrice = 150;
    
    for (let i = 0; i < 30; i++) {
        const date = new Date();
        date.setDate(date.getDate() - (29 - i));
        labels.push(date.toLocaleDateString());
        
        // Generate some realistic price movement
        const variation = (Math.random() - 0.5) * 10;
        const price = basePrice + variation + (i * 0.5);
        data.push(price);
    }
    
    priceChart = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [{
                label: `${currentSymbol} Price`,
                data: data,
                borderColor: 'rgb(75, 192, 192)',
                backgroundColor: 'rgba(75, 192, 192, 0.2)',
                tension: 0.1
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'top',
                },
                title: {
                    display: true,
                    text: `${currentSymbol} Price History (30 Days)`
                }
            },
            scales: {
                y: {
                    beginAtZero: false,
                    ticks: {
                        callback: function(value) {
                            return '$' + value.toFixed(2);
                        }
                    }
                }
            }
        }
    });
}

// Fetch portfolio data
async function fetchPortfolioData() {
    try {
        const response = await fetch(`${API_BASE_URL}/portfolio`);
        const data = await response.json();
        
        updatePortfolioInfo(data);
    } catch (error) {
        console.error('Error fetching portfolio data:', error);
    }
}

// Update portfolio information
function updatePortfolioInfo(data) {
    document.getElementById('portfolioValue').textContent = `$${formatNumber(data.total_value)}`;
    document.getElementById('portfolioGain').textContent = `$${formatNumber(data.total_gain_loss)}`;
    document.getElementById('portfolioPercent').textContent = `${data.total_gain_loss_percent >= 0 ? '+' : ''}${data.total_gain_loss_percent.toFixed(2)}%`;
    
    // Set color based on performance
    const gainElement = document.getElementById('portfolioGain');
    const percentElement = document.getElementById('portfolioPercent');
    
    if (data.total_gain_loss >= 0) {
        gainElement.className = 'text-success';
        percentElement.className = 'text-success';
    } else {
        gainElement.className = 'text-danger';
        percentElement.className = 'text-danger';
    }
}

// Load education content
async function loadEducationContent(topic) {
    try {
        const response = await fetch(`${API_BASE_URL}/education/${topic}`);
        const data = await response.json();
        
        // Show educational content in a modal or new section
        alert(`Educational Content: ${data.content}\n\nEstimated time: ${data.estimated_time} minutes`);
    } catch (error) {
        console.error('Error fetching education content:', error);
        alert('Error loading educational content. Please try again.');
    }
}

// Utility functions
function formatNumber(num) {
    if (num >= 1e9) {
        return (num / 1e9).toFixed(2) + 'B';
    } else if (num >= 1e6) {
        return (num / 1e6).toFixed(2) + 'M';
    } else if (num >= 1e3) {
        return (num / 1e3).toFixed(2) + 'K';
    } else {
        return num.toFixed(2);
    }
}

function showLoading(elementId) {
    const element = document.getElementById(elementId);
    if (element) {
        element.classList.add('loading');
    }
}

function hideLoading(elementId) {
    const element = document.getElementById(elementId);
    if (element) {
        element.classList.remove('loading');
    }
}

// Auto-refresh functionality
setInterval(() => {
    if (currentSymbol) {
        fetchStockData();
    }
}, 60000); // Refresh every minute

// WebSocket connection for real-time updates (placeholder)
function connectWebSocket() {
    // TODO: Implement WebSocket connection for real-time updates
    console.log('WebSocket connection would be established here');
}

// Initialize WebSocket connection
connectWebSocket();