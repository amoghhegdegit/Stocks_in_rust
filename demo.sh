#!/bin/bash

# Stock Market Analyst - Demo Script
# This script demonstrates the key features of the application

echo "🚀 Stock Market Analyst Demo"
echo "================================"

# Check if the application is running
if ! curl -s http://localhost:8080/health > /dev/null; then
    echo "❌ Application is not running. Please start it first:"
    echo "   cargo run --release"
    exit 1
fi

echo "✅ Application is running"

# Demo 1: Health Check
echo ""
echo "1. Health Check"
echo "---------------"
curl -s http://localhost:8080/health | jq '.'

# Demo 2: Stock Data
echo ""
echo "2. Stock Data (AAPL)"
echo "-------------------"
curl -s http://localhost:8080/stock/AAPL | jq '.'

# Demo 3: Technical Analysis
echo ""
echo "3. Technical Analysis (AAPL)"
echo "----------------------------"
curl -s http://localhost:8080/analysis/AAPL | jq '.'

# Demo 4: ML Predictions
echo ""
echo "4. ML Predictions (AAPL)"
echo "------------------------"
curl -s http://localhost:8080/predictions/AAPL | jq '.'

# Demo 5: Portfolio Overview
echo ""
echo "5. Portfolio Overview"
echo "--------------------"
curl -s http://localhost:8080/portfolio | jq '.'

# Demo 6: Education Content
echo ""
echo "6. Education Content"
echo "-------------------"
curl -s http://localhost:8080/education/technical-analysis | jq '.'

echo ""
echo "🎉 Demo completed!"
echo ""
echo "To view the web interface, open: http://localhost:8080"
echo ""
echo "To train ML models:"
echo "   cargo run --bin train_models -- --symbol AAPL --period 1y"
echo ""
echo "To evaluate models:"
echo "   cargo run --bin evaluate_models -- --symbol AAPL --test-period 3m"