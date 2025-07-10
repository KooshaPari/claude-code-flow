const { contextBridge, ipcRenderer } = require('electron');

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // App info
  getAppVersion: () => ipcRenderer.invoke('get-app-version'),
  
  // Storage
  getStoredData: (key) => ipcRenderer.invoke('get-stored-data', key),
  setStoredData: (key, value) => ipcRenderer.invoke('set-stored-data', key, value),
  
  // Dialogs
  showSaveDialog: (options) => ipcRenderer.invoke('show-save-dialog', options),
  showOpenDialog: (options) => ipcRenderer.invoke('show-open-dialog', options),
  showMessageBox: (options) => ipcRenderer.invoke('show-message-box', options),
  
  // Menu actions
  onMenuAction: (callback) => ipcRenderer.on('menu-action', callback),
  removeMenuActionListeners: () => ipcRenderer.removeAllListeners('menu-action'),
  
  // Benchmark operations
  runBenchmark: (type, config) => ipcRenderer.invoke('run-benchmark', type, config),
  stopBenchmark: (id) => ipcRenderer.invoke('stop-benchmark', id),
  getBenchmarkStatus: (id) => ipcRenderer.invoke('get-benchmark-status', id),
  
  // Swarm operations
  initializeSwarm: (config) => ipcRenderer.invoke('initialize-swarm', config),
  getSwarmStatus: () => ipcRenderer.invoke('get-swarm-status'),
  optimizeSwarm: () => ipcRenderer.invoke('optimize-swarm'),
  
  // Performance monitoring
  getPerformanceMetrics: () => ipcRenderer.invoke('get-performance-metrics'),
  onMetricsUpdate: (callback) => ipcRenderer.on('metrics-update', callback),
  removeMetricsListeners: () => ipcRenderer.removeAllListeners('metrics-update'),
  
  // File operations
  exportResults: (data, filename) => ipcRenderer.invoke('export-results', data, filename),
  importResults: (filepath) => ipcRenderer.invoke('import-results', filepath),
  
  // Neural operations
  getNeuralStatus: () => ipcRenderer.invoke('get-neural-status'),
  trainNeuralPatterns: (config) => ipcRenderer.invoke('train-neural-patterns', config),
  
  // System operations
  getSystemInfo: () => ipcRenderer.invoke('get-system-info'),
  checkForUpdates: () => ipcRenderer.invoke('check-for-updates'),
  
  // Real-time communication
  onBenchmarkProgress: (callback) => ipcRenderer.on('benchmark-progress', callback),
  removeBenchmarkProgressListeners: () => ipcRenderer.removeAllListeners('benchmark-progress'),
  
  onSwarmUpdate: (callback) => ipcRenderer.on('swarm-update', callback),
  removeSwarmUpdateListeners: () => ipcRenderer.removeAllListeners('swarm-update'),
  
  // Notifications
  showNotification: (title, body, options) => ipcRenderer.invoke('show-notification', title, body, options),
  
  // Advanced features
  runMCPTool: (toolName, params) => ipcRenderer.invoke('run-mcp-tool', toolName, params),
  getAvailableMCPTools: () => ipcRenderer.invoke('get-available-mcp-tools'),
  
  // Performance optimization
  runPerformanceOptimization: () => ipcRenderer.invoke('run-performance-optimization'),
  getOptimizationSuggestions: () => ipcRenderer.invoke('get-optimization-suggestions'),
  
  // Quality metrics
  getQualityMetrics: () => ipcRenderer.invoke('get-quality-metrics'),
  generateQualityReport: (config) => ipcRenderer.invoke('generate-quality-report', config)
});