use crate::api::local_lightning_client::LocalChannelInfo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnalytics {
    pub performance_score: f64, // 0-100 composite score
    pub roi_current: f64,       // Current ROI percentage
    pub roi_predicted_30d: f64, // Predicted 30-day ROI
    pub efficiency_score: f64,  // Capital efficiency score
    pub risk_score: f64,        // Risk assessment score
    pub centrality_score: f64,  // Network position score
    pub liquidity_score: f64,   // Liquidity management score
    pub reliability_score: f64, // Node reliability score
    pub growth_potential: f64,  // Growth opportunity score
    pub last_calculated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelAnalytics {
    pub channel_id: String,
    pub performance_metrics: ChannelPerformanceMetrics,
    pub profitability_metrics: ChannelProfitabilityMetrics,
    pub risk_metrics: ChannelRiskMetrics,
    pub optimization_suggestions: Vec<ChannelOptimization>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelPerformanceMetrics {
    pub forwarding_success_rate: f64, // % of successful forwards
    pub average_htlc_size: u64,       // Average HTLC amount
    pub forwards_per_day: f64,        // Daily forwarding volume
    pub total_fees_earned: u64,       // Total fees in satoshis
    pub fees_per_day_avg: f64,        // Average daily fees
    pub uptime_percentage: f64,       // Channel uptime %
    pub rebalancing_frequency: f64,   // Rebalances per week
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelProfitabilityMetrics {
    pub roi_annualized: f64,         // Annualized ROI %
    pub profit_per_sat: f64,         // Profit per satoshi locked
    pub fee_optimization_score: f64, // 0-100 fee optimization
    pub capital_efficiency: f64,     // Capital utilization score
    pub opportunity_cost: f64,       // Cost of locked capital
    pub break_even_days: f64,        // Days to break even
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRiskMetrics {
    pub peer_reliability_score: f64,  // Peer reliability 0-100
    pub liquidity_risk: f64,          // Risk of liquidity depletion
    pub force_close_probability: f64, // Probability of force close
    pub routing_competition: f64,     // Competition level for routes
    pub concentration_risk: f64,      // Risk from capital concentration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelOptimization {
    pub optimization_type: OptimizationType,
    pub description: String,
    pub expected_improvement: f64, // Expected improvement %
    pub implementation_cost: u64,  // Cost in satoshis
    pub confidence_level: f64,     // Confidence in prediction
    pub priority: OptimizationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationType {
    FeeAdjustment,
    Rebalancing,
    ChannelResize,
    PeerDiversification,
    LiquidityManagement,
    TimingOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveAnalytics {
    pub market_predictions: MarketPredictions,
    pub node_performance_forecast: NodePerformanceForecast,
    pub optimal_strategies: Vec<OptimalStrategy>,
    pub risk_scenarios: Vec<RiskScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPredictions {
    pub btc_price_trend: PriceTrend,
    pub network_growth_rate: f64,
    pub routing_demand_forecast: f64,
    pub fee_market_outlook: FeeMarketOutlook,
    pub liquidity_trends: LiquidityTrends,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTrend {
    pub direction: TrendDirection,
    pub magnitude: f64,
    pub confidence: f64,
    pub timeframe_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Bullish,
    Bearish,
    Sideways,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeMarketOutlook {
    pub average_fee_trend: TrendDirection,
    pub fee_volatility_forecast: f64,
    pub competitive_pressure: f64,
    pub optimal_fee_range: (u32, u32), // (min, max) in ppm
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityTrends {
    pub inbound_demand: f64,
    pub outbound_demand: f64,
    pub seasonal_patterns: Vec<SeasonalPattern>,
    pub peer_behavior_predictions: Vec<PeerBehaviorPrediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPattern {
    pub pattern_type: String,
    pub strength: f64,
    pub next_occurrence: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerBehaviorPrediction {
    pub peer_pubkey: String,
    pub predicted_activity: f64,
    pub reliability_forecast: f64,
    pub growth_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceForecast {
    pub roi_forecast_30d: f64,
    pub roi_forecast_90d: f64,
    pub roi_forecast_365d: f64,
    pub channel_count_forecast: u32,
    pub capacity_forecast: u64,
    pub performance_trajectory: PerformanceTrajectory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrajectory {
    pub trajectory_type: TrajectoryType,
    pub growth_rate: f64,
    pub peak_performance_eta: Option<chrono::DateTime<chrono::Utc>>,
    pub key_milestones: Vec<PerformanceMilestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrajectoryType {
    Exponential,
    Linear,
    Logarithmic,
    Plateau,
    Declining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMilestone {
    pub milestone_type: String,
    pub target_value: f64,
    pub estimated_date: chrono::DateTime<chrono::Utc>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalStrategy {
    pub strategy_name: String,
    pub description: String,
    pub expected_roi_improvement: f64,
    pub implementation_steps: Vec<StrategyStep>,
    pub risk_level: RiskLevel,
    pub time_horizon: TimeHorizon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyStep {
    pub step_number: u32,
    pub description: String,
    pub estimated_cost: u64,
    pub expected_benefit: f64,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeHorizon {
    Short,  // 1-30 days
    Medium, // 30-90 days
    Long,   // 90+ days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScenario {
    pub scenario_name: String,
    pub probability: f64,
    pub potential_impact: f64,
    pub mitigation_strategies: Vec<String>,
    pub early_warning_indicators: Vec<String>,
}

impl NodeAnalytics {
    pub fn calculate_from_channels(channels: &[LocalChannelInfo]) -> Self {
        // Simplified calculation - in real implementation this would use ML models
        let _total_capacity: u64 = channels.iter().map(|c| c.capacity).sum();
        let active_channels = channels.iter().filter(|c| c.active).count();

        let performance_score = if active_channels > 0 {
            (active_channels as f64 / channels.len() as f64) * 100.0
        } else {
            0.0
        };

        Self {
            performance_score,
            roi_current: 15.5, // Mock value
            roi_predicted_30d: 18.2,
            efficiency_score: 78.3,
            risk_score: 25.1,
            centrality_score: 82.7,
            liquidity_score: 91.4,
            reliability_score: 88.9,
            growth_potential: 73.6,
            last_calculated: chrono::Utc::now(),
        }
    }
}
