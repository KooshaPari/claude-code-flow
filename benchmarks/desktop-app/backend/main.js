const { app, BrowserWindow, Menu, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs').promises;
const { spawn } = require('child_process');

// Keep a global reference of the window object
let mainWindow;

// Mock data for development - in production, these would connect to actual services
let mockData = {
  systemInfo: {
    platform: process.platform,
    arch: process.arch,
    version: process.version,
    memory: process.memoryUsage()
  },
  
  benchmarkStatus: {
    running: {},
    results: {
      sweBench: { score: 86.2, status: 'completed', timestamp: Date.now() },
      humanEval: { score: 93.1, status: 'completed', timestamp: Date.now() },
      bigCode: { score: 88.7, status: 'completed', timestamp: Date.now() }
    }
  },
  
  swarmStatus: {
    status: 'active',
    activeAgents: 12,
    coordinationEfficiency: 94.2,
    config: {
      topology: 'hierarchical',
      coordination: 'queen',
      maxAgents: 15,
      neuralEnabled: true
    }
  },
  
  neuralStatus: {
    active: true,
    modelsLoaded: 27,
    avgAccuracy: 89.3,
    totalInferences: 45672
  },
  
  performanceMetrics: {
    overallScore: 89.3,
    cpu: 45,
    memory: 192,
    latency: 142,
    throughput: 1250
  },
  
  qualityMetrics: {
    overallQualityIndex: 88.7,
    codeQuality: 92.3,
    testCoverage: 89.5,
    documentation: 85.2,
    security: 91.8,
    performance: 86.4
  },
  
  settings: {
    maxParallelBenchmarks: 5,
    enableNeuralOptimization: true,
    enableSwarmCoordination: true,
    darkMode: true,
    autoRefreshInterval: 3000,
    defaultTopology: 'hierarchical',
    defaultCoordination: 'queen',
    maxAgents: 15
  }
};

function createWindow() {
  // Create the browser window with Claude-desktop styling
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1200,
    minHeight: 800,
    icon: path.join(__dirname, '../assets/icon.png'),
    titleBarStyle: 'hiddenInset', // macOS style
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      enableRemoteModule: false,
      preload: path.join(__dirname, 'electronAPI.js')
    },
    show: false, // Don't show until ready
    backgroundColor: '#0a0a0a', // Match dark theme
    vibrancy: 'dark', // macOS vibrancy effect
    frame: true,
    titleBarOverlay: {
      color: '#0a0a0a',
      symbolColor: '#ffffff'
    }
  });

  // Load the app
  const isDev = process.env.NODE_ENV === 'development';
  if (isDev) {
    mainWindow.loadURL('http://localhost:3000');
    mainWindow.webContents.openDevTools();
  } else {
    mainWindow.loadFile(path.join(__dirname, '../build/index.html'));
  }

  // Show window when ready to prevent visual flash
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
    
    // Focus the window on macOS
    if (process.platform === 'darwin') {
      mainWindow.focus();
    }
  });

  // Emitted when the window is closed
  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // Handle window events
  mainWindow.on('maximize', () => {
    mainWindow.webContents.send('window-maximized');
  });

  mainWindow.on('unmaximize', () => {
    mainWindow.webContents.send('window-unmaximized');
  });
}

// Application menu
function createMenu() {
  const template = [
    {
      label: 'File',
      submenu: [
        {
          label: 'New Session',
          accelerator: 'CmdOrCtrl+N',
          click: () => {
            mainWindow.webContents.send('menu-action', 'new-session');
          }
        },
        { type: 'separator' },
        {
          label: 'Export Results',
          accelerator: 'CmdOrCtrl+E',
          click: () => {
            mainWindow.webContents.send('menu-action', 'export-results');
          }
        },
        {
          label: 'Import Results',
          accelerator: 'CmdOrCtrl+I',
          click: async () => {
            const result = await dialog.showOpenDialog(mainWindow, {
              title: 'Import Results',
              filters: [
                { name: 'JSON Files', extensions: ['json'] },
                { name: 'All Files', extensions: ['*'] }
              ],
              properties: ['openFile']
            });

            if (!result.canceled) {
              mainWindow.webContents.send('menu-action', 'open-results', result.filePaths[0]);
            }
          }
        },
        { type: 'separator' },
        process.platform === 'darwin' ? { role: 'close' } : { role: 'quit' }
      ]
    },
    {
      label: 'Benchmarks',
      submenu: [
        {
          label: 'Run All Benchmarks',
          accelerator: 'CmdOrCtrl+R',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-all-benchmarks');
          }
        },
        { type: 'separator' },
        {
          label: 'SWE-Bench Suite',
          accelerator: 'CmdOrCtrl+1',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-swe-bench');
          }
        },
        {
          label: 'HumanEval Suite',
          accelerator: 'CmdOrCtrl+2',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-humaneval');
          }
        },
        {
          label: 'BigCode Suite',
          accelerator: 'CmdOrCtrl+3',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-bigcode');
          }
        }
      ]
    },
    {
      label: 'Swarm',
      submenu: [
        {
          label: 'Initialize Swarm',
          accelerator: 'CmdOrCtrl+Shift+S',
          click: () => {
            mainWindow.webContents.send('menu-action', 'initialize-swarm');
          }
        },
        {
          label: 'Optimize Performance',
          accelerator: 'CmdOrCtrl+Shift+O',
          click: () => {
            mainWindow.webContents.send('menu-action', 'optimize-performance');
          }
        }
      ]
    },
    {
      label: 'View',
      submenu: [
        {
          label: 'Dashboard',
          accelerator: 'CmdOrCtrl+D',
          click: () => {
            mainWindow.webContents.send('menu-action', 'show-dashboard');
          }
        },
        {
          label: 'Performance Monitor',
          accelerator: 'CmdOrCtrl+P',
          click: () => {
            mainWindow.webContents.send('menu-action', 'show-performance');
          }
        },
        {
          label: 'Swarm Status',
          accelerator: 'CmdOrCtrl+S',
          click: () => {
            mainWindow.webContents.send('menu-action', 'show-swarm-status');
          }
        },
        { type: 'separator' },
        { role: 'reload' },
        { role: 'forceReload' },
        { role: 'toggleDevTools' },
        { type: 'separator' },
        { role: 'resetZoom' },
        { role: 'zoomIn' },
        { role: 'zoomOut' },
        { type: 'separator' },
        { role: 'togglefullscreen' }
      ]
    },
    {
      label: 'Window',
      submenu: [
        { role: 'minimize' },
        { role: 'close' }
      ]
    },
    {
      role: 'help',
      submenu: [
        {
          label: 'About Claude Flow',
          click: () => {
            dialog.showMessageBox(mainWindow, {
              type: 'info',
              title: 'About Claude Flow Benchmark Suite',
              message: 'Claude Flow Benchmark Suite v2.0',
              detail: 'Comprehensive AI benchmarking with SWE-Bench, HumanEval, and BigCode suites.\nBuilt with Claude Code and inspired by Ruv\'s 84.8% performance achievements.'
            });
          }
        }
      ]
    }
  ];

  if (process.platform === 'darwin') {
    template.unshift({
      label: app.getName(),
      submenu: [
        { role: 'about' },
        { type: 'separator' },
        { role: 'services' },
        { type: 'separator' },
        { role: 'hide' },
        { role: 'hideOthers' },
        { role: 'unhide' },
        { type: 'separator' },
        { role: 'quit' }
      ]
    });

    // Window menu
    template[5].submenu = [
      { role: 'close' },
      { role: 'minimize' },
      { role: 'zoom' },
      { type: 'separator' },
      { role: 'front' }
    ];
  }

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
}

// IPC Handlers
function setupIpcHandlers() {
  // System Information
  ipcMain.handle('get-system-info', async () => {
    return mockData.systemInfo;
  });

  // Benchmark Operations
  ipcMain.handle('run-benchmark', async (event, suite, config) => {
    console.log(`Running benchmark: ${suite}`, config);
    
    // Simulate running a benchmark by spawning a Python process
    try {
      const benchmarkScript = path.join(__dirname, `../../${suite}/run_${suite}_complete.py`);
      
      // Start benchmark in background
      mockData.benchmarkStatus.running[suite] = {
        status: 'running',
        progress: 0,
        startTime: Date.now()
      };
      
      // Simulate progress updates
      let progress = 0;
      const progressInterval = setInterval(() => {
        progress += Math.random() * 10;
        if (progress >= 100) {
          progress = 100;
          clearInterval(progressInterval);
          
          // Mark as completed
          delete mockData.benchmarkStatus.running[suite];
          mockData.benchmarkStatus.results[suite] = {
            score: 80 + Math.random() * 15, // Random score between 80-95
            status: 'completed',
            timestamp: Date.now()
          };
          
          mainWindow.webContents.send('benchmark-progress', {
            suite,
            status: 'completed',
            progress: 100
          });
        } else {
          mockData.benchmarkStatus.running[suite].progress = progress;
          mainWindow.webContents.send('benchmark-progress', {
            suite,
            status: 'running',
            progress
          });
        }
      }, 2000);
      
      return { success: true, message: `${suite} benchmark started` };
    } catch (error) {
      console.error('Benchmark failed:', error);
      return { success: false, error: error.message };
    }
  });

  ipcMain.handle('stop-benchmark', async (event, suite) => {
    if (mockData.benchmarkStatus.running[suite]) {
      delete mockData.benchmarkStatus.running[suite];
      return { success: true };
    }
    return { success: false, error: 'Benchmark not running' };
  });

  ipcMain.handle('get-benchmark-status', async () => {
    return mockData.benchmarkStatus;
  });

  ipcMain.handle('get-benchmark-results', async () => {
    return mockData.benchmarkStatus.results;
  });

  // Swarm Operations
  ipcMain.handle('get-swarm-status', async () => {
    return mockData.swarmStatus;
  });

  ipcMain.handle('get-swarm-metrics', async () => {
    return {
      activeAgents: 12,
      coordinationEfficiency: 94.2 + Math.random() * 4 - 2,
      totalTasks: 847,
      completedTasks: 678,
      avgResponseTime: 142 + Math.random() * 20 - 10
    };
  });

  ipcMain.handle('get-agent-list', async () => {
    return [
      { id: 'agent_001', type: 'coordinator', name: 'Queen Agent', status: 'active', tasks: 156, accuracy: 97.2, uptime: '2h 34m' },
      { id: 'agent_002', type: 'researcher', name: 'Research Alpha', status: 'active', tasks: 89, accuracy: 91.8, uptime: '2h 31m' },
      { id: 'agent_003', type: 'coder', name: 'Code Ninja', status: 'active', tasks: 124, accuracy: 89.4, uptime: '2h 29m' },
      { id: 'agent_004', type: 'analyst', name: 'Data Wizard', status: 'active', tasks: 67, accuracy: 93.1, uptime: '2h 27m' },
      { id: 'agent_005', type: 'tester', name: 'QA Master', status: 'active', tasks: 78, accuracy: 95.6, uptime: '2h 25m' }
    ];
  });

  ipcMain.handle('initialize-swarm', async (event, config) => {
    console.log('Initializing swarm with config:', config);
    mockData.swarmStatus.status = 'active';
    mockData.swarmStatus.config = { ...mockData.swarmStatus.config, ...config };
    
    mainWindow.webContents.send('swarm-update', mockData.swarmStatus);
    return { success: true };
  });

  ipcMain.handle('optimize-swarm', async (event, config) => {
    console.log('Optimizing swarm:', config);
    
    // Simulate optimization time
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    mockData.swarmStatus.coordinationEfficiency = Math.min(98, mockData.swarmStatus.coordinationEfficiency + 5);
    
    return { 
      success: true, 
      improvement: '12%',
      newEfficiency: mockData.swarmStatus.coordinationEfficiency 
    };
  });

  // Neural Operations
  ipcMain.handle('get-neural-status', async () => {
    return mockData.neuralStatus;
  });

  ipcMain.handle('get-neural-models', async () => {
    return [
      { id: 'model_001', name: 'Coordination Master', type: 'coordination', accuracy: 97.8, status: 'active', loadTime: 45, memoryUsage: 128 },
      { id: 'model_002', name: 'Code Generator', type: 'code_generation', accuracy: 94.2, status: 'active', loadTime: 52, memoryUsage: 156 },
      { id: 'model_003', name: 'Pattern Analyzer', type: 'pattern_analysis', accuracy: 91.6, status: 'active', loadTime: 38, memoryUsage: 89 },
      { id: 'model_004', name: 'Decision Engine', type: 'decision_making', accuracy: 89.3, status: 'training', loadTime: 67, memoryUsage: 203 }
    ];
  });

  ipcMain.handle('train-all-neural-models', async (event, config) => {
    console.log('Training all neural models:', config);
    return { success: true, modelsCount: 27 };
  });

  ipcMain.handle('run-inference', async (event, modelId, config) => {
    console.log('Running inference:', modelId, config);
    
    // Simulate inference time
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    return { 
      success: true, 
      output: `Inference result for model ${modelId}:\n\nInput: ${config.input}\n\nGenerated response based on neural processing...` 
    };
  });

  // Performance Operations
  ipcMain.handle('get-performance-metrics', async () => {
    return {
      ...mockData.performanceMetrics,
      cpu: mockData.performanceMetrics.cpu + Math.random() * 10 - 5,
      memory: mockData.performanceMetrics.memory + Math.random() * 20 - 10,
      latency: mockData.performanceMetrics.latency + Math.random() * 30 - 15
    };
  });

  ipcMain.handle('get-optimization-status', async () => {
    return {
      batchSpawning: true,
      queenCoordination: true,
      neuralPatterns: true,
      memoryPooling: true,
      tokenOptimization: true,
      wasmAcceleration: false
    };
  });

  ipcMain.handle('run-performance-optimization', async () => {
    console.log('Running performance optimization...');
    
    // Simulate optimization
    await new Promise(resolve => setTimeout(resolve, 3000));
    
    return { success: true, improvement: '15%' };
  });

  // Quality Operations
  ipcMain.handle('get-quality-metrics', async () => {
    return mockData.qualityMetrics;
  });

  ipcMain.handle('run-quality-analysis', async (event, config) => {
    console.log('Running quality analysis:', config);
    
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    return { success: true };
  });

  // Settings Operations
  ipcMain.handle('get-settings', async () => {
    return mockData.settings;
  });

  ipcMain.handle('save-settings', async (event, settings) => {
    mockData.settings = { ...mockData.settings, ...settings };
    console.log('Settings saved:', settings);
    return { success: true };
  });

  ipcMain.handle('get-default-settings', async () => {
    return {
      maxParallelBenchmarks: 5,
      enableNeuralOptimization: true,
      enableSwarmCoordination: true,
      darkMode: true,
      autoRefreshInterval: 3000,
      defaultTopology: 'hierarchical',
      defaultCoordination: 'queen',
      maxAgents: 15
    };
  });

  // File Operations
  ipcMain.handle('show-save-dialog', async (event, options) => {
    return await dialog.showSaveDialog(mainWindow, options);
  });

  ipcMain.handle('show-open-dialog', async (event, options) => {
    return await dialog.showOpenDialog(mainWindow, options);
  });

  ipcMain.handle('export-settings', async (event, filePath, settings) => {
    try {
      await fs.writeFile(filePath, JSON.stringify(settings, null, 2));
      return { success: true };
    } catch (error) {
      return { success: false, error: error.message };
    }
  });

  ipcMain.handle('import-settings', async (event, filePath) => {
    try {
      const data = await fs.readFile(filePath, 'utf8');
      return JSON.parse(data);
    } catch (error) {
      return null;
    }
  });
}

// App event handlers
app.whenReady().then(() => {
  createWindow();
  createMenu();
  setupIpcHandlers();

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });

  // Start periodic updates
  setInterval(() => {
    if (mainWindow && !mainWindow.isDestroyed()) {
      mainWindow.webContents.send('metrics-update', {
        overallScore: mockData.performanceMetrics.overallScore + Math.random() * 2 - 1
      });
    }
  }, 5000);
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

// Security: Prevent new window creation
app.on('web-contents-created', (event, contents) => {
  contents.on('new-window', (event, navigationUrl) => {
    event.preventDefault();
  });
});

// Handle protocol for deep linking (future feature)
if (process.defaultApp) {
  if (process.argv.length >= 2) {
    app.setAsDefaultProtocolClient('claude-flow', process.execPath, [path.resolve(process.argv[1])]);
  }
} else {
  app.setAsDefaultProtocolClient('claude-flow');
}