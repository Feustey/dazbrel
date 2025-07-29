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
        const competitorCtx = document.getElementById('competitorChart');
        if (competitorCtx) {
            this.charts.competitor = new Chart(competitorCtx, {
                type: 'radar',
                data: {
                    labels: ['Accuracy', 'Speed', 'ROI', 'Features', 'UX', 'Automation'],
                    datasets: [{
                        label: 'Dazno Pro',
                        data: [95, 98, 92, 96, 94, 99],
                        borderColor: '#06D6A0',
                        backgroundColor: 'rgba(6, 214, 160, 0.2)',
                        pointBackgroundColor: '#06D6A0'
                    }, {
                        label: 'Amboss Magma',
                        data: [85, 82, 78, 88, 80, 75],
                        borderColor: '#64748B',
                        backgroundColor: 'rgba(100, 116, 139, 0.1)',
                        pointBackgroundColor: '#64748B'
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
                        r: {
                            ticks: { color: '#94A3B8' },
                            grid: { color: '#334155' },
                            pointLabels: { color: '#F8FAFC' }
                        }
                    }
                }
            });
        }
    }

    // Event listeners pour l'interactivité avancée
    setupEventListeners() {
        // Actions des recommandations
        document.addEventListener('click', (event) => {
            if (event.target.classList.contains('btn-auto-execute')) {
                this.autoExecuteRecommendation(event.target.dataset.id);
            } else if (event.target.classList.contains('btn-approve-smart')) {
                this.approveRecommendation(event.target.dataset.id);
            } else if (event.target.classList.contains('btn-simulate')) {
                this.simulateRecommendation(event.target.dataset.id);
            } else if (event.target.classList.contains('btn-schedule')) {
                this.scheduleRecommendation(event.target.dataset.id);
            } else if (event.target.classList.contains('btn-reject-smart')) {
                this.rejectRecommendation(event.target.dataset.id);
            }
        });

        // Contrôles d'automatisation
        const automationMode = document.getElementById('automationMode');
        if (automationMode) {
            automationMode.addEventListener('change', (e) => {
                this.updateAutomationMode(e.target.value);
            });
        }

        const maxActions = document.getElementById('maxActions');
        if (maxActions) {
            maxActions.addEventListener('input', (e) => {
                this.updateMaxActions(e.target.value);
                document.querySelector('.range-value').textContent = e.target.value;
            });
        }

        const autoExecution = document.getElementById('autoExecution');
        if (autoExecution) {
            autoExecution.addEventListener('change', (e) => {
                this.toggleAutoExecution(e.target.checked);
            });
        }

        // Force analysis button
        const forceAnalysis = document.querySelector('.btn-force-analysis');
        if (forceAnalysis) {
            forceAnalysis.addEventListener('click', () => {
                this.forceDeepAnalysis();
            });
        }
    }

    // Actions avancées sur les recommandations
    async autoExecuteRecommendation(id) {
        this.showNotification('⚡ Auto-executing recommendation...', 'info');
        
        try {
            const response = await fetch('/api/recommendations/auto-execute', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ 
                    recommendation_id: id,
                    execution_mode: 'auto'
                })
            });

            const result = await response.json();
            
            if (result.success) {
                this.showNotification(`✅ Auto-execution successful! ROI impact: +${result.roi_impact}%`, 'success');
                this.removeRecommendationCard(id);
                this.updateAutomationStats(result.stats);
            } else {
                this.showNotification(`❌ Auto-execution failed: ${result.message}`, 'error');
            }
        } catch (error) {
            this.showNotification('🚨 Network error during auto-execution', 'error');
        }
    }

    async simulateRecommendation(id) {
        this.isSimulating = true;
        this.showNotification('🎯 Running simulation...', 'info');
        
        try {
            const response = await fetch('/api/recommendations/simulate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ recommendation_id: id })
            });

            const simulation = await response.json();
            this.showSimulationModal(simulation);
            
        } catch (error) {
            this.showNotification('🚨 Simulation failed', 'error');
        } finally {
            this.isSimulating = false;
        }
    }

    async scheduleRecommendation(id) {
        const optimalTime = await this.getOptimalExecutionTime(id);
        
        const scheduleTime = prompt(`⏰ Schedule execution for optimal time: ${optimalTime}?\n\nOr enter custom time (YYYY-MM-DD HH:mm):`);
        
        if (scheduleTime) {
            try {
                const response = await fetch('/api/recommendations/schedule', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ 
                        recommendation_id: id,
                        scheduled_time: scheduleTime === 'optimal' ? optimalTime : scheduleTime
                    })
                });

                const result = await response.json();
                
                if (result.success) {
                    this.showNotification(`⏰ Recommendation scheduled for ${result.scheduled_time}`, 'success');
                } else {
                    this.showNotification(`❌ Scheduling failed: ${result.message}`, 'error');
                }
            } catch (error) {
                this.showNotification('🚨 Network error during scheduling', 'error');
            }
        }
    }

    async getOptimalExecutionTime(id) {
        try {
            const response = await fetch(`/api/recommendations/${id}/optimal-time`);
            const data = await response.json();
            return data.optimal_time;
        } catch (error) {
            return 'Unknown';
        }
    }

    // Simulation modal
    showSimulationModal(simulation) {
        const modal = document.getElementById('simulationModal');
        const resultsContainer = document.getElementById('simulationResults');
        
        resultsContainer.innerHTML = `
            <div class="simulation-results">
                <div class="sim-metric">
                    <span class="sim-label">Predicted ROI Impact</span>
                    <span class="sim-value positive">+${simulation.roi_impact}%</span>
                </div>
                <div class="sim-metric">
                    <span class="sim-label">Success Probability</span>
                    <span class="sim-value">${simulation.success_probability}%</span>
                </div>
                <div class="sim-metric">
                    <span class="sim-label">Risk Level</span>
                    <span class="sim-value risk-${simulation.risk_level.toLowerCase()}">${simulation.risk_level}</span>
                </div>
                <div class="sim-metric">
                    <span class="sim-label">Estimated Cost</span>
                    <span class="sim-value">${simulation.estimated_cost} sats</span>
                </div>
                
                <div class="sim-timeline">
                    <h4>Execution Timeline</h4>
                    ${simulation.timeline.map(step => `
                        <div class="timeline-step">
                            <span class="step-time">${step.time}</span>
                            <span class="step-action">${step.action}</span>
                            <span class="step-probability">${step.probability}%</span>
                        </div>
                    `).join('')}
                </div>
                
                <div class="sim-actions">
                    <button class="btn-execute-after-sim" onclick="daznoClient.executeAfterSimulation('${simulation.recommendation_id}')">
                        ⚡ Execute Now
                    </button>
                    <button class="btn-cancel-sim" onclick="daznoClient.closeSimulationModal()">
                        Cancel
                    </button>
                </div>
            </div>
        `;
        
        modal.style.display = 'flex';
    }

    closeSimulationModal() {
        document.getElementById('simulationModal').style.display = 'none';
    }

    async executeAfterSimulation(id) {
        this.closeSimulationModal();
        await this.autoExecuteRecommendation(id);
    }

    // Mise à jour temps réel des données
    updateROIDisplay(roiData) {
        const roiValue = document.querySelector('.roi-value');
        const roiPrediction = document.querySelector('.roi-prediction');
        
        if (roiValue) {
            roiValue.textContent = `${roiData.current}%`;
            roiValue.className = `roi-value ${roiData.trend}`;
        }
        
        if (roiPrediction) {
            roiPrediction.textContent = `→ ${roiData.predicted}%`;
        }

        // Mettre à jour le graphique
        if (this.charts.performance) {
            const chart = this.charts.performance;
            const now = moment().format('HH:mm');
            
            chart.data.labels.push(now);
            chart.data.datasets[0].data.push(roiData.current);
            chart.data.datasets[1].data.push(roiData.network_average);
            
            // Garder seulement les 20 derniers points
            if (chart.data.labels.length > 20) {
                chart.data.labels.shift();
                chart.data.datasets[0].data.shift();
                chart.data.datasets[1].data.shift();
            }
            
            chart.update('none');
        }
    }

    updateCompetitiveAnalysis(competitorData) {
        this.competitorData = competitorData;
        
        // Mettre à jour les métriques de comparaison
        const advantages = document.querySelectorAll('.competitive-metric');
        advantages.forEach((metric, index) => {
            if (competitorData[index]) {
                const valueEl = metric.querySelector('.metric-value');
                const comparisonEl = metric.querySelector('.metric-comparison');
                
                if (valueEl) valueEl.textContent = competitorData[index].dazno_value;
                if (comparisonEl) comparisonEl.textContent = `vs ${competitorData[index].competitor_value}`;
            }
        });
    }

    updateAutomationStats(stats) {
        const statsElements = document.querySelectorAll('.auto-value');
        if (statsElements.length >= 3) {
            statsElements[0].textContent = stats.actions_today;
            statsElements[1].textContent = `${stats.success_rate}%`;
            statsElements[2].textContent = `+${stats.roi_gained}%`;
        }
    }

    // Gestion des paramètres d'automatisation
    async updateAutomationMode(mode) {
        try {
            const response = await fetch('/api/automation/mode', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ mode })
            });

            if (response.ok) {
                this.showNotification(`🤖 Automation mode set to ${mode}`, 'success');
                this.automationSettings.mode = mode;
            }
        } catch (error) {
            this.showNotification('Failed to update automation mode', 'error');
        }
    }

    async updateMaxActions(maxActions) {
        try {
            const response = await fetch('/api/automation/max-actions', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ max_actions: parseInt(maxActions) })
            });

            if (response.ok) {
                this.automationSettings.maxActions = maxActions;
            }
        } catch (error) {
            console.error('Failed to update max actions:', error);
        }
    }

    async toggleAutoExecution(enabled) {
        try {
            const response = await fetch('/api/automation/auto-execution', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ enabled })
            });

            if (response.ok) {
                const message = enabled ? 
                    '🤖 Auto-execution enabled - AI will execute safe recommendations automatically' :
                    '⏸️ Auto-execution disabled - Manual approval required';
                    
                this.showNotification(message, enabled ? 'success' : 'warning');
                this.automationSettings.autoExecution = enabled;
            }
        } catch (error) {
            this.showNotification('Failed to toggle auto-execution', 'error');
        }
    }

    async forceDeepAnalysis() {
        this.showNotification('🧠 Initiating deep AI analysis...', 'info');
        
        const button = document.querySelector('.btn-force-analysis');
        button.disabled = true;
        button.textContent = 'Analyzing...';
        
        try {
            const response = await fetch('/api/analysis/force-deep', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' }
            });

            const result = await response.json();
            
            if (result.success) {
                this.showNotification(`🎯 Analysis complete! Found ${result.recommendations_count} new opportunities`, 'success');
                // Recharger les recommandations
                setTimeout(() => window.location.reload(), 2000);
            } else {
                this.showNotification('Analysis completed - no new opportunities found', 'info');
            }
        } catch (error) {
            this.showNotification('Analysis failed', 'error');
        } finally {
            button.disabled = false;
            button.textContent = 'Force Deep Analysis';
        }
    }

    // Système de notifications avancé
    showNotification(message, type = 'info', duration = 5000) {
        const container = document.getElementById('notificationsContainer');
        const notification = document.createElement('div');
        
        const icons = {
            success: '✅',
            error: '❌',
            warning: '⚠️',
            info: 'ℹ️'
        };
        
        notification.className = `notification ${type}`;
        notification.innerHTML = `
            <span class="notification-icon">${icons[type]}</span>
            <span class="notification-message">${message}</span>
            <button class="notification-close">&times;</button>
        `;
        
        container.appendChild(notification);
        
        // Auto-remove
        setTimeout(() => {
            if (notification.parentNode) {
                notification.remove();
            }
        }, duration);
        
        // Manual close
        notification.querySelector('.notification-close').addEventListener('click', () => {
            notification.remove();
        });
    }

    // Démarrage des mises à jour temps réel
    startRealTimeUpdates() {
        // Simuler des mises à jour temps réel pour la démo
        setInterval(() => {
            if (Math.random() > 0.7) { // 30% de chance
                const roiChange = (Math.random() - 0.5) * 2; // -1% à +1%
                const currentROI = 15.5 + roiChange;
                
                this.updateROIDisplay({
                    current: currentROI.toFixed(2),
                    predicted: (currentROI + 2.5).toFixed(2),
                    trend: roiChange > 0 ? 'positive' : 'negative',
                    network_average: 12.3
                });
            }
        }, 10000); // Toutes les 10 secondes
    }

    async loadAutomationSettings() {
        try {
            const response = await fetch('/api/automation/settings');
            this.automationSettings = await response.json();
        } catch (error) {
            console.error('Failed to load automation settings:', error);
        }
    }

    // Utilitaires
    removeRecommendationCard(id) {
        const card = document.querySelector(`[data-id="${id}"]`);
        if (card) {
            card.style.animation = 'slideOut 0.3s ease';
            setTimeout(() => card.remove(), 300);
        }
    }

    addRecommendation(recommendation) {
        // Ajouter dynamiquement une nouvelle recommandation
        const container = document.querySelector('.recommendations-container-pro');
        const card = this.createRecommendationCard(recommendation);
        container.insertBefore(card, container.firstChild);
        
        // Animation d'apparition
        card.style.animation = 'slideIn 0.5s ease';
    }
}

// Initialisation globale
let daznoClient;

document.addEventListener('DOMContentLoaded', () => {
    daznoClient = new SuperiorDaznoClient();
    
    // Ajouter les styles CSS pour les animations
    const style = document.createElement('style');
    style.textContent = `
        @keyframes slideIn {
            from { transform: translateX(-100%); opacity: 0; }
            to { transform: translateX(0); opacity: 1; }
        }
        
        @keyframes slideOut {
            from { transform: translateX(0); opacity: 1; }
            to { transform: translateX(100%); opacity: 0; }
        }
        
        .notification {
            display: flex;
            align-items: center;
            padding: 1rem;
            margin-bottom: 0.5rem;
            border-radius: 8px;
            background: var(--dazno-dark-surface);
            border-left: 4px solid var(--dazno-info);
            box-shadow: var(--shadow-lg);
            animation: slideIn 0.3s ease;
        }
        
        .notification.success { border-left-color: var(--dazno-success); }
        .notification.error { border-left-color: var(--dazno-danger); }
        .notification.warning { border-left-color: var(--dazno-warning); }
        
        .notification-icon {
            margin-right: 0.75rem;
            font-size: 1.25rem;
        }
        
        .notification-message {
            flex: 1;
            color: var(--dazno-light);
            font-weight: 500;
        }
        
        .notification-close {
            background: none;
            border: none;
            color: var(--dazno-text-muted);
            font-size: 1.5rem;
            cursor: pointer;
            padding: 0;
            margin-left: 1rem;
        }
        
        .notification-close:hover {
            color: var(--dazno-light);
        }
    `;
    document.head.appendChild(style);
});

// Export pour usage global
window.SuperiorDaznoClient = SuperiorDaznoClient;