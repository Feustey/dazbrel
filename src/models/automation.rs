use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationSettings {
    pub enabled: bool,
    pub auto_execution_enabled: bool,
    pub risk_tolerance: RiskTolerance,
    pub max_daily_actions: u32,
    pub max_amount_per_action: u64,
    pub blacklisted_peers: Vec<String>,
    pub whitelisted_peers: Vec<String>,
    pub notification_preferences: NotificationPreferences,
    pub advanced_settings: AdvancedAutomationSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskTolerance {
    Conservative,
    Moderate,
    Aggressive,
    Custom(CustomRiskSettings),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRiskSettings {
    pub max_channel_size_percentage: f64, // % of total node capacity
    pub min_peer_reliability_score: f64,  // 0-100
    pub max_force_close_probability: f64, // 0-1
    pub min_expected_roi: f64,            // % minimum ROI requirement
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub email_enabled: bool,
    pub email_address: Option<String>,
    pub webhook_enabled: bool,
    pub webhook_url: Option<String>,
    pub slack_enabled: bool,
    pub slack_webhook: Option<String>,
    pub notification_triggers: Vec<NotificationTrigger>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTrigger {
    pub trigger_type: TriggerType,
    pub threshold: f64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerType {
    HighImpactAction,     // ROI impact > threshold
    LargeAmountAction,    // Amount > threshold
    FailedExecution,      // Action failed
    UnusualActivity,      // Unusual pattern detected
    PerformanceThreshold, // Performance metric threshold
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedAutomationSettings {
    pub ml_confidence_threshold: f64, // Minimum ML confidence for auto-execution
    pub market_condition_checks: bool, // Check market conditions before executing
    pub peer_reputation_checks: bool, // Verify peer reputation
    pub liquidity_impact_analysis: bool, // Analyze liquidity impact
    pub rollback_on_failure: bool,    // Auto-rollback failed actions
    pub learning_mode: bool,          // Learn from user decisions
    pub strategy_optimization: bool,  // Optimize strategies over time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub conditions: Vec<AutomationCondition>,
    pub actions: Vec<AutomationAction>,
    pub enabled: bool,
    pub priority: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_triggered: Option<chrono::DateTime<chrono::Utc>>,
    pub trigger_count: u32,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationCondition {
    pub condition_type: ConditionType,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
    pub weight: f64, // Weight in decision making (0-1)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    ROIImpact,         // Expected ROI impact
    ChannelBalance,    // Channel balance threshold
    FeeRate,           // Current fee rate
    PeerReliability,   // Peer reliability score
    MarketCondition,   // Market condition indicator
    TimeOfDay,         // Time-based condition
    NetworkCapacity,   // Network capacity threshold
    RecentPerformance, // Recent performance metric
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Between,
    In,
    NotIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationAction {
    pub action_type: AutomationActionType,
    pub parameters: serde_json::Value,
    pub delay_seconds: Option<u32>,
    pub retry_count: u32,
    pub rollback_action: Option<Box<AutomationAction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationActionType {
    ExecuteRecommendation,
    AdjustFees,
    OpenChannel,
    CloseChannel,
    Rebalance,
    SendNotification,
    UpdateStrategy,
    PauseAutomation,
    LogEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationExecution {
    pub id: String,
    pub rule_id: String,
    pub recommendation_id: Option<String>,
    pub execution_type: AutomationExecutionType,
    pub status: ExecutionStatus,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub results: ExecutionResults,
    pub error_message: Option<String>,
    pub rollback_executed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationExecutionType {
    Immediate,   // Execute immediately
    Scheduled,   // Execute at scheduled time
    Conditional, // Execute when conditions are met
    Manual,      // Manual trigger with automation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResults {
    pub success: bool,
    pub actions_taken: Vec<ActionResult>,
    pub performance_impact: Option<PerformanceImpact>,
    pub cost: u64,
    pub time_taken_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action: String,
    pub success: bool,
    pub details: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImpact {
    pub roi_change: f64,
    pub fee_income_change: f64,
    pub channel_balance_change: i64,
    pub liquidity_score_change: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationMetrics {
    pub total_executions: u32,
    pub successful_executions: u32,
    pub failed_executions: u32,
    pub average_execution_time: f64,
    pub total_roi_improvement: f64,
    pub total_fees_saved: u64,
    pub automation_uptime: f64,
    pub user_override_rate: f64,
    pub last_30_days: AutomationPeriodMetrics,
    pub last_7_days: AutomationPeriodMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationPeriodMetrics {
    pub executions: u32,
    pub success_rate: f64,
    pub roi_improvement: f64,
    pub average_response_time: f64,
    pub top_performing_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartScheduling {
    pub optimal_execution_times: Vec<OptimalExecutionTime>,
    pub market_condition_analysis: MarketConditionAnalysis,
    pub peer_activity_patterns: Vec<PeerActivityPattern>,
    pub network_congestion_forecast: NetworkCongestionForecast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimalExecutionTime {
    pub hour: u32,
    pub day_of_week: u32,
    pub success_probability: f64,
    pub average_cost: u64,
    pub expected_roi: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketConditionAnalysis {
    pub current_conditions: MarketConditions,
    pub favorable_conditions_eta: Option<chrono::DateTime<chrono::Utc>>,
    pub risk_level: RiskLevel,
    pub recommendation: MarketRecommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketConditions {
    Bullish,
    Bearish,
    Volatile,
    Stable,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketRecommendation {
    ExecuteNow,
    WaitForBetterConditions,
    ExecuteWithCaution,
    PostponeExecution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerActivityPattern {
    pub peer_pubkey: String,
    pub peak_activity_hours: Vec<u32>,
    pub success_rate_by_hour: Vec<f64>,
    pub average_response_time: f64,
    pub reliability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCongestionForecast {
    pub current_congestion_level: f64,
    pub predicted_congestion_24h: Vec<CongestionPrediction>,
    pub optimal_execution_windows: Vec<ExecutionWindow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongestionPrediction {
    pub hour: u32,
    pub congestion_level: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionWindow {
    pub start_hour: u32,
    pub end_hour: u32,
    pub expected_success_rate: f64,
    pub expected_cost_savings: f64,
}

impl Default for AutomationSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            auto_execution_enabled: false,
            risk_tolerance: RiskTolerance::Conservative,
            max_daily_actions: 10,
            max_amount_per_action: 1000000, // 0.01 BTC
            blacklisted_peers: vec![],
            whitelisted_peers: vec![],
            notification_preferences: NotificationPreferences::default(),
            advanced_settings: AdvancedAutomationSettings::default(),
        }
    }
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            email_enabled: false,
            email_address: None,
            webhook_enabled: false,
            webhook_url: None,
            slack_enabled: false,
            slack_webhook: None,
            notification_triggers: vec![
                NotificationTrigger {
                    trigger_type: TriggerType::HighImpactAction,
                    threshold: 5.0, // 5% ROI impact
                    enabled: true,
                },
                NotificationTrigger {
                    trigger_type: TriggerType::FailedExecution,
                    threshold: 0.0,
                    enabled: true,
                },
            ],
        }
    }
}

impl Default for AdvancedAutomationSettings {
    fn default() -> Self {
        Self {
            ml_confidence_threshold: 0.8,
            market_condition_checks: true,
            peer_reputation_checks: true,
            liquidity_impact_analysis: true,
            rollback_on_failure: true,
            learning_mode: true,
            strategy_optimization: true,
        }
    }
}
