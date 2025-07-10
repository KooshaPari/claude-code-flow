const { contextBridge, ipcRenderer } = require('electron');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // System Information
  getSystemInfo: () => ipcRenderer.invoke('get-system-info'),
  
  // Benchmark Operations
  runBenchmark: (suite, config) => ipcRenderer.invoke('run-benchmark', suite, config),
  stopBenchmark: (suite) => ipcRenderer.invoke('stop-benchmark', suite),
  getBenchmarkStatus: () => ipcRenderer.invoke('get-benchmark-status'),
  getBenchmarkResults: () => ipcRenderer.invoke('get-benchmark-results'),
  
  // Swarm Operations
  getSwarmStatus: () => ipcRenderer.invoke('get-swarm-status'),
  getSwarmMetrics: () => ipcRenderer.invoke('get-swarm-metrics'),
  getAgentList: () => ipcRenderer.invoke('get-agent-list'),
  spawnAgent: (config) => ipcRenderer.invoke('spawn-agent', config),
  terminateAgent: (agentId) => ipcRenderer.invoke('terminate-agent', agentId),
  initializeSwarm: (config) => ipcRenderer.invoke('initialize-swarm', config),
  optimizeSwarm: (config) => ipcRenderer.invoke('optimize-swarm', config),
  updateSwarmConfig: (config) => ipcRenderer.invoke('update-swarm-config', config),
  
  // Neural Operations
  getNeuralStatus: () => ipcRenderer.invoke('get-neural-status'),
  getNeuralModels: () => ipcRenderer.invoke('get-neural-models'),
  getNeuralMetrics: () => ipcRenderer.invoke('get-neural-metrics'),
  getTrainingStatus: () => ipcRenderer.invoke('get-training-status'),
  startNeuralModel: (modelId) => ipcRenderer.invoke('start-neural-model', modelId),
  stopNeuralModel: (modelId) => ipcRenderer.invoke('stop-neural-model', modelId),
  trainNeuralModel: (modelId, config) => ipcRenderer.invoke('train-neural-model', modelId, config),
  trainAllNeuralModels: (config) => ipcRenderer.invoke('train-all-neural-models', config),
  optimizeNeuralModel: (modelId) => ipcRenderer.invoke('optimize-neural-model', modelId),
  runInference: (modelId, config) => ipcRenderer.invoke('run-inference', modelId, config),
  
  // Performance Operations
  getPerformanceMetrics: () => ipcRenderer.invoke('get-performance-metrics'),
  getOptimizationStatus: () => ipcRenderer.invoke('get-optimization-status'),
  toggleOptimization: (optimization, enabled) => ipcRenderer.invoke('toggle-optimization', optimization, enabled),
  runPerformanceOptimization: () => ipcRenderer.invoke('run-performance-optimization'),
  
  // Quality Operations
  getQualityMetrics: () => ipcRenderer.invoke('get-quality-metrics'),
  getVelocityMetrics: () => ipcRenderer.invoke('get-velocity-metrics'),
  getQualityRecommendations: () => ipcRenderer.invoke('get-quality-recommendations'),
  runQualityAnalysis: (config) => ipcRenderer.invoke('run-quality-analysis', config),
  
  // Settings Operations
  getSettings: () => ipcRenderer.invoke('get-settings'),
  saveSettings: (settings) => ipcRenderer.invoke('save-settings', settings),
  getDefaultSettings: () => ipcRenderer.invoke('get-default-settings'),
  exportSettings: (filePath, settings) => ipcRenderer.invoke('export-settings', filePath, settings),
  importSettings: (filePath) => ipcRenderer.invoke('import-settings', filePath),
  
  // File Operations
  showSaveDialog: (options) => ipcRenderer.invoke('show-save-dialog', options),
  showOpenDialog: (options) => ipcRenderer.invoke('show-open-dialog', options),
  importResults: (filePath) => ipcRenderer.invoke('import-results', filePath),
  
  // Event Listeners
  onMenuAction: (callback) => ipcRenderer.on('menu-action', callback),
  onMetricsUpdate: (callback) => ipcRenderer.on('metrics-update', callback),
  onBenchmarkProgress: (callback) => ipcRenderer.on('benchmark-progress', callback),
  onSwarmUpdate: (callback) => ipcRenderer.on('swarm-update', callback),
  
  // Remove Listeners
  removeMenuActionListeners: () => ipcRenderer.removeAllListeners('menu-action'),
  removeMetricsListeners: () => ipcRenderer.removeAllListeners('metrics-update'),
  removeBenchmarkProgressListeners: () => ipcRenderer.removeAllListeners('benchmark-progress'),
  removeSwarmUpdateListeners: () => ipcRenderer.removeAllListeners('swarm-update')
});