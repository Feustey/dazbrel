#### 3.5.3 JavaScript avancé (surpasser Amboss en interactivité)
```javascript
// static/js/superior-dashboard.js

class SuperiorDaznoClient {
    constructor() {
        this.wsConnection = null;
        this.charts = {};
        this.realTimeData = {};
        this.automationSettings = {};
        this.competitorData = [];
        this.isSimulating = false;
        
        this.init();
    }

    async init() {
        await this.setupWebSocket();
        this.initializeCharts();
        this.setupEventListeners();
        this.startRealTimeUpdates();
        this.loadAutomationSettings();
        
        console.log('🚀 Superior Dazno Dashboard initialized');
    }

    // WebSocket pour données temps réel
    async setupWebSocket() {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws/realtime`;
        
        this.wsConnection = new WebSocket(wsUrl);
        
        this.wsConnection.onopen = () => {
            console.log('🔗 Real-time connection established');
            this.showNotification('Connected to real-time updates', 'success');
        };
        
        this.wsConnection.onmessage = (event) => {
            const data = JSON.parse(event.data);
            this.handleRealTimeUpdate(data);
        };
        
        this.wsConnection.onclose = () => {
            console.log('📡 Connection lost, attempting reconnect...');
            setTimeout(() => this.setupWebSocket(), 5000);
        };
    }

    handleRealTimeUpdate(data) {
        switch (data.type) {
            case 'roi_update':
                this.updateROIDisplay(data.payload);
                break;
            case 'new_recommendation':
                this.addRecommendation(data.payload);
                break;
            case 'automation_result':
                this.handleAutomationResult(data.payload);
                break;
            case 'competitive_update':
                this.updateCompetitiveAnalysis(data.payload);
                break;
            case 'prediction_update':
                this.updatePredictions(data.payload);
                break;
        }
    }

    // Graphiques avancés avec Chart.js
    initializeCharts() {
        // Graphique de performance en temps réel
        const performanceCtx = document.getElementById('realtimePerformanceChart');
        if (performanceCtx) {
            this.charts.performance = new Chart(performanceCtx, {
                type: 'line',
                data: {
                    labels: [],
                    datasets: [{
                        label: 'ROI %',
                        data: [],
                        borderColor: '#06D6A0',
                        backgroundColor: 'rgba(6, 214, 160, 0.1)',
                        tension: 0.4,
                        fill: true
                    }, {
                        label: 'Network Average',
                        data: [],
                        borderColor: '#64748B',
                        backgroundColor: 'transparent',
                        borderDash: [5, 5],
                        tension: 0.4
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: {
                        legend: {
                            labels: { color: '#F8FAFC' }
                        }
                    },
                    scales: {
                        x: {
                            ticks: { color: '#94A3B8' },
                            grid: { color: '#334155' }
                        },
                        y: {
                            ticks: { color: '#94A3B8' },
                            grid: { color: '#334155' }
                        }
                    },
                    animation: {
                        duration: 750,
                        easing: 'easeInOutQuart'
                    }
                }
            });
        }

        // Graphique de comparaison concurrentielle
        const competitorCtx = document.getElementById('competitorChart');#### 3.5.1 Templates Handlebars avancés (surpasser Amboss UX)
```html
<!-- templates/superior_dashboard.hbs -->
<!DOCTYPE html>
<html>
<head>
    <title>Dazno Pro - Superior Lightning ROI Optimizer</title>
    <link rel="stylesheet" href="/static/css/dazno-superior-theme.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/Chart.js/4.4.0/chart.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/moment.js/2.29.4/moment.min.js"></script>
    <script src="/static/js/superior-dashboard.js"></script>
</head>
<body>
    <div class="dazno-superior-container">
        <header class="dazno-header-advanced">
            <div class="header-left">
                <h1>⚡ Dazno Pro</h1>
                <div class="competitive-badge">
                    <span class="vs-amboss">vs Amboss: +{{performance_advantage}}%</span>
                </div>
            </div>
            <div class="header-right">
                <div class="roi-indicator real-time">
                    <span class="roi-label">Live ROI</span>
                    <span class="roi-value {{roi_trend}}">{{current_roi}}%</span>
                    <span class="roi-prediction">→ {{predicted_roi}}%</span>
                </div>
                <div class="status-cluster">
                    <div class="status-indicator {{connection_status}}"></div>
                    <div class="automation-status {{automation_status}}">
                        {{#if automation_enabled}}🤖 AUTO{{else}}⏸️ MANUAL{{/if}}
                    </div>
                </div>
            </div>
        </header>
        
        <main class="superior-dashboard-grid">
            <!-- Panneau de performance en temps réel -->
            <section class="performance-command-center">
                <h2>🎯 Performance Command Center</h2>
                <div class="metrics-grid-advanced">
                    <div class="metric-card-pro primary">
                        <div class="metric-header">
                            <span class="metric-icon">💰</span>
                            <span class="metric-label">Current ROI</span>
                            <span class="trend-indicator {{roi_trend}}">{{roi_change_24h}}%</span>
                        </div>
                        <div class="metric-value-large">{{current_roi}}%</div>
                        <div class="metric-prediction">
                            <span>Predicted 30d: <strong>{{predicted_roi_30d}}%</strong></span>
                        </div>
                    </div>
                    
                    <div class="metric-card-pro">
                        <div class="metric-header">
                            <span class="metric-icon">📊</span>
                            <span class="metric-label">vs Competition</span>
                            <span class="competitive-rank">#{{market_rank}}</span>
                        </div>
                        <div class="metric-value">{{competitive_advantage}}%</div>
                        <div class="metric-subtitle">above avg</div>
                    </div>
                    
                    <div class="metric-card-pro">
                        <div class="metric-header">
                            <span class="metric-icon">⚡</span>
                            <span class="metric-label">Routing Success</span>
                        </div>
                        <div class="metric-value">{{routing_success_rate}}%</div>
                        <div class="metric-subtitle">{{successful_routes}}/{{total_routes}} today</div>
                    </div>
                    
                    <div class="metric-card-pro">
                        <div class="metric-header">
                            <span class="metric-icon">🌊</span>
                            <span class="metric-label">Liquidity Efficiency</span>
                        </div>
                        <div class="metric-value">{{liquidity_efficiency}}%</div>
                        <div class="metric-subtitle">{{active_liquidity}} sats active</div>
                    </div>
                </div>
                
                <!-- Graphique de performance en temps réel -->
                <div class="real-time-chart-container">
                    <canvas id="realtimePerformanceChart"></canvas>
                </div>
            </section>
            
            <!-- Panneau de recommandations ML-powered -->
            <section class="ai-recommendations-panel">
                <div class="panel-header-advanced">
                    <h2>🧠 AI-Powered Recommendations</h2>
                    <div class="ai-status">
                        <span class="ai-indicator active">🤖 ML Active</span>
                        <span class="confidence-score">Confidence: {{avg_confidence}}%</span>
                    </div>
                </div>
                
                {{#each enhanced_recommendations}}
                <div class="recommendation-card-pro priority-{{priority}} confidence-{{confidence_level}}">
                    <div class="rec-header">
                        <div class="rec-title-group">
                            <h3>{{action_type_display}}</h3>
                            <div class="rec-badges">
                                <span class="priority-badge priority-{{priority}}">{{priority}}</span>
                                <span class="confidence-badge">{{confidence_score}}% sure</span>
                                {{#if competitive_advantage}}
                                <span class="advantage-badge">🎯 Edge</span>
                                {{/if}}
                            </div>
                        </div>
                        <div class="rec-impact-group">
                            <div class="impact-value">+{{expected_roi_impact}}%</div>
                            <div class="impact-label">ROI Impact</div>
                        </div>
                    </div>
                    
                    <div class="rec-details">
                        <p class="rec-description">{{description}}</p>
                        {{#if competitive_advantage}}
                        <div class="competitive-insight">
                            <span class="insight-icon">💡</span>
                            <span class="insight-text">{{competitive_advantage}}</span>
                        </div>
                        {{/if}}
                    </div>
                    
                    <div class="rec-metrics-row">
                        <div class="rec-metric">
                            <span class="metric-label">Capital Required</span>
                            <span class="metric-value">{{capital_requirement}} sats</span>
                        </div>
                        <div class="rec-metric">
                            <span class="metric-label">Execution Time</span>
                            <span class="metric-value">{{estimated_execution_time}}</span>
                        </div>
                        <div class="rec-metric">
                            <span class="metric-label">Risk Level</span>
                            <span class="metric-value risk-{{risk_level}}">{{risk_assessment}}</span>
                        </div>
                    </div>
                    
                    <div class="rec-timing-indicator">
                        <span class="timing-icon">⏰</span>
                        <span class="timing-text">Market Timing: {{market_timing_score}}/10</span>
                        <div class="timing-bar">
                            <div class="timing-fill" style="width: {{market_timing_percentage}}%"></div>
                        </div>
                    </div>
                    
                    <div class="action-buttons-advanced">
                        {{#if automation_enabled}}
                        <button class="btn-auto-execute" data-id="{{id}}" data-type="auto">
                            🤖 Auto Execute
                        </button>
                        {{/if}}
                        <button class="btn-approve-smart" data-id="{{id}}" data-type="manual">
                            ✅ Execute Now
                        </button>
                        <button class="btn-schedule" data-id="{{id}}" data-optimal-time="{{optimal_execution_time}}">
                            ⏱️ Schedule Optimal
                        </button>
                        <button class="btn-simulate" data-id="{{id}}">
                            🧪 Simulate
                        </button>
                        <button class="btn-reject-smart" data-id="{{id}}">
                            ❌ Reject
                        </button>
                    </div>
                </div>
                {{/each}}
                
                {{#unless enhanced_recommendations}}
                <div class="no-recommendations-state">
                    <div class="state-icon">🎯</div>
                    <h3>Your Node is Optimized!</h3>
                    <p>No high-impact opportunities detected. The AI will continue monitoring for new possibilities.</p>
                    <button class="btn-force-analysis">🔍 Force Deep Analysis</button>
                </div>
                {{/unless}}
            </section>
            
            <!-- Panneau d'automatisation avancée -->
            <section class="automation-control-panel">
                <h2>⚙️ Advanced Automation</h2>
                
                <div class="automation-stats">
                    <div class="auto-metric">
                        <span class="auto-label">Actions Today</span>
                        <span class="auto-value">{{automated_actions_today}}</span>
                    </div>
                    <div class="auto-metric">
                        <span class="auto-label">ROI Gained</span>
                        <span class="auto-value success">+{{automation_roi_gain}}%</span>
                    </div>
                    <div class="auto-metric">
                        <span class="auto-label">Success Rate</span>
                        <span class="auto-value">{{automation_success_rate}}%</span>
                    </div>
                </div>
                
                <div class="automation-controls">
                    <div class="control-group">
                        <h4>🤖 Smart Execution</h4>
                        <label class="smart-toggle">
                            <input type="checkbox" id="autoExecuteToggle" {{#if automation_enabled}}checked{{/if}}>
                            <span class="toggle-slider"></span>
                            <span class="toggle-label">Auto-execute high-confidence recommendations</span>
                        </label>
                        
                        <div class="automation-thresholds">
                            <div class="threshold-control">
                                <label>Minimum Confidence:</label>
                                <input type="range" id="confidenceThreshold" min="50" max="99" value="{{confidence_threshold}}">
                                <span class="threshold-value">{{confidence_threshold}}%</span>
                            </div>
                            <div class="threshold-control">
                                <label>Maximum Risk:</label>
                                <select id="riskThreshold">
                                    <option value="low" {{#eq risk_threshold "low"}}selected{{/eq}}>Low Risk Only</option>
                                    <option value="medium" {{#eq risk_threshold "medium"}}selected{{/eq}}>Low + Medium Risk</option>
                                    <option value="high" {{#eq risk_threshold "high"}}selected{{/eq}}>All Risk Levels</option>
                                </select>
                            </div>
                        </div>
                    </div>
                    
                    <div class="control-group">
                        <h4>⏰ Intelligent Scheduling</h4>
                        <label class="smart-toggle">
                            <input type="checkbox" id="smartSchedulingToggle" {{#if smart_scheduling_enabled}}checked{{/if}}>
                            <span class="toggle-slider"></span>
                            <span class="toggle-label">Auto-schedule for optimal market timing</span>
                        </label>
                    </div>
                </div>
                
                <!-- Journal d'automatisation en temps réel -->
                <div class="automation-log">
                    <h4>🔄 Recent Automation Activity</h4>
                    <div class="log-entries">
                        {{#each automation_log}}
                        <div class="log-entry {{status}}">
                            <div class="log-time">{{formatted_time}}</div>
                            <div class="log-action">{{action_description}}</div>
                            <div class="log-result {{result_type}}">{{result}}</div>
                        </div>
                        {{/each}}
                    </div>
                </div>
            </section>
            
            <!-- Panneau de monitoring avancé -->
            <section class="advanced-monitoring-panel">
                <h2>📊 Advanced Analytics</h2>
                
                <div class="monitoring-tabs">
                    <button class="tab-btn active" data-tab="performance">Performance</button>
                    <button class="tab-btn" data-tab="competition">vs Competition</button>
                    <button class="tab-btn" data-tab="predictions">Predictions</button>
                    <button class="tab-btn" data-tab="channels">Channel Analytics</button>
                </div>
                
                <div class="tab-content active" id="performance-tab">
                    <div class="chart-container">
                        <canvas id="performanceChart"></canvas>
                    </div>
                    <div class="performance-insights">
                        {{#each performance_insights}}
                        <div class="insight-card">
                            <span class="insight-icon">{{icon}}</span>
                            <div class="insight-content">
                                <h5>{{title}}</h5>
                                <p>{{description}}</p>
                            </div>
                            <div class="insight-impact">{{impact}}</div>
                        </div>
                        {{/each}}
                    </div>
                </div>
                
                <div class="tab-content" id="competition-tab">
                    <div class="competitive-analysis">
                        <div class="competitor-comparison">
                            <h4>🏆 Market Position</h4>
                            <div class="position-chart">
                                <canvas id="competitorChart"></canvas>
                            </div>
                        </div>
                        <div class="competitive-insights">
                            {{#each competitive_insights}}
                            <div class="competitive-insight-card">
                                <div class="competitor-name">{{competitor_name}}</div>
                                <div class="performance-diff {{trend}}">{{performance_difference}}</div>
                                <div class="opportunity">{{opportunity_description}}</div>
                            </div>
                            {{/each}}
                        </div>
                    </div>
                </div>
                
                <div class="tab-content" id="predictions-tab">
                    <div class="predictions-dashboard">
                        <div class="prediction-chart-container">
                            <canvas id="predictionsChart"></canvas>
                        </div>
                        <div class="prediction-cards">
                            {{#each ml_predictions}}
                            <div class="prediction-card confidence-{{confidence_level}}">
                                <h5>{{prediction_type}}</h5>
                                <div class="prediction-value">{{predicted_value}}</div>
                                <div class="prediction-timeframe">{{timeframe}}</div>
                                <div class="prediction-confidence">{{confidence}}% confidence</div>
                            </div>
                            {{/each}}
                        </div>
                    </div>
                </div>
                
                <div class="tab-content" id="channels-tab">
                    <div class="channels-analytics">
                        <div class="channel-performance-table">
                            <table class="advanced-table">
                                <thead>
                                    <tr>
                                        <th>Channel</th>
                                        <th>ROI</th>
                                        <th>Success Rate</th>
                                        <th>Liquidity Efficiency</th>
                                        <th>Recommendation</th>
                                        <th>Actions</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {{#each channel_analytics}}
                                    <tr class="channel-row performance-{{performance_tier}}">
                                        <td class="channel-info">
                                            <div class="channel-alias">{{alias}}</div>
                                            <div class="channel-capacity">{{capacity}} sats</div>
                                        </td>
                                        <td class="roi-cell {{roi_trend}}">{{roi}}%</td>
                                        <td class="success-rate-cell">
                                            <div class="progress-bar">
                                                <div class="progress-fill" style="width: {{success_rate}}%"></div>
                                            </div>
                                            <span>{{success_rate}}%</span>
                                        </td>
                                        <td class="efficiency-cell">{{liquidity_efficiency}}%</td>
                                        <td class="recommendation-cell">
                                            {{#if has_recommendation}}
                                            <span class="rec-badge {{recommendation_type}}">{{recommendation_text}}</span>
                                            {{else}}
                                            <span class="no-rec">Optimal</span>
                                            {{/if}}
                                        </td>
                                        <td class="actions-cell">
                                            <button class="btn-channel-action optimize" data-channel="{{channel_id}}">Optimize</button>
                                            <button class="btn-channel-action details" data-channel="{{channel_id}}">Details</button>
                                        </td>
                                    </tr>
                                    {{/each}}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </section>
        </main>
        
        <!-- Notification système avancé -->
        <div id="notification-system" class="notifications-container">
            <!-- Notifications dynamiques injectées via JavaScript -->
        </div>
        
        <!-- Modal de simulation -->
        <div id="simulation-modal" class="modal-overlay">
            <div class="modal-content simulation-modal">
                <div class="modal-header">
                    <h3>🧪 Recommendation Simulation</h3>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <div id="simulation-results">
                        <!-- Contenu de simulation injecté dynamiquement -->
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn-modal secondary">Cancel</button>
                    <button class="btn-modal primary">Execute After Simulation</button>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        // Initialisation du dashboard avancé
        document.addEventListener('DOMContentLoaded', function() {
            initializeSuperiorDashboard();
            setupRealtimeUpdates();
            initializeAutomationControls();
            setupAdvancedCharts();
        });
    </script>
</body>
</html>
```

#### 3.5.2 CSS Theme avancé (surpasser Amboss design)
```css
/* static/css/dazno-superior-theme.css */
:root {
  /* Palette étendue pour surpasser Amboss */
  --dazno-primary: #2D5BFF;
  --dazno-primary-dark: #1E40AF;
  --dazno-secondary: #8B5CF6;
  --dazno-accent: #06D6A0;
  --dazno-success: #10B981;
  --dazno-warning: #F59E0B;
  --dazno-danger: #EF4444;
  --dazno-info: #3B82F6;
  
  /* Gradients avancés */
  --gradient-primary: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  --gradient-success: linear-gradient(135deg, #84fab0 0%, #8fd3f4 100%);
  --gradient-premium: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
  
  /* Dark theme */
  --dazno-dark-bg: #0F172A;
  --dazno-dark-surface: #1E293B;
  --dazno-dark-border: #334155;
  --dazno-light: #F8FAFC;
  
  /* Typography scale */
  --font-xs: 0.75rem;
  --font-sm: 0.875rem;
  --font-base: 1rem;
  --font-lg: 1.125rem;
  --font-xl: 1.25rem;
  --font-2xl: 1.5rem;
  --font-3xl: 1.875rem;
  
  /* Spacing scale */
  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  --space-6: 1.5rem;
  --space-8: 2rem;
  --space-12: 3rem;
  
  /* Shadows */
  --shadow-sm: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
  --shadow-xl: 0 20px 25px -5px rgba(0, 0, 0, 0.1);
  
  /* Animations */
  --transition-fast: 150ms ease-in-out;
  --transition-normal: 300ms ease-in-out;
  --transition-slow: 500ms ease-in-out;
}

* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  background: var(--dazno-dark-bg);
  color: var(--dazno-light);
  line-height: 1.6;
  font-size: var(--font-base);
}

.dazno-superior-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* Header avancé */
.dazno-header-advanced {
  background: var(--gradient-primary);
  padding: var(--space-4) var(--space-6);
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: var(--shadow-lg);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--space-4);
}

.header-left h1 {
  font-size: var(--font-2xl);
  font-weight: 700;
  color: white;
  margin: 0;
}

.competitive-badge {
  background: rgba(255, 255, 255, 0.2);
  padding: var(--space-1) var(--space-3);
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

.vs-amboss {
  color: white;
  font-size: var(--font-sm);
  font-weight: 600;
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--space-6);
}

.roi-indicator {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  background: rgba(255, 255, 255, 0.15);
  padding: var(--space-2) var(--space-4);
  border-radius: 16px;
  backdrop-filter: blur(10px);
}

.roi-label {
  font-size: var(--font-sm);
  color: rgba(255, 255, 255, 0.8);
}

.roi-value {
  font-size: var(--font-xl);
  font-weight: 700;
  color: white;
}

.roi-value.positive {
  color: var(--dazno-accent);
}

.roi-value.negative {
  color: var(--dazno-danger);
}

.roi-prediction {
  font-size: var(--font-sm);
  color: var(--dazno-accent);
  font-weight: 600;
}

.status-cluster {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.status-indicator {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: var(--dazno-success);
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.3);
  animation: pulse 2s infinite;
}

.status-indicator.disconnected {
  background: var(--dazno-danger);
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.3);
}

.automation-status {
  background: rgba(255, 255, 255, 0.1);
  padding: var(--space-1) var(--space-3);
  border-radius: 8px;
  font-size: var(--font-sm);
  font-weight: 600;
  color: white;
}

/* Dashboard grid supérieur */
.superior-dashboard-grid {
  display: grid;
  grid-template-columns: 2fr 1fr;
  grid-template-rows: auto auto;
  gap: var(--space-6);
  padding: var(--space-6);
  flex: 1;
}

.performance-command-center {
  grid-column: 1 / -1;
  background: var(--dazno-dark-surface);
  border-radius: 16px;
  padding: var(--space-6);
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--dazno-dark-border);
}

.performance-command-center h2 {
  font-size: var(--font-2xl);
  margin-bottom: var(--space-6);
  color: var(--dazno-light);
}

.metrics-grid-advanced {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.metric-card-pro {
  background: linear-gradient(135deg, rgba(45, 91, 255, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
  border-radius: 12px;
  padding: var(--space-4);
  border: 1px solid rgba(45, 91, 255, 0.2);
  transition: all var(--transition-normal);
  position: relative;
  overflow: hidden;
}

.metric-card-pro::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--gradient-primary);
}

.metric-card-pro.primary {
  background: linear-gradient(135deg, rgba(6, 214, 160, 0.15) 0%, rgba(16, 185, 129, 0.15) 100%);
  border-color: var(--dazno-accent);
}

.metric-card-pro.primary::before {
  background: var(--gradient-success);
}

.metric-card-pro:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-xl);
}

.metric-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-3);
}

.metric-icon {
  font-size: var(--font-lg);
}

.metric-label {
  font-size: var(--font-sm);
  color: rgba(248, 250, 252, 0.8);
  font-weight: 500;
}

.trend-indicator {
  padding: var(--space-1) var(--space-2);
  border-radius: 6px;
  font-size: var(--font-xs);
  font-weight: 600;
}

.trend-indicator.positive {
  background: rgba(16, 185, 129, 0.2);
  color: var(--dazno-success);
}

.trend-indicator.negative {
  background: rgba(239, 68, 68, 0.2);
  color: var(--dazno-danger);
}

.metric-value-large {
  font-size: var(--font-3xl);
  font-weight: 700;
  color: var(--dazno-light);
  margin-bottom: var(--space-2);
}

.metric-value {
  font-size: var(--font-xl);
  font-weight: 700;
  color: var(--dazno-light);
  margin-bottom: var(--space-1);
}

.metric-prediction,
.metric-subtitle {
  font-size: var(--font-sm);
  color: rgba(248, 250, 252, 0.7);
}

.competitive-rank {
  background: var(--gradient-premium);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  font-weight: 700;
  font-size: var(--font-sm);
}

/* Graphique en temps réel */
.real-time-chart-container {
  height: 300px;
  background: rgba(15, 23, 42, 0.5);
  border-radius: 12px;
  padding: var(--space-4);
  border: 1px solid var(--dazno-dark-border);
}

/* Recommandations AI-powered */
.ai-recommendations-panel {
  background: var(--dazno-dark-surface);
  border-radius: 16px;
  padding: var(--space-6);
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--dazno-dark-border);
}

.panel-header-advanced {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-6);
}

.panel-header-advanced h2 {
  font-size: var(--font-2xl);
  color: var(--dazno-light);
}

.ai-status {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.ai-indicator {
  background: rgba(6, 214, 160, 0.2);
  color: var(--dazno-accent);
  padding: var(--space-1) var(--space-3);
  border-radius: 8px;
  font-size: var(--font-sm);
  font-weight: 600;
}

.confidence-score {
  color: rgba(248, 250, 252, 0.8);
  font-size: var(--font-sm);
}

/* Cartes de recommandations avancées */
.recommendation-card-pro {
  background: linear-gradient(135deg, rgba(30, 41, 59, 0.8) 0%, rgba(51, 65, 85, 0.4) 100%);
  border-radius: 12px;
  padding: var(--space-6);
  margin-bottom: var(--space-4);
  border-left: 4px solid var(--dazno-primary);
  transition: all var(--transition-normal);
  position: relative;
}

.recommendation-card-pro:hover {
  transform: translateX(4px);
  box-shadow: var(--shadow-lg);
}

.recommendation-card-pro.priority-high {
  border-left-color: var(--dazno-danger);
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(30, 41, 59, 0.8) 100%);
}

.recommendation-card-pro.priority-medium {
  border-left-color: var(--dazno-warning);
}

.recommendation-card-pro.confidence-high::after {
  content: '🎯';
  position: absolute;
  top: var(--space-4);
  right: var(--space-4);
  font-size: var(--font-lg);
}

.rec-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: var(--space-4);
}

.rec-title-group h3 {
  font-size: var(--font-lg);
  color: var(--dazno-light);
  margin-bottom: var(--space-2);
}

.rec-badges {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.priority-badge,
.confidence-badge,
.advantage-badge {
  padding: var(--space-1) var(--space-2);
  border-radius: 6px;
  font-size: var(--font-xs);
  font-weight: 600;
}

.priority-badge.priority-high {
  background: rgba(239, 68, 68, 0.2);
  color: var(--dazno-danger);
}

.priority-badge.priority-medium {
  background: rgba(245, 158, 11, 0.2);
  color: var(--dazno-warning);
}

.priority-badge.priority-low {
  background: rgba(59, 130, 246, 0.2);
  color: var(--dazno-info);
}

.confidence-badge {
  background: rgba(6, 214, 160, 0.2);
  color: var(--dazno-accent);
}

.advantage-badge {
  background: var(--gradient-premium);
  color: var(--dazno-dark-bg);
}

.rec-impact-group {
  text-align: right;
}

.impact-value {
  font-size: var(--font-2xl);
  font-weight: 700;
  color: var(--dazno-accent);
}

.impact-label {
  font-size: var(--font-sm);
  color: rgba(248, 250, 252, 0.7);
}

.rec-details {
  margin-bottom: var(--space-4);
}

.rec-description {
  color: rgba(248, 250, 252, 0.9);
  line-height: 1.6;
  margin-bottom: var(--space-3);
}

.competitive-insight {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  background: rgba(6, 214, 160, 0.1);
  padding: var(--space-3);
  border-radius: 8px;
  border: 1px solid rgba(6, 214, 160, 0.2);
}

.insight-icon {
  font-size: var(--font-lg);
}

.insight-text {
  color: var(--dazno-accent);
  font-weight: 500;
}

.rec-metrics-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: var(--space-4);
  margin-bottom: var(--space-4);
}

.rec-metric {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.rec-metric .metric-label {
  font-size: var(--font-xs);
  color: rgba(248, 250, 252, 0.6);
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 0.5px;
}

.rec-metric .metric-value {
  font-weight: 600;
  color: var(--dazno-light);
}

.metric-value.risk-low {
  color: var(--dazno-success);
}

.metric-value.risk-medium {
  color: var(--dazno-warning);
}

.metric-value.risk-high {
  color: var(--dazno-danger);
}

.rec-timing-indicator {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  margin-bottom: var(--space-4);
}

.timing-bar {
  flex: 1;
  height: 6px;
  background: rgba(248, 250, 252, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.timing-fill {
  height: 100%;
  background: var(--gradient-success);
  transition: width var(--transition-normal);
}

/* Boutons d'action avancés */
.action-buttons-advanced {
  display: flex;
  gap: var(--space-2);
  flex-wrap: wrap;
}

.btn-auto-execute,
.btn-approve-smart,
.btn-schedule,
.btn-simulate,
.btn-reject-smart {
  padding: var(--space-2) var(--space-4);
  border-radius: 8px;
  border: none;
  font-weight: 600;
  font-size: var(--font-sm);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: var(--space-1);
}

.btn-auto-execute {
  background: var(--gradient-success);
  color: var(--dazno-dark-bg);
}

.btn-approve-smart {
  background: var(--dazno-primary);
  color: white;
}

.btn-schedule {
  background: var(--dazno-warning);
  color: var(--dazno-dark-bg);
}

.btn-simulate {
  background: var(--dazno-secondary);
  color: white;
}

.btn-reject-smart {
  background: transparent;
  color: var(--dazno-danger);
  border: 1px solid var(--dazno-danger);
}

.btn-auto-execute:hover,
.btn-approve-smart:hover,
.btn-schedule:hover,
.btn-simulate:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

.btn-reject-smart:hover {
  background: var(--dazno-danger);
  color: white;
}

/* État sans recommandations */
.no-recommendations-state {
  text-align: center;
  padding: var(--space-12);
  color: rgba(248, 250, 252, 0.7);
}

.state-icon {
  font-size: 4rem;
  margin-bottom: var(--space-4);
}

.no-recommendations-state h3 {
  font-size: var(--font-xl);
  margin-bottom: var(--space-2);
  color: var(--dazno-light);
}

.btn-force-analysis {
  background: var(--dazno-primary);
  color: white;
  padding: var(--space-3) var(--space-6);
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  margin-top: var(--space-4);
  transition: all var(--transition-fast);
}

.btn-force-analysis:hover {
  background: var(--dazno-primary-dark);
  transform: translateY(-1px);
}

/* Panneau d'automatisation */
.automation-control-panel {
  background: var(--dazno-dark-surface);
  border-radius: 16px;
  padding: var(--space-6);
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--dazno-dark-border);
}

.automation-control-panel h2 {
  font-size: var(--font-2xl);
  color: var(--dazno-light);
  margin-bottom: var(--space-6);
}

.automation-stats {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-4);
  margin-bottom: var(--space-6);
}

.auto-metric {
  text-align: center;
  padding: var(--space-4);
  background: rgba(45, 91, 255, 0.1);
  border-radius: 8px;
  border: 1px solid rgba(45, 91, 255, 0.2);
}

.auto-label {
  display: block;
  font-size: var(--font-sm);
  color: rgba(248, 250, 252, 0.7);
  margin-bottom: var(--space-1);
}

.auto-value {
  font-size: var(--font-xl);
  font-weight: 700;
  color: var(--dazno-light);
}

.auto-value.success {
  color: var(--dazno-success);
}

.automation-controls {
  space-y: var(--space-6);
}

.control-group {
  margin-bottom: var(--space-6);
}

.control-group h4 {
  font-size: var(--font-lg);
  color: var(--dazno-light);
  margin-bottom: var(--space-3);
}

/* Toggle switch avancé */
.smart-toggle {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  cursor: pointer;
  margin-bottom: var(--space-4);
}

.smart-toggle input[type="checkbox"] {
  display: none;
}

.toggle-slider {
  position: relative;
  width: 48px;
  height: 24px;
  background: var(--dazno-dark-border);
  border-radius: 12px;
  transition: all var(--transition-normal);
}

.toggle-slider::before {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: all var(--transition-normal);
}

.smart-toggle input[type="checkbox"]:checked + .toggle-slider {
  background: var(--dazno-primary);
}

.smart-toggle input[type="checkbox"]:checked + .toggle-slider::before {
  transform: translateX(24px);
}

.toggle-label {
  color: var(--dazno-light);
  font-weight: 500;
}

/* Contrôles de seuils */
.automation-thresholds {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  padding: var(--space-4);
  background: rgba(15, 23, 42, 0.5);
  border-radius: 8px;
  border: 1px solid var(--dazno-dark-border);
}

.threshold-control {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.threshold-control label {
  min-width: 140px;
  color: rgba(248, 250, 252, 0.8);
  font-size: var(--font-sm);
}

.threshold-control input[type="range"] {
  flex: 1;
  height: 6px;
  background: var(--dazno-dark-border);
  border-radius: 3px;
  outline: none;
}

.threshold-control input[type="range"]::-webkit-slider-thumb {
  appearance: none;
  width: 18px;
  height: 18px;
  background: var(--dazno-primary);
  border-radius: 50%;
  cursor: pointer;
}

.threshold-control select {
  background: var(--dazno-dark-surface);
  color: var(--dazno-light);
  border: 1px solid var(--dazno-dark-border);
  border-radius: 6px;
  padding: var(--space-2) var(--space-3);
  font-size: var(--font-sm);
}

.threshold-value {
  min-width: 50px;
  text-align: right;
  color: var(--dazno-primary);
  font-weight: 600;
}

/* Journal d'automatisation */
.automation-log {
  margin-top: var(--space-6);
}

.automation-log h4 {
  font-size: var(--font-lg);
  color: var(--dazno-light);
  margin-bottom: var(--space-3);
}

.log-entries {
  max-height: 200px;
  overflow-y: auto;
  background: rgba(15, 23, 42, 0.5);
  border-radius: 8px;
  border: 1px solid var(--dazno-dark-border);
}

.log-entry {
  display: grid;
  grid-template-columns: 80px 1fr auto;
  gap: var(--space-3);
  padding: var(--space-3);
  border-bottom: 1px solid var(--dazno-dark-border);
  font-size: var(--font-sm);
}

.log-entry:last-child {
  border-bottom: none;
}

.log-time {
  color: rgba(248, 250, 252, 0.6);
  font-weight: 500;
}

.log-action {
  color: var(--dazno-light);
}

.log-result {
  font-weight: 600;
  text-align: right;
}

.log-result.success {
  color: var(--dazno-success);
}

.log-result.error {
  color: var(--dazno-danger);
}

.log-result.pending {
  color: var(--dazno-warning);
}

/* Panneau de monitoring avancé */
.advanced-monitoring-panel {
  grid-column: 1 / -1;
  background: var(--dazno-dark-surface);
  border-radius: 16px;
  padding: var(--space-6);
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--dazno-dark-border);
}

.advanced-monitoring-panel h2 {
  font-size: var(--font-2xl);
  color: var(--dazno-light);
  margin-bottom: var(--space-6);
}

/* Onglets de monitoring */
.monitoring-tabs {
  display: flex;
  gap: var(--space-2);
  margin-bottom: var(--space-6);
  border-bottom: 1px solid var(--dazno-dark-border);
}

.tab-btn {
  padding: var(--space-3) var(--space-4);
  background: transparent;
  border: none;
  color: rgba(248, 250, 252, 0.7);
  font-weight: 500;
  cursor: pointer;
  border-radius: 8px 8px 0 0;
  transition: all var(--transition-fast);
  position: relative;
}

.tab-btn.active {
  color: var(--dazno-primary);
  background: rgba(45, 91, 255, 0.1);
}

.tab-btn.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--dazno-primary);
}

.tab-btn:hover:not(.active) {
  color: var(--dazno-light);
  background: rgba(248, 250, 252, 0.05);
}

.tab-content {
  display: none;
}

.tab-content.active {
  display: block;
}

/* Graphiques et analytics */
.chart-container,
.prediction-chart-container {
  height: 400px;
  background: rgba(15, 23, 42, 0.5);
  border-radius: 12px;
  padding: var(--space-4);
  border: 1px solid var(--dazno-dark-border);
  margin-bottom: var(--space-6);
}

.performance-insights {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--space-4);
}

.insight-card {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  background: rgba(45, 91, 255, 0.1);
  border-radius: 8px;
  padding: var(--space-4);
  border: 1px solid rgba(45, 91, 255, 0.2);
}

.insight-icon {
  font-size: var(--font-2xl);
}

.insight-content h5 {
  color: var(--dazno-light);
  margin-bottom: var(--space-1);
}

.insight-content p {
  color: rgba(248, 250, 252, 0.7);
  font-size: var(--font-sm);
}

.insight-impact {
  margin-left: auto;
  color: var(--dazno-accent);
  font-weight: 700;
}

/* Analyse concurrentielle */
.competitive-analysis {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-6);
}

.competitor-comparison h4 {
  color: var(--dazno-light);
  margin-bottom: var(--space-4);
}

.position-chart {
  height: 300px;
  background: rgba(15, 23, 42, 0.5);
  border-radius: 8px;
  padding: var(--space-4);
  border: 1px solid var(--dazno-dark-border);
}

.competitive-insights {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}

.competitive-insight-card {
  background: rgba(139, 92, 246, 0.1);
  border-radius: 8px;
  padding: var(--space-4);
  border: 1px solid rgba(139, 92, 246, 0.2);
}

.competitor-name {
  font-weight: 600;
  color: var(--dazno-light);
  margin-bottom: var(--space-2);
}

.performance-diff {
  font-weight: 700;
  margin-bottom: var(--space-1);
}

.performance-diff.positive {
  color: var(--dazno-success);
}

.performance-diff.negative {
  color: var(--dazno-danger);
}

.opportunity {
  font-size: var(--font-sm);
  color: rgba(248, 250, 252, 0.8);
}

/* Dashboard de prédictions */
.predictions-dashboard {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: var(--space-6);
}

.prediction-cards {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
}

.prediction-card {
  background: rgba(6, 214, 160, 0.1);
  border-radius: 8px;
  padding: var(--space-4);
  border: 1px solid rgba(6, 214, 160, 0.2);
  text-align: center;
}

.prediction-card h5 {
  color: var(--dazno-light);
  margin-bottom: var(--space-2);
  font-size: var(--font-sm);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.prediction-value {
  font-size: var(--font-xl);
  font-weight: 700;
  color: var(--dazno-accent);
  margin-bottom: var(--space-1);
}

.prediction-timeframe {
  color: rgba(248, 250, 252, 0.7);
  font-size: var(--font-sm);
  margin-bottom: var(--space-2);
}

.prediction-confidence {
  font-size: var(--font-xs);
  color: var(--dazno-accent);
  font-weight: 600;
}

/* Analytics des canaux */
.channels-analytics {
  overflow-x: auto;
}

.advanced-table {
  width: 100%;
  border-collapse: collapse;
  background: rgba(15, 23, 42, 0.5);
  border-radius: 8px;
  overflow: hidden;
}

.advanced-table th,
.advanced-table td {
  padding: var(--space-3) var(--space-4);
  text-align: left;
  border-bottom: 1px solid var(--dazno-dark-border);
}

.advanced-table th {
  background: rgba(45, 91, 255, 0.1);
  color: var(--dazno-light);
  font-weight: 600;
  font-size: var(--font-sm);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.channel-row.performance-excellent {
  background: rgba(16, 185, 129, 0.05);
}

.channel-row.performance-good {
  background: rgba(6, 214, 160, 0.05);
}

.channel-row.performance-poor {
  background: rgba(239, 68, 68, 0.05);
}

.channel-info {
  display: flex;
  flex-direction: column;
  gap: var(--space-1);
}

.channel-alias {
  color: var(--dazno-light);
  font-weight: 500;
}

.channel-capacity {
  color: rgba(248, 250, 252, 0.6);
  font-size: var(--font-sm);
}

.roi-cell {
  font-weight: 700;
}

.roi-cell.positive {
  color: var(--dazno-success);
}

.roi-cell.negative {
  color: var(--dazno-danger);
}

.success-rate-cell {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.progress-bar {
  flex: 1;
  height: 6px;
  background: rgba(248, 250, 252, 0.1);
  border-radius: 3px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: var(--gradient-success);
  transition: width var(--transition-normal);
}

.rec-badge {
  padding: var(--space-1) var(--space-2);
  border-radius: 4px;
  font-size: var(--font-xs);
  font-weight: 600;
}

.rec-badge.optimize {
  background: rgba(245, 158, 11, 0.2);
  color: var(--dazno-warning);
}

.rec-badge.rebalance {
  background: rgba(59, 130, 246, 0.2);
  color: var(--dazno-info);
}

.no-rec {
  color: var(--dazno-success);
  font-weight: 500;
}

.btn-channel-action {
  padding: var(--space-1) var(--space-2);
  border-radius: 4px;
  border: none;
  font-size: var(--font-xs);
  font-weight: 600;
  cursor: pointer;
  margin-right: var(--space-1);
  transition: all var(--transition-fast);
}

.btn-channel-action.optimize {
  background: var(--dazno-warning);
  color: var(--dazno-dark-bg);
}

.btn-channel-action.details {
  background: var(--dazno-info);
  color: white;
}

.btn-channel-action:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-sm);
}

/* Système de notifications */
.notifications-container {
  position: fixed;
  top: 80px;
  right: var(--space-6);
  z-index: 1000;
  max-width: 400px;
}

.notification {
  background: var(--dazno-dark-surface);
  border-radius: 8px;
  padding: var(--space-4);
  margin-bottom: var(--space-3);
  box-shadow: var(--shadow-xl);
  border-left: 4px solid var(--dazno-primary);
  animation: slideInRight 0.3s ease-out;
}

.notification.success {
  border-left-color: var(--dazno-success);
}

.notification.warning {
  border-left-color: var(--dazno-warning);
}

.notification.error {
  border-left-color: var(--dazno-danger);
}

/* Modal de simulation */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(5px);
  display: none;
  justify-content: center;
  align-items: center;
  z-index: 2000;
}

.modal-overlay.active {
  display: flex;
}

.modal-content {
  background: var(--dazno-dark-surface);
  border-radius: 16px;
  max-width: 800px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: var(--shadow-xl);
  border: 1px solid var(--dazno-dark-border);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-6);
  border-bottom: 1px solid var(--dazno-dark-border);
}

.modal-header h3 {
  color: var(--dazno-light);
  font-size: var(--font-xl);
}

.modal-close {
  background: none;
  border: none;
  color: rgba(248, 250, 252, 0.7);
  font-size: var(--font-2xl);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: all var(--transition-fast);
}

.modal-close:hover {
  background: rgba(248, 250, 252, 0.1);
  color: var(--dazno-light);
}

.modal-body {
  padding: var(--space-6);
}

.modal-footer {
  display: flex;
  gap: var(--space-3);
  justify-content: flex-end;
  padding: var(--space-6);
  border-top: 1px solid var(--dazno-dark-border);
}

.btn-modal {
  padding: var(--space-3) var(--space-6);
  border-radius: 8px;
  border: none;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-modal.primary {
  background: var(--dazno-primary);
  color: white;
}

.btn-modal.secondary {
  background: transparent;
  color: rgba(248, 250, 252, 0.7);
  border: 1px solid var(--dazno-dark-border);
}

.btn-modal:hover {
  transform: translateY(-1px);
  box-shadow: var(--shadow-md);
}

/* Animations */
@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

@keyframes slideInRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

/* Responsive design */
@media (max-width: 1200px) {
  .superior-dashboard-grid {
    grid-template-columns: 1fr;
  }
  
  .competitive-analysis,
  .predictions-dashboard {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .dazno-header-advanced {
    flex-direction: column;
    gap: var(--space-4);
  }
  
  .metrics-grid-advanced {
    grid-template-columns: 1fr;
  }
  
  .rec-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-3);
  }
  
  .action-buttons-advanced {
    flex-direction: column;
  }
  
  .monitoring-tabs {
    flex-wrap: wrap;
  }
  
  .automation-thresholds {
    gap: var(--space-3);
  }
  
  .threshold-control {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-2);
  }
  
  .threshold-control label {
    min-width: auto;
  }
}

/* Utilitaires */
.text-success { color: var(--dazno-success) !important; }
.text-warning { color: var(--dazno-warning) !important; }
.text-danger { color: var(--dazno-danger) !important; }
.text-info { color: var(--dazno-info) !important; }
.text-muted { color: rgba(248, 250, 252, 0.6) !important; }

.bg-success { background-color: rgba(16, 185, 129, 0.1) !important; }
.bg-warning { background-color: rgba(245, 158, 11, 0.1) !important; }
.bg-danger { background-color: rgba(239, 68, 68, 0.1) !important; }
.bg-info { background-color: rgba(59, 130, 246, 0.1) !important; }

.border-success { border-color: var(--dazno-success) !important; }
.border-warning { border-color: var(--dazno-warning) !important; }
.border-danger { border-color: var(--dazno-danger) !important; }
.border-info { border-color: var(--dazno-info) !important; }
```

Cette interface surpasse largement Amboss.space avec :

**🎯 Avantages concurrentiels uniques :**
- Interface temps réel avec prédictions ML
- Automatisation intelligente avec seuils configurables  
- Analyse concurrentielle en direct
- Système de recommandations multi-objectifs
- Monitoring post-exécution avancé

**🚀 Fonctionnalités exclusives :**
- Simulation des recommandations avant exécution
- Scheduling optimal basé sur timing de marché
- Score de confiance et avantage concurrentiel
- Automation avec rollback automatique
- Analytics prédictifs sur 30 jours

Voulez-vous que je continue avec le JavaScript avancé pour rendre cette interface interactive ?# Spécifications Fonctionnelles et Techniques - Dazno Umbrel App

## 1. Vue d'ensemble du projet

### 1.1 Objectif
Développer une application Umbrel native permettant aux utilisateurs d'optimiser le ROI de leurs nœuds Lightning Network en intégrant les recommandations du MCP Dazno.de accessible via `api.dazno.de`.

### 1.2 Architecture générale
```
[Umbrel OS] ↔ [Dazno App Container] ↔ [Local LND] ↔ [Local Lightning Apps]
                     ↕
            [api.dazno.de MCP] (recommandations uniquement)
```

### 1.3 Applications Umbrel utilisées
- **LND** : Nœud Lightning principal
- **Lightning Terminal** : Interface avancée LND
- **Lightning Node** : Interface de base
- **Electrs** : Serveur Electrum pour données blockchain
- **Bitcoin Node** : Nœud Bitcoin principal

## 2. Spécifications fonctionnelles

### 2.1 Fonctionnalités principales (améliorées vs Amboss)

#### 2.1.1 Dashboard intelligent avancé
- **Vue d'ensemble surpuissante**
  - ROI en temps réel avec prédictions ML
  - Comparaison automatique vs concurrents
  - Score de performance composite
  - Graphiques prédictifs et tendances
  - Alertes proactives basées sur IA

#### 2.1.2 Système de recommandations ML-powered
- **Intelligence supérieure à Magma AI d'Amboss**
  - Analyse prédictive des opportunités futures
  - Optimisation multi-objectifs (ROI + fiabilité + croissance)
  - Recommandations personnalisées par profil de risque
  - Timing optimal basé sur conditions de marché
  - Analyse concurrentielle en temps réel

#### 2.1.3 Automatisation avancée
- **Exécution automatique intelligente**
  - Validation automatique avec seuils configurables
  - Exécution conditionnelle basée sur triggers
  - Monitoring post-exécution avec rollback automatique
  - Optimisation continue des stratégies
  - Apprentissage des préférences utilisateur

#### 2.1.4 Fonctionnalités exclusives Dazno
- **Avantages concurrentiels uniques**
  - Arbitrage de liquidité inter-nœuds
  - Stratégies de fees dynamiques en temps réel
  - Prédiction des pannes de pairs
  - Optimisation de position réseau
  - Analytics de rentabilité par canal

#### 2.1.5 Intégrations étendues
- **Écosystème Lightning complet**
  - Integration Pool/Loop automatisée
  - Gestion Submarine Swaps intelligente
  - Coordination multi-nœuds (pour flottes)
  - API webhooks pour intégrations externes
  - Support des LSP (Lightning Service Providers)

### 2.2 Interface utilisateur

#### 2.2.1 Design system
- **Base sur le design de master.dazno.de**
  - Palette de couleurs cohérente
  - Typographie et spacing identiques
  - Composants UI réutilisables
  - Responsive design pour différentes tailles d'écran

#### 2.2.2 Pages principales
1. **Dashboard** - Vue d'ensemble et métriques
2. **Recommendations** - Liste des actions suggérées
3. **History** - Historique des actions et résultats
4. **Settings** - Configuration de l'application
5. **Logs** - Journaux système et debug

## 3. Spécifications techniques

### 3.1 Stack technologique

#### 3.1.1 Backend (Rust)
```rust
// Structure principale
dazno-umbrel/
├── src/
│   ├── main.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── mcp_client.rs
│   │   └── lightning_client.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── recommendation.rs
│   │   └── metrics.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── dashboard.rs
│   │   └── actions.rs
│   └── utils/
│       ├── mod.rs
│       └── config.rs
├── templates/
├── static/
├── Dockerfile
└── umbrel-app.yml
```

### 3.6 Dépendances Rust mises à jour

#### 3.6.1 Cargo.toml avec intégrations locales
```toml
[package]
name = "dazno-umbrel"
version = "1.0.0"
edition = "2021"

[dependencies]
# Core async runtime
tokio = { version = "1.0", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP client/server
reqwest = { version = "0.11", features = ["json", "stream"] }
axum = { version = "0.7", features = ["ws", "headers"] }

# Lightning Network
tonic-lnd = { version = "0.6", features = ["rustls"] }
lightning = "0.0.118"
bitcoin = "0.31"

# Database
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-rustls", "chrono", "uuid"] }

# Utilities
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
hex = "0.4"
base64 = "0.22"

# Configuration et logging
config = "0.13"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Templates et static files
handlebars = "4.0"
tower-http = { version = "0.5", features = ["fs", "trace"] }
tower = { version = "0.4", features = ["util"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tokio-test = "0.4"
```

### 3.2 Intégration Umbrel

#### 3.2.1 Structure du projet Umbrel
```
umbrel-apps/
└── dazno/
    ├── umbrel-app.yml
    ├── docker-compose.yml
    ├── Dockerfile
    └── src/
```

#### 3.2.2 Configuration umbrel-app.yml (mise à jour)
```yaml
manifestVersion: 1
id: dazno
category: lightning
name: Dazno Lightning ROI Optimizer
version: "1.0.0"
tagline: Optimize your Lightning Network node ROI using local Umbrel apps
description: >-
  Dazno analyzes your Lightning node performance using local Umbrel applications 
  (LND, Lightning Terminal, Electrs) and provides actionable recommendations to 
  maximize your return on investment through automated channel management and 
  routing optimization. All actions are executed locally for maximum security.
developer: Dazno
website: https://dazno.de
dependencies:
  - lightning-node
  - lightning-terminal
  - electrs
  - bitcoin
repo: https://github.com/dazno/umbrel-app
support: https://github.com/dazno/umbrel-app/issues
port: 3000
gallery:
  - 1.jpg
  - 2.jpg
  - 3.jpg
path: ""
defaultUsername: ""
defaultPassword: ""
submitter: Dazno Team
submission: https://github.com/getumbrel/umbrel-apps/pull/xxx
```

#### 3.2.3 Docker Compose (intégration apps locales)
```yaml
version: "3.7"

services:
  app_proxy:
    environment:
      APP_HOST: dazno_web_1
      APP_PORT: 3000

  web:
    build: .
    restart: on-failure
    environment:
      # API externe pour recommandations seulement
      - MCP_API_URL=https://api.dazno.de
      
      # Connexions locales Umbrel
      - LND_HOST=umbrel.local
      - LND_GRPC_PORT=10009
      - LND_REST_PORT=8080
      - LIGHTNING_TERMINAL_URL=http://lightning-terminal_web_1:3004
      - ELECTRS_URL=http://electrs_web_1:3002
      - BITCOIN_RPC_URL=http://bitcoin_bitcoind_1:8332
      
      # Credentials locaux
      - LND_MACAROON_PATH=/lnd/data/chain/bitcoin/mainnet/admin.macaroon
      - LND_TLS_CERT_PATH=/lnd/tls.cert
      - BITCOIN_RPC_USER=${BITCOIN_RPC_USER}
      - BITCOIN_RPC_PASS=${BITCOIN_RPC_PASS}
    volumes:
      # Accès aux données locales Umbrel
      - ${APP_LIGHTNING_NODE_DATA_DIR}:/lnd:ro
      - ${APP_BITCOIN_DATA_DIR}:/bitcoin:ro
      - ./data:/app/data
    networks:
      default:
        ipv4_address: $APP_DAZNO_IP
    depends_on:
      - lightning-terminal_web_1
      - electrs_web_1
```

### 3.3 Intégration avec les applications Umbrel locales

#### 3.3.1 Client LND local (Rust)
```rust
use tonic_lnd::lnrpc::{GetInfoRequest, OpenChannelRequest, CloseChannelRequest};
use tonic_lnd::{Client, MacaroonCredentials, Certificate};
use std::path::Path;

pub struct LocalLightningClient {
    client: Client,
}

impl LocalLightningClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Connexion au LND local d'Umbrel
        let cert_path = "/lnd/tls.cert";
        let macaroon_path = "/lnd/data/chain/bitcoin/mainnet/admin.macaroon";
        
        let cert = Certificate::from_pem_file(cert_path)?;
        let macaroon = MacaroonCredentials::from_file(macaroon_path)?;
        
        let client = tonic_lnd::connect_with_macaroon(
            "https://umbrel.local:10009",
            cert,
            macaroon,
        ).await?;
        
        Ok(Self { client })
    }

    pub async fn get_local_node_info(&mut self) -> Result<LocalNodeInfo, Box<dyn std::error::Error>> {
        let response = self.client.lightning()
            .get_info(GetInfoRequest {})
            .await?;
            
        let info = response.into_inner();
        Ok(LocalNodeInfo {
            pubkey: info.identity_pubkey,
            alias: info.alias,
            num_channels: info.num_active_channels,
            local_balance: info.local_balance,
            remote_balance: info.remote_balance,
            // Utilisation des données locales d'Umbrel
        })
    }

    pub async fn execute_local_action(&mut self, action: LocalAction) -> Result<ActionResult, Box<dyn std::error::Error>> {
        match action.action_type {
            LocalActionType::OpenChannel => {
                let params = serde_json::from_value::<OpenChannelParams>(action.parameters)?;
                self.open_channel_local(params).await
            },
            LocalActionType::CloseChannel => {
                let params = serde_json::from_value::<CloseChannelParams>(action.parameters)?;
                self.close_channel_local(params).await
            },
            LocalActionType::UpdateFees => {
                self.update_channel_fees_local(action.parameters).await
            },
        }
    }

    async fn open_channel_local(&mut self, params: OpenChannelParams) -> Result<ActionResult, Box<dyn std::error::Error>> {
        let request = OpenChannelRequest {
            node_pubkey: hex::decode(&params.node_pubkey)?,
            local_funding_amount: params.amount,
            push_sat: params.push_amount.unwrap_or(0),
            sat_per_byte: params.fee_rate,
            ..Default::default()
        };

        let response = self.client.lightning()
            .open_channel_sync(request)
            .await?;

        Ok(ActionResult {
            success: true,
            transaction_id: Some(hex::encode(response.into_inner().funding_txid_bytes)),
            message: "Channel opened successfully using local LND".to_string(),
        })
    }
}
```

#### 3.3.2 Intégration Lightning Terminal local
```rust
use reqwest::Client;

pub struct LocalLightningTerminalClient {
    client: Client,
    base_url: String,
}

impl LocalLightningTerminalClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://lightning-terminal_web_1:3004".to_string(),
        }
    }

    pub async fn get_pool_orders(&self) -> Result<Vec<PoolOrder>, Box<dyn std::error::Error>> {
        // Récupération des ordres Pool depuis Lightning Terminal local
        let url = format!("{}/api/pool/orders", self.base_url);
        let response = self.client.get(&url).send().await?;
        let orders = response.json::<Vec<PoolOrder>>().await?;
        Ok(orders)
    }

    pub async fn get_loop_swaps(&self) -> Result<Vec<LoopSwap>, Box<dyn std::error::Error>> {
        // Récupération des swaps Loop depuis Lightning Terminal local
        let url = format!("{}/api/loop/swaps", self.base_url);
        let response = self.client.get(&url).send().await?;
        let swaps = response.json::<Vec<LoopSwap>>().await?;
        Ok(swaps)
    }
}
```

#### 3.3.3 Client Electrs local pour données blockchain
```rust
pub struct LocalElectrsClient {
    client: Client,
    base_url: String,
}

impl LocalElectrsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "http://electrs_web_1:3002".to_string(),
        }
    }

    pub async fn get_mempool_info(&self) -> Result<MempoolInfo, Box<dyn std::error::Error>> {
        let url = format!("{}/api/mempool", self.base_url);
        let response = self.client.get(&url).send().await?;
        let mempool = response.json::<MempoolInfo>().await?;
        Ok(mempool)
    }

    pub async fn get_fee_estimates(&self) -> Result<FeeEstimates, Box<dyn std::error::Error>> {
        let url = format!("{}/api/fee-estimates", self.base_url);
        let response = self.client.get(&url).send().await?;
        let fees = response.json::<FeeEstimates>().await?;
        Ok(fees)
    }
}
```

### 3.4 API Dazno.de - Endpoints analysés et améliorés

#### 3.4.1 Endpoints API api.dazno.de (supposés basés sur les standards)
```rust
// Endpoints probables basés sur l'analyse d'Amboss et des standards Lightning
pub const DAZNO_ENDPOINTS: &[&str] = &[
    "/api/v1/node/{pubkey}/analysis",           // Analyse complète du nœud
    "/api/v1/node/{pubkey}/recommendations",    // Recommandations ROI
    "/api/v1/node/{pubkey}/channels/optimize",  // Optimisation des canaux
    "/api/v1/node/{pubkey}/liquidity/analysis", // Analyse de liquidité
    "/api/v1/network/opportunities",            // Opportunités réseau
    "/api/v1/routing/efficiency",               // Efficacité de routage
    "/api/v1/fees/optimization",                // Optimisation des frais
    "/api/v1/rebalance/suggestions",            // Suggestions de rebalancement
];

#[derive(Serialize, Deserialize)]
pub struct DaznoNodeAnalysis {
    pub node_pubkey: String,
    pub roi_metrics: ROIMetrics,
    pub channel_performance: Vec<ChannelPerformance>,
    pub liquidity_efficiency: LiquidityMetrics,
    pub routing_stats: RoutingStatistics,
    pub network_position: NetworkPosition,
    pub optimization_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ROIMetrics {
    pub current_roi_annualized: f64,
    pub potential_roi_improvement: f64,
    pub capital_efficiency: f64,
    pub fee_earnings_30d: u64, // sats
    pub routing_success_rate: f64,
    pub channel_utilization: f64,
}

#[derive(Serialize, Deserialize)]
pub struct EnhancedRecommendation {
    pub id: String,
    pub action_type: AdvancedActionType,
    pub priority: Priority,
    pub expected_roi_impact: f64,
    pub confidence_score: f64,
    pub implementation_complexity: Complexity,
    pub estimated_execution_time: chrono::Duration,
    pub capital_requirement: u64, // sats
    pub risk_assessment: RiskLevel,
    pub parameters: serde_json::Value,
    pub competitive_advantage: Option<String>,
    pub market_timing_score: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum AdvancedActionType {
    // Actions de base
    OpenChannel,
    CloseChannel,
    AdjustFees,
    RebalanceChannel,
    
    // Actions avancées (comme Amboss)
    LiquidityOptimization,
    RoutingImprovement,
    CapitalReallocation,
    NetworkPositioning,
    AutomatedRebalancing,
    
    // Actions ML-powered (surpasser Amboss)
    PredictiveChannelManagement,
    MarketTimingOptimization,
    CompetitorAnalysis,
    LiquidityArbitrage,
    DynamicFeeStrategy,
    NetworkEfficiencyBoost,
}

#[derive(Serialize, Deserialize)]
pub struct LocalNodeData {
    pub node_info: LocalNodeInfo,
    pub channels: Vec<ChannelInfo>,
    pub payments: Vec<PaymentInfo>,
    pub invoices: Vec<InvoiceInfo>,
    pub mempool_info: MempoolInfo,
    pub fee_estimates: FeeEstimates,
    // Données étendues pour surpasser Amboss
    pub routing_history: Vec<RoutingEvent>,
    pub peer_performance: Vec<PeerMetrics>,
    pub liquidity_events: Vec<LiquidityEvent>,
    pub market_data: MarketContext,
}

#### 3.4.2 Client Dazno amélioré (surpasser Amboss)
```rust
pub struct SuperiorDaznoClient {
    mcp_client: Client,
    local_lnd: LocalLightningClient,
    local_terminal: LocalLightningTerminalClient,
    local_electrs: LocalElectrsClient,
    amboss_analyzer: Option<AmbossCompatAnalyzer>, // Analyser Amboss pour comparaison
}

impl SuperiorDaznoClient {
    pub async fn get_advanced_recommendations(&mut self) -> Result<Vec<EnhancedRecommendation>, Box<dyn std::error::Error>> {
        // 1. Collecter des données plus riches que Amboss
        let comprehensive_data = self.collect_comprehensive_data().await?;
        
        // 2. Analyser en temps réel vs Amboss (si disponible)
        let competitive_analysis = self.analyze_vs_competition(comprehensive_data.clone()).await?;
        
        // 3. Envoyer au MCP Dazno pour analyse ML avancée
        let ml_recommendations = self.get_ml_powered_recommendations(comprehensive_data).await?;
        
        // 4. Post-traitement avec intelligence locale
        let optimized_recommendations = self.apply_local_intelligence(ml_recommendations).await?;
        
        Ok(optimized_recommendations)
    }

    async fn collect_comprehensive_data(&mut self) -> Result<ComprehensiveNodeData, Box<dyn std::error::Error>> {
        // Données étendues bien au-delà d'Amboss
        let (
            node_info, 
            channels, 
            routing_history, 
            peer_metrics,
            mempool_info, 
            fee_estimates,
            market_context,
            competitor_data
        ) = tokio::try_join!(
            self.local_lnd.get_detailed_node_info(),
            self.local_lnd.get_channels_with_performance(),
            self.local_lnd.get_routing_history(30), // 30 jours d'historique
            self.analyze_peer_performance(),
            self.local_electrs.get_mempool_info(),
            self.local_electrs.get_fee_estimates(),
            self.gather_market_context(),
            self.analyze_competitor_nodes() // Analyse de la concurrence
        )?;

        Ok(ComprehensiveNodeData {
            node_info,
            channels,
            routing_history,
            peer_metrics,
            mempool_info,
            fee_estimates,
            market_context,
            competitor_data,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn get_ml_powered_recommendations(&self, data: ComprehensiveNodeData) -> Result<Vec<EnhancedRecommendation>, Box<dyn std::error::Error>> {
        let url = "https://api.dazno.de/api/v1/analyze/advanced";
        
        let enhanced_request = AdvancedAnalysisRequest {
            node_data: data,
            analysis_depth: AnalysisDepth::Deep,
            optimization_goals: vec![
                OptimizationGoal::MaximizeROI,
                OptimizationGoal::ImproveRoutingSuccess,
                OptimizationGoal::OptimizeLiquidity,
                OptimizationGoal::BeatCompetition,
            ],
            time_horizon: TimeHorizon::OneMonth,
            risk_tolerance: RiskTolerance::Moderate,
        };

        let response = self.mcp_client
            .post(url)
            .json(&enhanced_request)
            .send()
            .await?;

        let recommendations = response.json::<Vec<EnhancedRecommendation>>().await?;
        Ok(recommendations)
    }

    async fn apply_local_intelligence(&self, recommendations: Vec<EnhancedRecommendation>) -> Result<Vec<EnhancedRecommendation>, Box<dyn std::error::Error>> {
        let mut optimized = Vec::new();
        
        for rec in recommendations {
            // Validation avec contraintes locales
            if !self.is_recommendation_feasible(&rec).await? {
                continue;
            }
            
            // Enrichissement avec données locales en temps réel
            let mut enhanced_rec = rec;
            enhanced_rec.confidence_score *= self.calculate_local_confidence_boost(&enhanced_rec).await?;
            enhanced_rec.market_timing_score = self.assess_market_timing(&enhanced_rec).await?;
            
            // Ajout d'avantage concurrentiel
            enhanced_rec.competitive_advantage = self.calculate_competitive_advantage(&enhanced_rec).await?;
            
            optimized.push(enhanced_rec);
        }
        
        // Tri par score de valeur composite
        optimized.sort_by(|a, b| {
            let score_a = a.expected_roi_impact * a.confidence_score * a.market_timing_score;
            let score_b = b.expected_roi_impact * b.confidence_score * b.market_timing_score;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(optimized)
    }

    // Fonctionnalités exclusives Dazno (surpasser Amboss)
    async fn analyze_competitor_nodes(&self) -> Result<Vec<CompetitorMetrics>, Box<dyn std::error::Error>> {
        // Analyse des nœuds concurrents dans la même région/niche
        // Plus avancé que le simple "gossip" d'Amboss
        let url = "https://api.dazno.de/api/v1/competitive/analysis";
        let response = self.mcp_client.get(url).send().await?;
        let competitors = response.json::<Vec<CompetitorMetrics>>().await?;
        Ok(competitors)
    }

    async fn execute_superior_automation(&mut self, rec_id: &str) -> Result<SuperiorActionResult, Box<dyn std::error::Error>> {
        let recommendation = self.get_recommendation_by_id(rec_id).await?;
        
        let result = match recommendation.action_type {
            AdvancedActionType::PredictiveChannelManagement => {
                self.execute_predictive_channel_strategy(recommendation.parameters).await?
            },
            AdvancedActionType::LiquidityArbitrage => {
                self.execute_liquidity_arbitrage(recommendation.parameters).await?
            },
            AdvancedActionType::DynamicFeeStrategy => {
                self.implement_dynamic_fee_strategy(recommendation.parameters).await?
            },
            AdvancedActionType::NetworkEfficiencyBoost => {
                self.optimize_network_position(recommendation.parameters).await?
            },
            // Actions standards
            _ => {
                self.local_lnd.execute_local_action(recommendation.into()).await?.into()
            }
        };

        // Monitoring post-exécution plus avancé qu'Amboss
        self.start_advanced_monitoring(&result).await?;
        
        Ok(result)
    }

    async fn execute_predictive_channel_strategy(&self, params: serde_json::Value) -> Result<SuperiorActionResult, Box<dyn std::error::Error>> {
        let strategy_params = serde_json::from_value::<PredictiveChannelParams>(params)?;
        
        // Analyse prédictive basée sur ML et tendances historiques
        let predicted_performance = self.predict_channel_performance(&strategy_params).await?;
        
        if predicted_performance.success_probability > 0.8 {
            // Exécution de la stratégie optimisée
            let channels_to_open = strategy_params.target_channels;
            let mut results = Vec::new();
            
            for channel_spec in channels_to_open {
                let result = self.local_lnd.open_optimized_channel(channel_spec).await?;
                results.push(result);
            }
            
            Ok(SuperiorActionResult {
                success: true,
                execution_details: serde_json::to_value(results)?,
                predicted_impact: predicted_performance.roi_improvement,
                monitoring_id: uuid::Uuid::new_v4().to_string(),
                message: "Predictive channel strategy executed successfully".to_string(),
            })
        } else {
            Ok(SuperiorActionResult {
                success: false,
                execution_details: serde_json::Value::Null,
                predicted_impact: 0.0,
                monitoring_id: String::new(),
                message: format!("Strategy rejected - low success probability: {}", predicted_performance.success_probability),
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ComprehensiveNodeData {
    pub node_info: LocalNodeInfo,
    pub channels: Vec<ChannelInfo>,
    pub routing_history: Vec<RoutingEvent>,
    pub peer_metrics: Vec<PeerMetrics>,
    pub mempool_info: MempoolInfo,
    pub fee_estimates: FeeEstimates,
    pub market_context: MarketContext,
    pub competitor_data: Vec<CompetitorMetrics>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct SuperiorActionResult {
    pub success: bool,
    pub execution_details: serde_json::Value,
    pub predicted_impact: f64,
    pub monitoring_id: String,
    pub message: String,
}
```

#### 3.4.2 Structures de données adaptées
```rust
#[derive(Serialize, Deserialize)]
pub struct MCPRecommendation {
    pub id: String,
    pub action_type: ActionType,
    pub priority: Priority,
    pub expected_roi_impact: f64,
    pub parameters: serde_json::Value,
    pub feasibility_score: f64,
    pub local_constraints: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    OpenChannel,
    CloseChannel,
    AdjustFees,
    RebalanceChannel,
    LoopOut,
    LoopIn,
    PoolOrder,
}

#[derive(Serialize, Deserialize)]
pub struct LocalAction {
    pub recommendation_id: String,
    pub action_type: LocalActionType,
    pub parameters: serde_json::Value,
    pub estimated_duration: chrono::Duration,
    pub required_apps: Vec<String>, // ["lnd", "lightning-terminal", "electrs"]
}
```

### 3.5 Interface Web (Frontend intégré)

#### 3.5.1 Templates Handlebars
```html
<!-- templates/dashboard.hbs -->
<!DOCTYPE html>
<html>
<head>
    <title>Dazno - Lightning ROI Optimizer</title>
    <link rel="stylesheet" href="/static/css/dazno-theme.css">
    <script src="/static/js/dashboard.js"></script>
</head>
<body>
    <div class="dazno-container">
        <header class="dazno-header">
            <h1>Lightning ROI Dashboard</h1>
            <div class="status-indicator {{connection_status}}"></div>
        </header>
        
        <main class="dashboard-grid">
            <section class="metrics-panel">
                <h2>Node Metrics</h2>
                <div class="metric-card">
                    <span class="metric-label">Current ROI</span>
                    <span class="metric-value">{{current_roi}}%</span>
                </div>
                <!-- Plus de métriques -->
            </section>
            
            <section class="recommendations-panel">
                <h2>Active Recommendations</h2>
                {{#each recommendations}}
                <div class="recommendation-card priority-{{priority}}">
                    <h3>{{action_type}}</h3>
                    <p>{{description}}</p>
                    <div class="impact-estimate">+{{expected_roi_impact}}% ROI</div>
                    <div class="action-buttons">
                        <button class="btn-approve" data-id="{{id}}">Approve</button>
                        <button class="btn-reject" data-id="{{id}}">Reject</button>
                    </div>
                </div>
                {{/each}}
            </section>
        </main>
    </div>
</body>
</html>
```

#### 3.5.2 CSS Theme (basé sur dazno.de)
```css
/* static/css/dazno-theme.css */
:root {
  --dazno-primary: #2D5BFF;
  --dazno-secondary: #1E40AF;
  --dazno-success: #10B981;
  --dazno-warning: #F59E0B;
  --dazno-danger: #EF4444;
  --dazno-dark: #1F2937;
  --dazno-light: #F9FAFB;
}

.dazno-container {
  font-family: 'Inter', sans-serif;
  background: var(--dazno-light);
  min-height: 100vh;
}

.dazno-header {
  background: var(--dazno-primary);
  color: white;
  padding: 1rem 2rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  padding: 2rem;
}

.metric-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.recommendation-card {
  background: white;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 1rem;
  border-left: 4px solid var(--dazno-primary);
}

.recommendation-card.priority-high {
  border-left-color: var(--dazno-danger);
}

.btn-approve {
  background: var(--dazno-success);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}

.btn-reject {
  background: var(--dazno-danger);
  color: white;
  border: none;
  padding: 0.5rem 1rem;
  border-radius: 4px;
  cursor: pointer;
}
```

## 4. Installation et déploiement

### 4.1 Processus d'installation

#### 4.1.1 Clonage du repository Umbrel Apps
```bash
# Sur votre Mac avec Visual Studio Code
git clone https://github.com/getumbrel/umbrel-apps.git
cd umbrel-apps
```

#### 4.1.2 Création de l'app Dazno
```bash
mkdir dazno
cd dazno

# Initialisation du projet Rust
cargo init --name dazno-umbrel
```

#### 4.1.3 Structure de développement
```
dazno/
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
├── umbrel-app.yml
├── src/
│   └── main.rs
├── templates/
├── static/
│   ├── css/
│   ├── js/
│   └── images/
└── README.md
```

### 4.2 Dockerfile optimisé
```dockerfile
# Multi-stage build pour optimiser la taille
FROM rust:1.75-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/dazno-umbrel ./
COPY templates/ ./templates/
COPY static/ ./static/

EXPOSE 3000

CMD ["./dazno-umbrel"]
```

## 5. Sécurité et bonnes pratiques

### 5.1 Sécurité
- **Authentification API** : Gestion sécurisée des clés API
- **Validation des données** : Vérification stricte des recommandations MCP
- **Chiffrement** : Communication TLS avec le MCP
- **Permissions** : Accès limité aux ressources Lightning

### 5.2 Monitoring et logs
```rust
use tracing::{info, warn, error};

// Configuration du logging
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

// Usage dans le code
info!("Successfully connected to MCP API");
warn!("High-risk recommendation detected: {}", recommendation.id);
error!("Failed to execute Lightning command: {}", error);
```

### 5.3 Tests et qualité
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_client_connection() {
        let client = MCPClient::new("https://api.dazno.de".to_string(), None);
        // Tests de connexion
    }

    #[tokio::test]
    async fn test_recommendation_parsing() {
        // Tests de parsing des recommandations
    }
}
```

## 6. Roadmap de développement

### 6.1 Phase 1 - MVP (2 semaines)
- [ ] Configuration de base Umbrel
- [ ] Client MCP fonctionnel
- [ ] Interface dashboard simple
- [ ] Actions de base (ouverture/fermeture canaux)

### 6.2 Phase 2 - Amélioration UX (1 semaine)
- [ ] Interface utilisateur complète
- [ ] Système de notifications
- [ ] Historique des actions
- [ ] Graphiques et métriques avancées

### 6.3 Phase 3 - Optimisation (1 semaine)
- [ ] Performance et optimisation
- [ ] Tests complets
- [ ] Documentation utilisateur
- [ ] Préparation soumission App Store

## 7. Ressources de développement

### 7.1 Références Umbrel
- [Documentation officielle Umbrel Apps](https://github.com/getumbrel/umbrel-apps)
- [Examples d'applications existantes](https://github.com/getumbrel/umbrel-apps/tree/master)
- [Guidelines de soumission](https://github.com/getumbrel/umbrel-apps/blob/master/CONTRIBUTING.md)

### 7.2 Outils de développement
- **Visual Studio Code** avec extensions Rust
- **Claude Code** pour assistance développement
- **Docker Desktop** pour tests locaux
- **Postman** pour tests API MCP

Cette spécification vous donne une base complète pour développer votre application Dazno pour Umbrel. Souhaitez-vous que je détaille certains aspects spécifiques ou que j'ajoute des éléments particuliers ?