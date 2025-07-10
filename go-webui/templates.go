package main

// HTML templates for the web UI

const baseTemplate = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{.Title}} | Claude Flow</title>
    <link rel="stylesheet" href="/static/styles.css">
    <link rel="icon" href="/static/favicon.ico">
</head>
<body class="claude-flow-ui">
    <div id="app">
        <header class="header">
            <div class="header-content">
                <div class="logo">
                    <h1>🌊 Claude Flow</h1>
                    <span class="version">v{{.Version}}</span>
                </div>
                <nav class="main-nav">
                    <button class="nav-button active" data-nav="dashboard">📊 Dashboard</button>
                    <button class="nav-button" data-nav="neural">🧠 Neural</button>
                    <button class="nav-button" data-nav="agents">🤖 Agents</button>
                    <button class="nav-button" data-nav="memory">💾 Memory</button>
                    <button class="nav-button" data-nav="swarm">🐝 Swarm</button>
                    <button class="nav-button" data-nav="github">🐙 GitHub</button>
                    <button class="nav-button" data-nav="performance">📈 Performance</button>
                    <button class="nav-button" data-nav="config">⚙️ Config</button>
                    <button class="nav-button" data-nav="terminal">⌨️ Terminal</button>
                </nav>
                <div class="header-actions">
                    <div class="connection-status" id="connection-status">🔴 Disconnected</div>
                    <button class="action-button" data-action="refresh_data">🔄 Refresh</button>
                </div>
            </div>
        </header>
        
        <main class="main-content" id="main-content">
            {{template "content" .}}
        </main>
        
        <div class="alerts-container" id="alerts-container"></div>
    </div>

    <!-- WebAssembly Support -->
    <script src="/wasm/wasm_exec.js"></script>
    <script>
        // Global WebSocket connection
        let ws;
        let wasmApp;

        // Initialize WebAssembly
        async function initWasm() {
            const go = new Go();
            const result = await WebAssembly.instantiateStreaming(fetch("/wasm/main.wasm"), go.importObject);
            go.run(result.instance);
            
            // Initialize Claude Flow WASM app
            if (window.claudeFlowWasm) {
                wasmApp = window.claudeFlowWasm;
                const initResult = wasmApp.init();
                console.log('🧠 WASM initialized:', initResult);
            }
        }

        // WebSocket connection management
        function claudeFlowConnectWebSocket() {
            const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            const wsUrl = protocol + '//' + window.location.host + '/ws';
            
            ws = new WebSocket(wsUrl);
            
            ws.onopen = function() {
                console.log('🔗 WebSocket connected');
                updateConnectionStatus(true);
            };
            
            ws.onmessage = function(event) {
                try {
                    const message = JSON.parse(event.data);
                    console.log('📨 WebSocket message:', message);
                    
                    // Pass to WASM for processing if available
                    if (wasmApp && wasmApp.processWebSocketMessage) {
                        wasmApp.processWebSocketMessage(event.data);
                    }
                    
                    // Handle specific message types
                    handleWebSocketMessage(message);
                } catch (error) {
                    console.error('Error processing WebSocket message:', error);
                }
            };
            
            ws.onclose = function() {
                console.log('🔌 WebSocket disconnected');
                updateConnectionStatus(false);
                
                // Attempt to reconnect after 5 seconds
                setTimeout(claudeFlowConnectWebSocket, 5000);
            };
            
            ws.onerror = function(error) {
                console.error('🚨 WebSocket error:', error);
                updateConnectionStatus(false);
            };
        }

        function updateConnectionStatus(connected) {
            const statusEl = document.getElementById('connection-status');
            if (statusEl) {
                statusEl.textContent = connected ? '🟢 Connected' : '🔴 Disconnected';
                statusEl.className = 'connection-status ' + (connected ? 'connected' : 'disconnected');
            }
        }

        function handleWebSocketMessage(message) {
            switch (message.type) {
                case 'dashboard_update':
                    updateDashboard(message.data);
                    break;
                case 'alert':
                    showAlert(message.data);
                    break;
                case 'action_response':
                    handleActionResponse(message.data);
                    break;
            }
        }

        function updateDashboard(data) {
            // Update dashboard widgets with new data
            console.log('📊 Updating dashboard:', data);
        }

        function showAlert(alertData) {
            const alertsContainer = document.getElementById('alerts-container');
            if (alertsContainer) {
                const alertEl = document.createElement('div');
                alertEl.className = 'alert alert-' + alertData.level;
                alertEl.innerHTML = 
                    '<div class="alert-content">' +
                    '<span class="alert-message">' + alertData.message + '</span>' +
                    '<button class="alert-close" onclick="this.parentElement.parentElement.remove()">×</button>' +
                    '</div>';
                alertsContainer.appendChild(alertEl);
                
                // Auto-remove after 10 seconds
                setTimeout(() => {
                    if (alertEl.parentElement) {
                        alertEl.remove();
                    }
                }, 10000);
            }
        }

        function handleActionResponse(data) {
            console.log('⚡ Action response:', data);
        }

        // Send WebSocket message
        function sendWebSocketMessage(message) {
            if (ws && ws.readyState === WebSocket.OPEN) {
                ws.send(JSON.stringify(message));
            } else {
                console.warn('WebSocket not connected');
            }
        }

        // Navigation handling
        document.addEventListener('click', function(e) {
            if (e.target.hasAttribute('data-nav')) {
                const view = e.target.getAttribute('data-nav');
                navigateToView(view);
            } else if (e.target.hasAttribute('data-action')) {
                const action = e.target.getAttribute('data-action');
                executeAction(action);
            }
        });

        function navigateToView(view) {
            console.log('🧭 Navigating to:', view);
            
            // Update active navigation
            document.querySelectorAll('.nav-button').forEach(btn => {
                btn.classList.remove('active');
            });
            document.querySelector('[data-nav="' + view + '"]').classList.add('active');
            
            // Render view with WASM if available
            if (wasmApp) {
                switch (view) {
                    case 'dashboard':
                        wasmApp.renderDashboard();
                        break;
                    case 'neural':
                        wasmApp.renderNeuralUI();
                        break;
                    case 'agents':
                        wasmApp.renderAgentUI();
                        break;
                    case 'memory':
                        wasmApp.renderMemoryUI();
                        break;
                    case 'swarm':
                        wasmApp.renderSwarmUI();
                        break;
                    default:
                        loadViewFromServer(view);
                }
            } else {
                loadViewFromServer(view);
            }
        }

        function loadViewFromServer(view) {
            window.location.href = '/' + view;
        }

        function executeAction(action, data = {}) {
            console.log('⚡ Executing action:', action);
            
            // Send action via WebSocket
            sendWebSocketMessage({
                type: 'action',
                action: action,
                data: data,
                timestamp: new Date().toISOString()
            });
            
            // Handle via WASM if available
            if (wasmApp && wasmApp.handleUserAction) {
                wasmApp.handleUserAction(action, JSON.stringify(data));
            }
        }

        // Initialize when page loads
        document.addEventListener('DOMContentLoaded', function() {
            console.log('🚀 Initializing Claude Flow Web UI');
            
            // Initialize WebAssembly
            initWasm().catch(error => {
                console.warn('WASM initialization failed:', error);
            });
            
            // Connect WebSocket
            claudeFlowConnectWebSocket();
            
            // Initialize view based on current path
            const path = window.location.pathname.slice(1) || 'dashboard';
            navigateToView(path);
        });

        // Keep WebSocket alive
        setInterval(() => {
            if (ws && ws.readyState === WebSocket.OPEN) {
                sendWebSocketMessage({
                    type: 'ping',
                    timestamp: new Date().toISOString()
                });
            }
        }, 30000);
    </script>
</body>
</html>`

const dashboardTemplate = `{{define "content"}}
<div class="dashboard-container">
    <div class="dashboard-header">
        <h2>📊 System Dashboard</h2>
        <div class="dashboard-actions">
            <button class="action-button" data-action="refresh_dashboard">🔄 Refresh</button>
        </div>
    </div>
    
    <div class="dashboard-grid" id="dashboard-content">
        <!-- System Status Widget -->
        <div class="widget system-status">
            <div class="widget-header">
                <h3>System Status</h3>
                <div class="status-indicator operational">🟢 Operational</div>
            </div>
            <div class="widget-content">
                <div class="status-item">
                    <span class="label">Uptime:</span>
                    <span class="value" id="system-uptime">2h 45m</span>
                </div>
                <div class="status-item">
                    <span class="label">Version:</span>
                    <span class="value">{{.Version}}</span>
                </div>
                <div class="status-item">
                    <span class="label">Environment:</span>
                    <span class="value">Development</span>
                </div>
            </div>
        </div>
        
        <!-- System Metrics Widget -->
        <div class="widget metrics">
            <div class="widget-header">
                <h3>System Metrics</h3>
            </div>
            <div class="widget-content">
                <div class="metric-item">
                    <div class="metric-label">CPU Usage</div>
                    <div class="metric-bar">
                        <div class="metric-fill" style="width: 65%" id="cpu-usage"></div>
                    </div>
                    <div class="metric-value">65.4%</div>
                </div>
                <div class="metric-item">
                    <div class="metric-label">Memory Usage</div>
                    <div class="metric-bar">
                        <div class="metric-fill" style="width: 78%" id="memory-usage"></div>
                    </div>
                    <div class="metric-value">78.2%</div>
                </div>
                <div class="metric-item">
                    <div class="metric-label">Active Agents</div>
                    <div class="metric-value large" id="active-agents">12</div>
                </div>
                <div class="metric-item">
                    <div class="metric-label">Total Tasks</div>
                    <div class="metric-value large" id="total-tasks">156</div>
                </div>
            </div>
        </div>
        
        <!-- Services Status Widget -->
        <div class="widget services">
            <div class="widget-header">
                <h3>Services Status</h3>
            </div>
            <div class="widget-content">
                <div class="service-item">
                    <div class="service-name">🌊 Claude Flow</div>
                    <div class="service-status healthy">Healthy</div>
                    <div class="service-latency">45ms</div>
                </div>
                <div class="service-item">
                    <div class="service-name">🧠 Neural Engine</div>
                    <div class="service-status healthy">Healthy</div>
                    <div class="service-latency">32ms</div>
                </div>
                <div class="service-item">
                    <div class="service-name">🔧 MCP Server</div>
                    <div class="service-status healthy">Healthy</div>
                    <div class="service-latency">28ms</div>
                </div>
                <div class="service-item">
                    <div class="service-name">💾 Memory Store</div>
                    <div class="service-status warning">Warning</div>
                    <div class="service-latency">95ms</div>
                </div>
            </div>
        </div>
        
        <!-- Recent Activity Widget -->
        <div class="widget activity">
            <div class="widget-header">
                <h3>Recent Activity</h3>
            </div>
            <div class="widget-content">
                <div class="activity-item">
                    <div class="activity-icon">🤖</div>
                    <div class="activity-details">
                        <div class="activity-title">New coordination agent spawned</div>
                        <div class="activity-time">5 minutes ago</div>
                    </div>
                    <div class="activity-status completed">✅</div>
                </div>
                <div class="activity-item">
                    <div class="activity-icon">🧠</div>
                    <div class="activity-details">
                        <div class="activity-title">Neural training started</div>
                        <div class="activity-time">12 minutes ago</div>
                    </div>
                    <div class="activity-status in-progress">🔄</div>
                </div>
                <div class="activity-item">
                    <div class="activity-icon">🐝</div>
                    <div class="activity-details">
                        <div class="activity-title">Swarm topology created</div>
                        <div class="activity-time">18 minutes ago</div>
                    </div>
                    <div class="activity-status completed">✅</div>
                </div>
            </div>
        </div>
        
        <!-- Quick Actions Widget -->
        <div class="widget quick-actions">
            <div class="widget-header">
                <h3>Quick Actions</h3>
            </div>
            <div class="widget-content">
                <button class="quick-action-btn" data-action="agent_create">
                    <div class="action-icon">🤖</div>
                    <div class="action-label">Create Agent</div>
                </button>
                <button class="quick-action-btn" data-action="neural_train">
                    <div class="action-icon">🧠</div>
                    <div class="action-label">Train Model</div>
                </button>
                <button class="quick-action-btn" data-action="swarm_init">
                    <div class="action-icon">🐝</div>
                    <div class="action-label">Init Swarm</div>
                </button>
                <button class="quick-action-btn" data-action="memory_backup">
                    <div class="action-icon">💾</div>
                    <div class="action-label">Backup Memory</div>
                </button>
            </div>
        </div>
        
        <!-- Alerts Widget -->
        <div class="widget alerts">
            <div class="widget-header">
                <h3>System Alerts</h3>
            </div>
            <div class="widget-content" id="dashboard-alerts">
                <div class="alert-item warning">
                    <div class="alert-icon">⚠️</div>
                    <div class="alert-message">High memory usage detected</div>
                    <div class="alert-time">2 minutes ago</div>
                </div>
            </div>
        </div>
    </div>
</div>
{{end}}`

const neuralTemplate = `{{define "content"}}
<div class="neural-container">
    <div class="neural-header">
        <h2>🧠 Neural Network Management</h2>
        <div class="neural-actions">
            <button class="action-button" data-action="neural_refresh">🔄 Refresh</button>
            <button class="action-button primary" data-action="neural_train">🚀 Start Training</button>
        </div>
    </div>
    
    <div class="neural-grid">
        <!-- Models Panel -->
        <div class="widget models-panel">
            <div class="widget-header">
                <h3>Neural Models</h3>
                <span class="model-count">3 models</span>
            </div>
            <div class="widget-content">
                <div class="model-item ready">
                    <div class="model-icon">🎯</div>
                    <div class="model-details">
                        <div class="model-name">coordination-optimizer</div>
                        <div class="model-type">Optimization</div>
                        <div class="model-accuracy">92% accuracy</div>
                    </div>
                    <div class="model-status ready">Ready</div>
                    <div class="model-actions">
                        <button class="btn-sm" data-action="neural_predict" data-model="coordination-optimizer">Predict</button>
                    </div>
                </div>
                
                <div class="model-item training">
                    <div class="model-icon">🎭</div>
                    <div class="model-details">
                        <div class="model-name">behavior-analyzer</div>
                        <div class="model-type">Classification</div>
                        <div class="model-accuracy">89% accuracy</div>
                    </div>
                    <div class="model-status training">Training</div>
                    <div class="model-progress">
                        <div class="progress-bar">
                            <div class="progress-fill" style="width: 67%"></div>
                        </div>
                        <span class="progress-text">67% (Epoch 67/100)</span>
                    </div>
                </div>
                
                <div class="model-item ready">
                    <div class="model-icon">📊</div>
                    <div class="model-details">
                        <div class="model-name">performance-predictor</div>
                        <div class="model-type">Regression</div>
                        <div class="model-accuracy">85% accuracy</div>
                    </div>
                    <div class="model-status ready">Ready</div>
                    <div class="model-actions">
                        <button class="btn-sm" data-action="neural_predict" data-model="performance-predictor">Predict</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Training Panel -->
        <div class="widget training-panel">
            <div class="widget-header">
                <h3>Training Jobs</h3>
            </div>
            <div class="widget-content">
                <div class="training-form">
                    <div class="form-group">
                        <label>Model Type:</label>
                        <select id="model-type">
                            <option value="optimization">Optimization</option>
                            <option value="classification">Classification</option>
                            <option value="regression">Regression</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Pattern:</label>
                        <select id="pattern-type">
                            <option value="coordination">Coordination</option>
                            <option value="behavior">Behavior Analysis</option>
                            <option value="performance">Performance</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Epochs:</label>
                        <input type="number" id="epochs" value="50" min="10" max="1000">
                    </div>
                    <button class="btn primary" onclick="startTraining()">🚀 Start Training</button>
                </div>
                
                <div class="training-jobs">
                    <div class="job-item active">
                        <div class="job-details">
                            <div class="job-name">behavior-analyzer-training</div>
                            <div class="job-progress">67/100 epochs</div>
                        </div>
                        <div class="job-status running">Running</div>
                        <button class="btn-sm danger" data-action="cancel_training">Cancel</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Performance Metrics -->
        <div class="widget performance-metrics">
            <div class="widget-header">
                <h3>Performance Metrics</h3>
            </div>
            <div class="widget-content">
                <div class="metric-grid">
                    <div class="metric-card">
                        <div class="metric-value">1,247</div>
                        <div class="metric-label">Total Predictions</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">88.7%</div>
                        <div class="metric-label">Avg Accuracy</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">3</div>
                        <div class="metric-label">Models Loaded</div>
                    </div>
                    <div class="metric-card">
                        <div class="metric-value">1</div>
                        <div class="metric-label">Training Jobs</div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Predictions History -->
        <div class="widget predictions-history">
            <div class="widget-header">
                <h3>Recent Predictions</h3>
            </div>
            <div class="widget-content">
                <div class="prediction-item">
                    <div class="prediction-model">coordination-optimizer</div>
                    <div class="prediction-confidence">95% confidence</div>
                    <div class="prediction-time">2 minutes ago</div>
                </div>
                <div class="prediction-item">
                    <div class="prediction-model">performance-predictor</div>
                    <div class="prediction-confidence">87% confidence</div>
                    <div class="prediction-time">5 minutes ago</div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function startTraining() {
            const modelType = document.getElementById('model-type').value;
            const patternType = document.getElementById('pattern-type').value;
            const epochs = parseInt(document.getElementById('epochs').value);
            
            executeAction('neural_train', {
                model_type: modelType,
                pattern: patternType,
                epochs: epochs
            });
        }
    </script>
</div>
{{end}}`

const agentsTemplate = `{{define "content"}}
<div class="agents-container">
    <div class="agents-header">
        <h2>🤖 Agent Management</h2>
        <div class="agents-actions">
            <button class="action-button" data-action="agents_refresh">🔄 Refresh</button>
            <button class="action-button primary" data-action="agent_create">➕ Create Agent</button>
        </div>
    </div>
    
    <div class="agents-grid">
        <!-- Active Agents Panel -->
        <div class="widget agents-panel">
            <div class="widget-header">
                <h3>Active Agents</h3>
                <span class="agent-count">3 agents</span>
            </div>
            <div class="widget-content">
                <div class="agent-item active">
                    <div class="agent-avatar">🎯</div>
                    <div class="agent-details">
                        <div class="agent-name">Coordinator</div>
                        <div class="agent-type">coordinator</div>
                        <div class="agent-capabilities">coordination, planning, optimization</div>
                        <div class="agent-stats">
                            <span>Uptime: 2h 30m</span>
                            <span>Tasks: 25</span>
                            <span>Score: 92%</span>
                        </div>
                    </div>
                    <div class="agent-status active">Active</div>
                    <div class="agent-actions">
                        <button class="btn-sm" data-action="agent_pause" data-agent="agent_001">⏸️ Pause</button>
                        <button class="btn-sm" data-action="agent_details" data-agent="agent_001">ℹ️ Details</button>
                    </div>
                </div>
                
                <div class="agent-item active">
                    <div class="agent-avatar">🔍</div>
                    <div class="agent-details">
                        <div class="agent-name">Researcher</div>
                        <div class="agent-type">researcher</div>
                        <div class="agent-capabilities">analysis, research, data processing</div>
                        <div class="agent-stats">
                            <span>Uptime: 1h 45m</span>
                            <span>Tasks: 18</span>
                            <span>Score: 89%</span>
                        </div>
                    </div>
                    <div class="agent-status active">Active</div>
                    <div class="agent-actions">
                        <button class="btn-sm" data-action="agent_pause" data-agent="agent_002">⏸️ Pause</button>
                        <button class="btn-sm" data-action="agent_details" data-agent="agent_002">ℹ️ Details</button>
                    </div>
                </div>
                
                <div class="agent-item idle">
                    <div class="agent-avatar">💻</div>
                    <div class="agent-details">
                        <div class="agent-name">Developer</div>
                        <div class="agent-type">developer</div>
                        <div class="agent-capabilities">coding, testing, debugging</div>
                        <div class="agent-stats">
                            <span>Uptime: 45m</span>
                            <span>Tasks: 12</span>
                            <span>Score: 85%</span>
                        </div>
                    </div>
                    <div class="agent-status idle">Idle</div>
                    <div class="agent-actions">
                        <button class="btn-sm" data-action="agent_activate" data-agent="agent_003">▶️ Activate</button>
                        <button class="btn-sm" data-action="agent_details" data-agent="agent_003">ℹ️ Details</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Agent Creation Panel -->
        <div class="widget creation-panel">
            <div class="widget-header">
                <h3>Create New Agent</h3>
            </div>
            <div class="widget-content">
                <div class="creation-form">
                    <div class="form-group">
                        <label>Agent Type:</label>
                        <select id="agent-type">
                            <option value="coordinator">🎯 Coordinator</option>
                            <option value="researcher">🔍 Researcher</option>
                            <option value="developer">💻 Developer</option>
                            <option value="analyst">📊 Analyst</option>
                            <option value="tester">🧪 Tester</option>
                            <option value="worker">⚒️ Worker</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Agent Name:</label>
                        <input type="text" id="agent-name" placeholder="Enter agent name">
                    </div>
                    <div class="form-group">
                        <label>Capabilities:</label>
                        <div class="capabilities-grid">
                            <label><input type="checkbox" value="coordination"> Coordination</label>
                            <label><input type="checkbox" value="analysis"> Analysis</label>
                            <label><input type="checkbox" value="coding"> Coding</label>
                            <label><input type="checkbox" value="testing"> Testing</label>
                            <label><input type="checkbox" value="research"> Research</label>
                            <label><input type="checkbox" value="optimization"> Optimization</label>
                        </div>
                    </div>
                    <button class="btn primary" onclick="createAgent()">🚀 Create Agent</button>
                </div>
            </div>
        </div>
        
        <!-- Agent Performance -->
        <div class="widget performance-panel">
            <div class="widget-header">
                <h3>Performance Overview</h3>
            </div>
            <div class="widget-content">
                <div class="performance-grid">
                    <div class="perf-card">
                        <div class="perf-value">3</div>
                        <div class="perf-label">Total Agents</div>
                    </div>
                    <div class="perf-card">
                        <div class="perf-value">2</div>
                        <div class="perf-label">Active Agents</div>
                    </div>
                    <div class="perf-card">
                        <div class="perf-value">55</div>
                        <div class="perf-label">Tasks Completed</div>
                    </div>
                    <div class="perf-card">
                        <div class="perf-value">94%</div>
                        <div class="perf-label">Avg Uptime</div>
                    </div>
                </div>
                
                <div class="performance-chart">
                    <h4>Agent Performance Trends</h4>
                    <div class="chart-placeholder">
                        📈 Performance chart would go here
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Task Queue -->
        <div class="widget tasks-panel">
            <div class="widget-header">
                <h3>Task Queue</h3>
            </div>
            <div class="widget-content">
                <div class="task-item priority-high">
                    <div class="task-priority">🔴</div>
                    <div class="task-details">
                        <div class="task-title">Optimize neural network parameters</div>
                        <div class="task-assigned">Assigned to: Coordinator</div>
                    </div>
                    <div class="task-status">In Progress</div>
                </div>
                
                <div class="task-item priority-medium">
                    <div class="task-priority">🟡</div>
                    <div class="task-details">
                        <div class="task-title">Research new coordination algorithms</div>
                        <div class="task-assigned">Assigned to: Researcher</div>
                    </div>
                    <div class="task-status">Pending</div>
                </div>
                
                <div class="task-item priority-low">
                    <div class="task-priority">🟢</div>
                    <div class="task-details">
                        <div class="task-title">Update documentation</div>
                        <div class="task-assigned">Unassigned</div>
                    </div>
                    <div class="task-status">Queued</div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function createAgent() {
            const agentType = document.getElementById('agent-type').value;
            const agentName = document.getElementById('agent-name').value;
            const capabilities = Array.from(document.querySelectorAll('.capabilities-grid input:checked'))
                .map(cb => cb.value);
            
            executeAction('agent_create', {
                type: agentType,
                name: agentName,
                capabilities: capabilities
            });
        }
    </script>
</div>
{{end}}`

const memoryTemplate = `{{define "content"}}
<div class="memory-container">
    <div class="memory-header">
        <h2>💾 Memory Browser</h2>
        <div class="memory-actions">
            <button class="action-button" data-action="memory_refresh">🔄 Refresh</button>
            <button class="action-button" data-action="memory_backup">💾 Backup</button>
            <button class="action-button" data-action="memory_optimize">⚡ Optimize</button>
        </div>
    </div>
    
    <div class="memory-grid">
        <!-- Namespaces Panel -->
        <div class="widget namespaces-panel">
            <div class="widget-header">
                <h3>Memory Namespaces</h3>
            </div>
            <div class="widget-content">
                <div class="namespace-item active" data-namespace="agents">
                    <div class="namespace-icon">🤖</div>
                    <div class="namespace-details">
                        <div class="namespace-name">agents</div>
                        <div class="namespace-stats">45 entries • 2.0 MB</div>
                    </div>
                    <div class="namespace-usage">
                        <div class="usage-bar">
                            <div class="usage-fill" style="width: 60%"></div>
                        </div>
                    </div>
                </div>
                
                <div class="namespace-item" data-namespace="tasks">
                    <div class="namespace-icon">📋</div>
                    <div class="namespace-details">
                        <div class="namespace-name">tasks</div>
                        <div class="namespace-stats">123 entries • 5.1 MB</div>
                    </div>
                    <div class="namespace-usage">
                        <div class="usage-bar">
                            <div class="usage-fill" style="width: 85%"></div>
                        </div>
                    </div>
                </div>
                
                <div class="namespace-item" data-namespace="swarm">
                    <div class="namespace-icon">🐝</div>
                    <div class="namespace-details">
                        <div class="namespace-name">swarm</div>
                        <div class="namespace-stats">28 entries • 1.0 MB</div>
                    </div>
                    <div class="namespace-usage">
                        <div class="usage-bar">
                            <div class="usage-fill" style="width: 40%"></div>
                        </div>
                    </div>
                </div>
                
                <div class="namespace-item" data-namespace="neural">
                    <div class="namespace-icon">🧠</div>
                    <div class="namespace-details">
                        <div class="namespace-name">neural</div>
                        <div class="namespace-stats">67 entries • 3.2 MB</div>
                    </div>
                    <div class="namespace-usage">
                        <div class="usage-bar">
                            <div class="usage-fill" style="width: 70%"></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Search Panel -->
        <div class="widget search-panel">
            <div class="widget-header">
                <h3>Memory Search</h3>
            </div>
            <div class="widget-content">
                <div class="search-form">
                    <div class="search-input-group">
                        <input type="text" id="memory-search" placeholder="Search memory keys and values...">
                        <button class="btn primary" onclick="searchMemory()">🔍 Search</button>
                    </div>
                    <div class="search-filters">
                        <select id="search-namespace">
                            <option value="">All Namespaces</option>
                            <option value="agents">Agents</option>
                            <option value="tasks">Tasks</option>
                            <option value="swarm">Swarm</option>
                            <option value="neural">Neural</option>
                        </select>
                        <select id="search-type">
                            <option value="all">All Types</option>
                            <option value="string">String</option>
                            <option value="object">Object</option>
                            <option value="array">Array</option>
                        </select>
                    </div>
                </div>
                
                <div class="search-results" id="search-results">
                    <div class="result-item">
                        <div class="result-key">agent/coordination/strategy</div>
                        <div class="result-value">hierarchical</div>
                        <div class="result-namespace">agents</div>
                        <div class="result-actions">
                            <button class="btn-sm">📋 Copy</button>
                            <button class="btn-sm">✏️ Edit</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Memory Browser -->
        <div class="widget browser-panel">
            <div class="widget-header">
                <h3>Memory Browser</h3>
                <div class="browser-breadcrumb">
                    <span class="breadcrumb-item active">agents</span>
                    <span class="breadcrumb-separator">></span>
                    <span class="breadcrumb-item">coordination</span>
                </div>
            </div>
            <div class="widget-content">
                <div class="memory-entries">
                    <div class="entry-item">
                        <div class="entry-icon">📝</div>
                        <div class="entry-details">
                            <div class="entry-key">strategy</div>
                            <div class="entry-value">hierarchical</div>
                            <div class="entry-meta">
                                <span>Size: 12 bytes</span>
                                <span>Updated: 30 min ago</span>
                            </div>
                        </div>
                        <div class="entry-actions">
                            <button class="btn-sm">👁️ View</button>
                            <button class="btn-sm">✏️ Edit</button>
                            <button class="btn-sm danger">🗑️ Delete</button>
                        </div>
                    </div>
                    
                    <div class="entry-item">
                        <div class="entry-icon">📊</div>
                        <div class="entry-details">
                            <div class="entry-key">performance_metrics</div>
                            <div class="entry-value">{tasks_completed: 25, accuracy: 0.92}</div>
                            <div class="entry-meta">
                                <span>Size: 156 bytes</span>
                                <span>Updated: 2 min ago</span>
                            </div>
                        </div>
                        <div class="entry-actions">
                            <button class="btn-sm">👁️ View</button>
                            <button class="btn-sm">✏️ Edit</button>
                            <button class="btn-sm danger">🗑️ Delete</button>
                        </div>
                    </div>
                    
                    <div class="entry-item">
                        <div class="entry-icon">⚙️</div>
                        <div class="entry-details">
                            <div class="entry-key">configuration</div>
                            <div class="entry-value">[Object with 8 properties]</div>
                            <div class="entry-meta">
                                <span>Size: 1.2 KB</span>
                                <span>Updated: 1 hour ago</span>
                            </div>
                        </div>
                        <div class="entry-actions">
                            <button class="btn-sm">👁️ View</button>
                            <button class="btn-sm">✏️ Edit</button>
                            <button class="btn-sm danger">🗑️ Delete</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Memory Statistics -->
        <div class="widget stats-panel">
            <div class="widget-header">
                <h3>Memory Statistics</h3>
            </div>
            <div class="widget-content">
                <div class="stats-grid">
                    <div class="stat-card">
                        <div class="stat-value">263</div>
                        <div class="stat-label">Total Entries</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">11.3 MB</div>
                        <div class="stat-label">Total Size</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">4</div>
                        <div class="stat-label">Namespaces</div>
                    </div>
                    <div class="stat-card">
                        <div class="stat-value">89%</div>
                        <div class="stat-label">Cache Hit Rate</div>
                    </div>
                </div>
                
                <div class="memory-usage-chart">
                    <h4>Memory Usage by Namespace</h4>
                    <div class="usage-breakdown">
                        <div class="usage-item">
                            <div class="usage-color" style="background: #4CAF50"></div>
                            <div class="usage-label">tasks</div>
                            <div class="usage-percent">45%</div>
                        </div>
                        <div class="usage-item">
                            <div class="usage-color" style="background: #2196F3"></div>
                            <div class="usage-label">neural</div>
                            <div class="usage-percent">28%</div>
                        </div>
                        <div class="usage-item">
                            <div class="usage-color" style="background: #FF9800"></div>
                            <div class="usage-label">agents</div>
                            <div class="usage-percent">18%</div>
                        </div>
                        <div class="usage-item">
                            <div class="usage-color" style="background: #9C27B0"></div>
                            <div class="usage-label">swarm</div>
                            <div class="usage-percent">9%</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function searchMemory() {
            const query = document.getElementById('memory-search').value;
            const namespace = document.getElementById('search-namespace').value;
            const type = document.getElementById('search-type').value;
            
            executeAction('memory_search', {
                query: query,
                namespace: namespace,
                type: type
            });
        }
        
        // Handle namespace selection
        document.addEventListener('click', function(e) {
            if (e.target.closest('.namespace-item')) {
                const namespace = e.target.closest('.namespace-item').dataset.namespace;
                selectNamespace(namespace);
            }
        });
        
        function selectNamespace(namespace) {
            document.querySelectorAll('.namespace-item').forEach(item => {
                item.classList.remove('active');
            });
            document.querySelector('[data-namespace="' + namespace + '"]').classList.add('active');
            
            // Load namespace entries
            loadNamespaceEntries(namespace);
        }
        
        function loadNamespaceEntries(namespace) {
            console.log('Loading entries for namespace:', namespace);
            // Implementation would fetch and display entries for the selected namespace
        }
    </script>
</div>
{{end}}`

const swarmTemplate = `{{define "content"}}
<div class="swarm-container">
    <div class="swarm-header">
        <h2>🐝 Swarm Visualizer</h2>
        <div class="swarm-actions">
            <button class="action-button" data-action="swarm_refresh">🔄 Refresh</button>
            <button class="action-button primary" data-action="swarm_init">🚀 Initialize Swarm</button>
        </div>
    </div>
    
    <div class="swarm-grid">
        <!-- Topology Visualization -->
        <div class="widget topology-panel">
            <div class="widget-header">
                <h3>Swarm Topology</h3>
                <div class="topology-controls">
                    <select id="topology-type">
                        <option value="hierarchical">Hierarchical</option>
                        <option value="mesh">Mesh</option>
                        <option value="ring">Ring</option>
                        <option value="star">Star</option>
                    </select>
                    <button class="btn-sm" onclick="changeTopology()">Apply</button>
                </div>
            </div>
            <div class="widget-content">
                <div class="swarm-visualization">
                    <svg width="100%" height="400" viewBox="0 0 500 400" class="topology-svg">
                        <!-- Coordinator node -->
                        <circle cx="250" cy="100" r="25" class="node coordinator active" data-node="coordinator">
                            <title>Coordinator Node</title>
                        </circle>
                        <text x="250" y="105" text-anchor="middle" class="node-label">C</text>
                        
                        <!-- Worker nodes -->
                        <circle cx="150" cy="250" r="20" class="node worker active" data-node="worker-1">
                            <title>Worker Node 1</title>
                        </circle>
                        <text x="150" y="255" text-anchor="middle" class="node-label">W1</text>
                        
                        <circle cx="350" cy="250" r="20" class="node worker active" data-node="worker-2">
                            <title>Worker Node 2</title>
                        </circle>
                        <text x="350" y="255" text-anchor="middle" class="node-label">W2</text>
                        
                        <circle cx="100" cy="350" r="20" class="node worker idle" data-node="worker-3">
                            <title>Worker Node 3 (Idle)</title>
                        </circle>
                        <text x="100" y="355" text-anchor="middle" class="node-label">W3</text>
                        
                        <circle cx="400" cy="350" r="20" class="node worker active" data-node="worker-4">
                            <title>Worker Node 4</title>
                        </circle>
                        <text x="400" y="355" text-anchor="middle" class="node-label">W4</text>
                        
                        <!-- Connections -->
                        <line x1="250" y1="125" x2="150" y2="230" class="connection active" data-connection="coord-w1">
                            <title>Coordinator → Worker 1</title>
                        </line>
                        <line x1="250" y1="125" x2="350" y2="230" class="connection active" data-connection="coord-w2">
                            <title>Coordinator → Worker 2</title>
                        </line>
                        <line x1="150" y1="270" x2="100" y2="330" class="connection idle" data-connection="w1-w3">
                            <title>Worker 1 → Worker 3</title>
                        </line>
                        <line x1="350" y1="270" x2="400" y2="330" class="connection active" data-connection="w2-w4">
                            <title>Worker 2 → Worker 4</title>
                        </line>
                        
                        <!-- Message indicators -->
                        <circle cx="200" cy="175" r="3" class="message-indicator">
                            <animate attributeName="opacity" values="1;0;1" dur="2s" repeatCount="indefinite"/>
                        </circle>
                        <circle cx="300" cy="175" r="3" class="message-indicator">
                            <animate attributeName="opacity" values="1;0;1" dur="1.5s" repeatCount="indefinite"/>
                        </circle>
                    </svg>
                </div>
            </div>
        </div>
        
        <!-- Swarm Metrics -->
        <div class="widget metrics-panel">
            <div class="widget-header">
                <h3>Swarm Metrics</h3>
            </div>
            <div class="widget-content">
                <div class="swarm-stats-grid">
                    <div class="stat-item">
                        <div class="stat-icon">🏗️</div>
                        <div class="stat-details">
                            <div class="stat-label">Topology</div>
                            <div class="stat-value">Hierarchical</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">📊</div>
                        <div class="stat-details">
                            <div class="stat-label">Nodes</div>
                            <div class="stat-value">4/5 Active</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">💬</div>
                        <div class="stat-details">
                            <div class="stat-label">Messages/sec</div>
                            <div class="stat-value">127.5</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">⚡</div>
                        <div class="stat-details">
                            <div class="stat-label">Avg Latency</div>
                            <div class="stat-value">12.5ms</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">✅</div>
                        <div class="stat-details">
                            <div class="stat-label">Success Rate</div>
                            <div class="stat-value">98.7%</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">📈</div>
                        <div class="stat-details">
                            <div class="stat-label">Throughput</div>
                            <div class="stat-value">2.8k tasks/hour</div>
                        </div>
                    </div>
                </div>
                
                <div class="performance-trend">
                    <h4>Performance Trend (Last Hour)</h4>
                    <div class="trend-chart">
                        📈 Performance trend chart would go here
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Node Details -->
        <div class="widget node-details-panel">
            <div class="widget-header">
                <h3>Node Details</h3>
                <div class="node-selector">
                    <select id="selected-node">
                        <option value="coordinator">Coordinator</option>
                        <option value="worker-1">Worker 1</option>
                        <option value="worker-2">Worker 2</option>
                        <option value="worker-3">Worker 3</option>
                        <option value="worker-4">Worker 4</option>
                    </select>
                </div>
            </div>
            <div class="widget-content">
                <div class="node-info" id="node-info">
                    <div class="node-status-card">
                        <div class="node-title">
                            <span class="node-icon">🎯</span>
                            <span class="node-name">Coordinator</span>
                            <span class="node-status-badge active">Active</span>
                        </div>
                        
                        <div class="node-metrics">
                            <div class="metric-row">
                                <span class="metric-label">CPU Usage:</span>
                                <span class="metric-value">45.2%</span>
                                <div class="metric-bar">
                                    <div class="metric-fill" style="width: 45%"></div>
                                </div>
                            </div>
                            
                            <div class="metric-row">
                                <span class="metric-label">Memory:</span>
                                <span class="metric-value">67.8%</span>
                                <div class="metric-bar">
                                    <div class="metric-fill" style="width: 68%"></div>
                                </div>
                            </div>
                            
                            <div class="metric-row">
                                <span class="metric-label">Tasks:</span>
                                <span class="metric-value">12 active</span>
                            </div>
                            
                            <div class="metric-row">
                                <span class="metric-label">Uptime:</span>
                                <span class="metric-value">2h 45m</span>
                            </div>
                        </div>
                        
                        <div class="node-connections">
                            <h5>Connections</h5>
                            <div class="connection-list">
                                <div class="connection-item active">
                                    <span class="connection-target">→ Worker 1</span>
                                    <span class="connection-status">Active (8.5ms)</span>
                                </div>
                                <div class="connection-item active">
                                    <span class="connection-target">→ Worker 2</span>
                                    <span class="connection-status">Active (9.2ms)</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Swarm Control Panel -->
        <div class="widget control-panel">
            <div class="widget-header">
                <h3>Swarm Control</h3>
            </div>
            <div class="widget-content">
                <div class="control-section">
                    <h4>Initialize New Swarm</h4>
                    <div class="control-form">
                        <div class="form-group">
                            <label>Topology:</label>
                            <select id="new-topology">
                                <option value="hierarchical">Hierarchical</option>
                                <option value="mesh">Mesh</option>
                                <option value="ring">Ring</option>
                                <option value="star">Star</option>
                            </select>
                        </div>
                        
                        <div class="form-group">
                            <label>Max Agents:</label>
                            <input type="number" id="max-agents" value="10" min="2" max="50">
                        </div>
                        
                        <div class="form-group">
                            <label>Strategy:</label>
                            <select id="swarm-strategy">
                                <option value="balanced">Balanced</option>
                                <option value="performance">Performance</option>
                                <option value="resilient">Resilient</option>
                                <option value="adaptive">Adaptive</option>
                            </select>
                        </div>
                        
                        <button class="btn primary" onclick="initializeSwarm()">🚀 Initialize</button>
                    </div>
                </div>
                
                <div class="control-section">
                    <h4>Swarm Actions</h4>
                    <div class="action-buttons">
                        <button class="btn" data-action="swarm_pause">⏸️ Pause Swarm</button>
                        <button class="btn" data-action="swarm_resume">▶️ Resume Swarm</button>
                        <button class="btn danger" data-action="swarm_stop">⏹️ Stop Swarm</button>
                        <button class="btn" data-action="swarm_optimize">⚡ Optimize</button>
                    </div>
                </div>
                
                <div class="control-section">
                    <h4>Message Log</h4>
                    <div class="message-log">
                        <div class="log-entry">
                            <span class="log-time">14:32:15</span>
                            <span class="log-message">Coordinator → Worker 1: Task assignment</span>
                        </div>
                        <div class="log-entry">
                            <span class="log-time">14:32:12</span>
                            <span class="log-message">Worker 2 → Coordinator: Task completed</span>
                        </div>
                        <div class="log-entry">
                            <span class="log-time">14:32:08</span>
                            <span class="log-message">Coordinator → Worker 4: Status request</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function initializeSwarm() {
            const topology = document.getElementById('new-topology').value;
            const maxAgents = parseInt(document.getElementById('max-agents').value);
            const strategy = document.getElementById('swarm-strategy').value;
            
            executeAction('swarm_init', {
                topology: topology,
                max_agents: maxAgents,
                strategy: strategy
            });
        }
        
        function changeTopology() {
            const newTopology = document.getElementById('topology-type').value;
            executeAction('swarm_change_topology', {
                topology: newTopology
            });
        }
        
        // Handle node selection in visualization
        document.addEventListener('click', function(e) {
            if (e.target.classList.contains('node')) {
                const nodeId = e.target.dataset.node;
                selectNode(nodeId);
            }
        });
        
        function selectNode(nodeId) {
            // Update selected node in dropdown
            document.getElementById('selected-node').value = nodeId;
            
            // Update node details panel
            updateNodeDetails(nodeId);
            
            // Highlight selected node in visualization
            document.querySelectorAll('.node').forEach(node => {
                node.classList.remove('selected');
            });
            document.querySelector('[data-node="' + nodeId + '"]').classList.add('selected');
        }
        
        function updateNodeDetails(nodeId) {
            // In a real implementation, this would fetch and display actual node details
            console.log('Updating details for node:', nodeId);
        }
    </script>
</div>
{{end}}`

const githubTemplate = `{{define "content"}}
<div class="github-container">
    <div class="github-header">
        <h2>🐙 GitHub Integration</h2>
        <div class="github-actions">
            <button class="action-button" data-action="github_refresh">🔄 Refresh</button>
            <button class="action-button primary" data-action="github_sync">🔄 Sync Repos</button>
        </div>
    </div>
    
    <div class="github-grid">
        <!-- Repository List -->
        <div class="widget repos-panel">
            <div class="widget-header">
                <h3>Repositories</h3>
                <div class="repo-stats">2 repos</div>
            </div>
            <div class="widget-content">
                <div class="repo-item active">
                    <div class="repo-details">
                        <div class="repo-name">🌊 claude-flow</div>
                        <div class="repo-description">AI-powered development workflow automation</div>
                        <div class="repo-meta">
                            <span class="repo-language">TypeScript</span>
                            <span class="repo-stars">⭐ 1,247</span>
                            <span class="repo-forks">🍴 89</span>
                        </div>
                    </div>
                    <div class="repo-actions">
                        <button class="btn-sm" data-action="github_analyze" data-repo="claude-flow">📊 Analyze</button>
                        <button class="btn-sm" data-action="github_issues" data-repo="claude-flow">🐛 Issues</button>
                    </div>
                </div>
                
                <div class="repo-item">
                    <div class="repo-details">
                        <div class="repo-name">🐝 ruv-swarm</div>
                        <div class="repo-description">Swarm intelligence framework</div>
                        <div class="repo-meta">
                            <span class="repo-language">JavaScript</span>
                            <span class="repo-stars">⭐ 456</span>
                            <span class="repo-forks">🍴 23</span>
                        </div>
                    </div>
                    <div class="repo-actions">
                        <button class="btn-sm" data-action="github_analyze" data-repo="ruv-swarm">📊 Analyze</button>
                        <button class="btn-sm" data-action="github_issues" data-repo="ruv-swarm">🐛 Issues</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Repository Analysis -->
        <div class="widget analysis-panel">
            <div class="widget-header">
                <h3>Repository Analysis</h3>
                <div class="analysis-controls">
                    <select id="analysis-type">
                        <option value="overview">Overview</option>
                        <option value="code_quality">Code Quality</option>
                        <option value="security">Security</option>
                        <option value="performance">Performance</option>
                    </select>
                    <button class="btn-sm" onclick="runAnalysis()">▶️ Run</button>
                </div>
            </div>
            <div class="widget-content">
                <div class="analysis-results" id="analysis-results">
                    <div class="analysis-score-card">
                        <div class="score-item">
                            <div class="score-label">Code Quality</div>
                            <div class="score-value excellent">87%</div>
                            <div class="score-bar">
                                <div class="score-fill" style="width: 87%"></div>
                            </div>
                        </div>
                        
                        <div class="score-item">
                            <div class="score-label">Test Coverage</div>
                            <div class="score-value good">73%</div>
                            <div class="score-bar">
                                <div class="score-fill" style="width: 73%"></div>
                            </div>
                        </div>
                        
                        <div class="score-item">
                            <div class="score-label">Documentation</div>
                            <div class="score-value excellent">92%</div>
                            <div class="score-bar">
                                <div class="score-fill" style="width: 92%"></div>
                            </div>
                        </div>
                        
                        <div class="score-item">
                            <div class="score-label">Security Score</div>
                            <div class="score-value excellent">89%</div>
                            <div class="score-bar">
                                <div class="score-fill" style="width: 89%"></div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="analysis-recommendations">
                        <h4>Recommendations</h4>
                        <div class="recommendation-item">
                            <div class="rec-icon">📝</div>
                            <div class="rec-text">Increase test coverage in neural modules</div>
                            <div class="rec-priority high">High</div>
                        </div>
                        <div class="recommendation-item">
                            <div class="rec-icon">📚</div>
                            <div class="rec-text">Add more inline documentation</div>
                            <div class="rec-priority medium">Medium</div>
                        </div>
                        <div class="recommendation-item">
                            <div class="rec-icon">🔧</div>
                            <div class="rec-text">Update dependency versions</div>
                            <div class="rec-priority low">Low</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Pull Requests -->
        <div class="widget pr-panel">
            <div class="widget-header">
                <h3>Pull Requests</h3>
                <div class="pr-stats">
                    <span class="pr-count open">3 open</span>
                    <span class="pr-count merged">12 merged</span>
                </div>
            </div>
            <div class="widget-content">
                <div class="pr-item open">
                    <div class="pr-details">
                        <div class="pr-title">🚀 Add neural network optimization</div>
                        <div class="pr-meta">
                            <span class="pr-author">by @developer</span>
                            <span class="pr-time">2 hours ago</span>
                            <span class="pr-checks">✅ 8/8 checks</span>
                        </div>
                    </div>
                    <div class="pr-status open">Open</div>
                    <div class="pr-actions">
                        <button class="btn-sm primary">👁️ Review</button>
                        <button class="btn-sm success">✅ Approve</button>
                    </div>
                </div>
                
                <div class="pr-item open">
                    <div class="pr-details">
                        <div class="pr-title">🐛 Fix memory leak in agent spawning</div>
                        <div class="pr-meta">
                            <span class="pr-author">by @bugfixer</span>
                            <span class="pr-time">6 hours ago</span>
                            <span class="pr-checks">❌ 6/8 checks</span>
                        </div>
                    </div>
                    <div class="pr-status open">Open</div>
                    <div class="pr-actions">
                        <button class="btn-sm">👁️ Review</button>
                        <button class="btn-sm warning">🔧 Needs Work</button>
                    </div>
                </div>
                
                <div class="pr-item open">
                    <div class="pr-details">
                        <div class="pr-title">📚 Update documentation for v2.0</div>
                        <div class="pr-meta">
                            <span class="pr-author">by @documenter</span>
                            <span class="pr-time">1 day ago</span>
                            <span class="pr-checks">✅ 8/8 checks</span>
                        </div>
                    </div>
                    <div class="pr-status open">Open</div>
                    <div class="pr-actions">
                        <button class="btn-sm primary">👁️ Review</button>
                        <button class="btn-sm success">✅ Approve</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Issues Tracking -->
        <div class="widget issues-panel">
            <div class="widget-header">
                <h3>Issues</h3>
                <div class="issue-stats">
                    <span class="issue-count open">7 open</span>
                    <span class="issue-count closed">45 closed</span>
                </div>
            </div>
            <div class="widget-content">
                <div class="issue-item high">
                    <div class="issue-priority">🔴</div>
                    <div class="issue-details">
                        <div class="issue-title">Memory usage spikes during neural training</div>
                        <div class="issue-meta">
                            <span class="issue-labels">
                                <span class="label bug">bug</span>
                                <span class="label performance">performance</span>
                            </span>
                            <span class="issue-assignee">@optimizer</span>
                        </div>
                    </div>
                    <div class="issue-time">3 hours ago</div>
                </div>
                
                <div class="issue-item medium">
                    <div class="issue-priority">🟡</div>
                    <div class="issue-details">
                        <div class="issue-title">Add support for custom swarm topologies</div>
                        <div class="issue-meta">
                            <span class="issue-labels">
                                <span class="label enhancement">enhancement</span>
                                <span class="label swarm">swarm</span>
                            </span>
                            <span class="issue-assignee">@architect</span>
                        </div>
                    </div>
                    <div class="issue-time">1 day ago</div>
                </div>
                
                <div class="issue-item low">
                    <div class="issue-priority">🟢</div>
                    <div class="issue-details">
                        <div class="issue-title">Improve error messages in CLI</div>
                        <div class="issue-meta">
                            <span class="issue-labels">
                                <span class="label improvement">improvement</span>
                                <span class="label cli">cli</span>
                            </span>
                            <span class="issue-assignee">Unassigned</span>
                        </div>
                    </div>
                    <div class="issue-time">3 days ago</div>
                </div>
            </div>
        </div>
        
        <!-- Workflow Automation -->
        <div class="widget workflow-panel">
            <div class="widget-header">
                <h3>Workflow Automation</h3>
            </div>
            <div class="widget-content">
                <div class="workflow-item active">
                    <div class="workflow-name">🚀 CI/CD Pipeline</div>
                    <div class="workflow-status success">✅ Passing</div>
                    <div class="workflow-details">
                        <div class="workflow-runs">Last run: 2 hours ago</div>
                        <div class="workflow-duration">Duration: 4m 23s</div>
                    </div>
                </div>
                
                <div class="workflow-item active">
                    <div class="workflow-name">🧪 Automated Testing</div>
                    <div class="workflow-status success">✅ Passing</div>
                    <div class="workflow-details">
                        <div class="workflow-runs">Last run: 30 minutes ago</div>
                        <div class="workflow-duration">Duration: 2m 15s</div>
                    </div>
                </div>
                
                <div class="workflow-item error">
                    <div class="workflow-name">📊 Performance Benchmarks</div>
                    <div class="workflow-status error">❌ Failed</div>
                    <div class="workflow-details">
                        <div class="workflow-runs">Last run: 6 hours ago</div>
                        <div class="workflow-duration">Duration: 1m 45s</div>
                    </div>
                </div>
                
                <div class="workflow-actions">
                    <button class="btn primary" data-action="github_run_workflow">▶️ Run Workflow</button>
                    <button class="btn" data-action="github_view_logs">📋 View Logs</button>
                </div>
            </div>
        </div>
        
        <!-- Release Management -->
        <div class="widget release-panel">
            <div class="widget-header">
                <h3>Release Management</h3>
            </div>
            <div class="widget-content">
                <div class="release-item latest">
                    <div class="release-tag">v2.0.0</div>
                    <div class="release-details">
                        <div class="release-title">Claude Flow 2.0 - Major Release</div>
                        <div class="release-date">Released 2 weeks ago</div>
                        <div class="release-stats">
                            <span>📥 1,247 downloads</span>
                            <span>🏷️ Latest</span>
                        </div>
                    </div>
                </div>
                
                <div class="release-actions">
                    <button class="btn primary" data-action="github_create_release">🏷️ Create Release</button>
                    <button class="btn" data-action="github_draft_release">📝 Draft Release</button>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        function runAnalysis() {
            const analysisType = document.getElementById('analysis-type').value;
            const activeRepo = document.querySelector('.repo-item.active .repo-name').textContent.replace('🌊 ', '').replace('🐝 ', '');
            
            executeAction('github_analyze', {
                repository: activeRepo,
                analysis: analysisType
            });
        }
    </script>
</div>
{{end}}`

const performanceTemplate = `{{define "content"}}
<div class="performance-container">
    <div class="performance-header">
        <h2>📈 Performance Monitor</h2>
        <div class="performance-actions">
            <button class="action-button" data-action="performance_refresh">🔄 Refresh</button>
            <button class="action-button" data-action="performance_export">📊 Export</button>
        </div>
    </div>
    
    <div class="performance-grid">
        <!-- Real-time Metrics -->
        <div class="widget metrics-panel">
            <div class="widget-header">
                <h3>System Performance</h3>
            </div>
            <div class="widget-content">
                <div class="perf-metric-grid">
                    <div class="perf-metric">
                        <div class="metric-icon">💻</div>
                        <div class="metric-details">
                            <div class="metric-label">CPU Usage</div>
                            <div class="metric-value">65.4%</div>
                            <div class="metric-trend up">+2.1%</div>
                        </div>
                        <div class="metric-chart">
                            <div class="mini-chart">📈</div>
                        </div>
                    </div>
                    
                    <div class="perf-metric">
                        <div class="metric-icon">💾</div>
                        <div class="metric-details">
                            <div class="metric-label">Memory Usage</div>
                            <div class="metric-value">78.2%</div>
                            <div class="metric-trend down">-1.5%</div>
                        </div>
                        <div class="metric-chart">
                            <div class="mini-chart">📈</div>
                        </div>
                    </div>
                    
                    <div class="perf-metric">
                        <div class="metric-icon">💿</div>
                        <div class="metric-details">
                            <div class="metric-label">Disk I/O</div>
                            <div class="metric-value">23.7 MB/s</div>
                            <div class="metric-trend stable">±0.2%</div>
                        </div>
                        <div class="metric-chart">
                            <div class="mini-chart">📈</div>
                        </div>
                    </div>
                    
                    <div class="perf-metric">
                        <div class="metric-icon">🌐</div>
                        <div class="metric-details">
                            <div class="metric-label">Network</div>
                            <div class="metric-value">145.2 KB/s</div>
                            <div class="metric-trend up">+5.7%</div>
                        </div>
                        <div class="metric-chart">
                            <div class="mini-chart">📈</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Performance Alerts -->
        <div class="widget alerts-panel">
            <div class="widget-header">
                <h3>Performance Alerts</h3>
                <div class="alert-summary">2 active</div>
            </div>
            <div class="widget-content">
                <div class="alert-item warning">
                    <div class="alert-severity">⚠️</div>
                    <div class="alert-details">
                        <div class="alert-title">High Memory Usage Detected</div>
                        <div class="alert-description">Memory usage has exceeded 75% threshold</div>
                        <div class="alert-time">2 minutes ago</div>
                    </div>
                    <div class="alert-actions">
                        <button class="btn-sm" data-action="alert_investigate">🔍 Investigate</button>
                        <button class="btn-sm" data-action="alert_dismiss">✕ Dismiss</button>
                    </div>
                </div>
                
                <div class="alert-item critical">
                    <div class="alert-severity">🚨</div>
                    <div class="alert-details">
                        <div class="alert-title">Agent Response Time Degraded</div>
                        <div class="alert-description">Average response time increased by 300%</div>
                        <div class="alert-time">8 minutes ago</div>
                    </div>
                    <div class="alert-actions">
                        <button class="btn-sm" data-action="alert_investigate">🔍 Investigate</button>
                        <button class="btn-sm" data-action="alert_dismiss">✕ Dismiss</button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
{{end}}`

const configTemplate = `{{define "content"}}
<div class="config-container">
    <div class="config-header">
        <h2>⚙️ Configuration Manager</h2>
        <div class="config-actions">
            <button class="action-button" data-action="config_reload">🔄 Reload</button>
            <button class="action-button primary" data-action="config_save">💾 Save</button>
        </div>
    </div>
    
    <div class="config-grid">
        <!-- Server Configuration -->
        <div class="widget server-config">
            <div class="widget-header">
                <h3>Server Configuration</h3>
            </div>
            <div class="widget-content">
                <div class="config-form">
                    <div class="form-group">
                        <label>Port:</label>
                        <input type="number" id="server-port" value="8080" min="1000" max="65535">
                    </div>
                    <div class="form-group">
                        <label>Host:</label>
                        <input type="text" id="server-host" value="localhost">
                    </div>
                    <div class="form-group">
                        <label>Environment:</label>
                        <select id="server-env">
                            <option value="development">Development</option>
                            <option value="staging">Staging</option>
                            <option value="production">Production</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Log Level:</label>
                        <select id="log-level">
                            <option value="debug">Debug</option>
                            <option value="info">Info</option>
                            <option value="warn">Warning</option>
                            <option value="error">Error</option>
                        </select>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Service Endpoints -->
        <div class="widget services-config">
            <div class="widget-header">
                <h3>Service Endpoints</h3>
            </div>
            <div class="widget-content">
                <div class="config-form">
                    <div class="form-group">
                        <label>Claude Flow API:</label>
                        <input type="url" id="claude-flow-endpoint" value="http://localhost:3000">
                    </div>
                    <div class="form-group">
                        <label>Neural Engine:</label>
                        <input type="url" id="neural-endpoint" value="http://localhost:8081">
                    </div>
                    <div class="form-group">
                        <label>MCP Server:</label>
                        <input type="url" id="mcp-endpoint" value="http://localhost:8082">
                    </div>
                    <div class="form-group">
                        <label>Memory Store:</label>
                        <input type="url" id="memory-endpoint" value="http://localhost:8083">
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
{{end}}`

const terminalTemplate = `{{define "content"}}
<div class="terminal-container">
    <div class="terminal-header">
        <h2>⌨️ Terminal Emulator</h2>
        <div class="terminal-actions">
            <button class="action-button" data-action="terminal_clear">🗑️ Clear</button>
            <button class="action-button" data-action="terminal_fullscreen">⛶ Fullscreen</button>
        </div>
    </div>
    
    <div class="terminal-wrapper">
        <div class="terminal" id="terminal">
            <div class="terminal-output" id="terminal-output">
                <div class="terminal-line">
                    <span class="terminal-prompt">claude-flow@localhost:~$</span>
                    <span class="terminal-text">Welcome to Claude Flow Terminal</span>
                </div>
                <div class="terminal-line">
                    <span class="terminal-prompt">claude-flow@localhost:~$</span>
                    <span class="terminal-text">Type 'help' for available commands</span>
                </div>
            </div>
            <div class="terminal-input-line">
                <span class="terminal-prompt">claude-flow@localhost:~$</span>
                <input type="text" class="terminal-input" id="terminal-input" autocomplete="off" autocorrect="off" autocapitalize="off" spellcheck="false">
            </div>
        </div>
    </div>
    
    <script>
        const terminalInput = document.getElementById('terminal-input');
        const terminalOutput = document.getElementById('terminal-output');
        
        terminalInput.addEventListener('keydown', function(e) {
            if (e.key === 'Enter') {
                const command = e.target.value.trim();
                if (command) {
                    executeTerminalCommand(command);
                    e.target.value = '';
                }
            }
        });
        
        function executeTerminalCommand(command) {
            // Add command to output
            addTerminalLine('claude-flow@localhost:~$', command, 'command');
            
            // Process command
            processCommand(command);
        }
        
        function processCommand(command) {
            const parts = command.split(' ');
            const cmd = parts[0];
            
            switch (cmd) {
                case 'help':
                    showHelp();
                    break;
                case 'status':
                    showStatus();
                    break;
                case 'agents':
                    showAgents();
                    break;
                case 'clear':
                    clearTerminal();
                    break;
                default:
                    addTerminalLine('', 'Command not found: ' + cmd, 'error');
            }
        }
        
        function addTerminalLine(prompt, text, type = 'output') {
            const line = document.createElement('div');
            line.className = 'terminal-line ' + type;
            
            if (prompt) {
                const promptSpan = document.createElement('span');
                promptSpan.className = 'terminal-prompt';
                promptSpan.textContent = prompt;
                line.appendChild(promptSpan);
            }
            
            const textSpan = document.createElement('span');
            textSpan.className = 'terminal-text';
            textSpan.textContent = text;
            line.appendChild(textSpan);
            
            terminalOutput.appendChild(line);
            terminalOutput.scrollTop = terminalOutput.scrollHeight;
        }
        
        function showHelp() {
            const helpText = [
                'Available commands:',
                '  help     - Show this help message',
                '  status   - Show system status',
                '  agents   - List active agents',
                '  clear    - Clear terminal',
            ];
            
            helpText.forEach(line => addTerminalLine('', line));
        }
        
        function showStatus() {
            addTerminalLine('', 'System Status: Operational');
            addTerminalLine('', 'Uptime: 2h 45m');
            addTerminalLine('', 'Active Agents: 12');
            addTerminalLine('', 'Memory Usage: 78.2%');
        }
        
        function showAgents() {
            addTerminalLine('', 'Active Agents:');
            addTerminalLine('', '  agent-001: Coordinator (active)');
            addTerminalLine('', '  agent-002: Researcher (active)');
            addTerminalLine('', '  agent-003: Developer (idle)');
        }
        
        function clearTerminal() {
            terminalOutput.innerHTML = '';
        }
        
        // Focus terminal input when terminal is clicked
        document.getElementById('terminal').addEventListener('click', function() {
            terminalInput.focus();
        });
        
        // Auto-focus terminal input
        terminalInput.focus();
    </script>
</div>
{{end}}`