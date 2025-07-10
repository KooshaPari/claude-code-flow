const { app, BrowserWindow, Menu, ipcMain, dialog, shell } = require('electron');
const path = require('path');
const isDev = require('electron-is-dev');
const Store = require('electron-store');

// Initialize electron store for persistent settings
const store = new Store();

let mainWindow;

function createWindow() {
  // Create the browser window with Claude-desktop-like styling
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    minWidth: 1200,
    minHeight: 800,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      enableRemoteModule: false,
      preload: path.join(__dirname, 'preload.js')
    },
    titleBarStyle: 'hiddenInset', // macOS style
    frame: process.platform !== 'darwin',
    show: false,
    backgroundColor: '#1a1a1a', // Dark theme like Claude
    icon: path.join(__dirname, '../public/icon.png')
  });

  // Load the React app
  const startUrl = isDev 
    ? 'http://localhost:3000' 
    : `file://${path.join(__dirname, '../build/index.html')}`;
  
  mainWindow.loadURL(startUrl);

  // Show window when ready to prevent visual flash
  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
    
    // Focus on window
    if (isDev) {
      mainWindow.webContents.openDevTools();
    }
  });

  // Handle window closed
  mainWindow.on('closed', () => {
    mainWindow = null;
  });

  // Handle external links
  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    shell.openExternal(url);
    return { action: 'deny' };
  });

  // Set up menu
  createMenu();
}

function createMenu() {
  const template = [
    {
      label: 'File',
      submenu: [
        {
          label: 'New Benchmark Session',
          accelerator: 'CmdOrCtrl+N',
          click: () => {
            mainWindow.webContents.send('menu-action', 'new-session');
          }
        },
        {
          label: 'Open Results',
          accelerator: 'CmdOrCtrl+O',
          click: async () => {
            const result = await dialog.showOpenDialog(mainWindow, {
              properties: ['openFile'],
              filters: [
                { name: 'JSON Files', extensions: ['json'] },
                { name: 'All Files', extensions: ['*'] }
              ]
            });
            
            if (!result.canceled) {
              mainWindow.webContents.send('menu-action', 'open-results', result.filePaths[0]);
            }
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
        { type: 'separator' },
        process.platform === 'darwin' ? { role: 'close' } : { role: 'quit' }
      ]
    },
    {
      label: 'Edit',
      submenu: [
        { role: 'undo' },
        { role: 'redo' },
        { type: 'separator' },
        { role: 'cut' },
        { role: 'copy' },
        { role: 'paste' },
        { role: 'selectall' }
      ]
    },
    {
      label: 'Benchmark',
      submenu: [
        {
          label: 'Run All Benchmarks',
          accelerator: 'CmdOrCtrl+R',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-all-benchmarks');
          }
        },
        {
          label: 'Run SWE-Bench Suite',
          accelerator: 'CmdOrCtrl+1',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-swe-bench');
          }
        },
        {
          label: 'Run HumanEval Suite',
          accelerator: 'CmdOrCtrl+2',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-humaneval');
          }
        },
        {
          label: 'Run BigCode Suite',
          accelerator: 'CmdOrCtrl+3',
          click: () => {
            mainWindow.webContents.send('menu-action', 'run-bigcode');
          }
        },
        { type: 'separator' },
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
        { role: 'reload' },
        { role: 'forceReload' },
        { role: 'toggleDevTools' },
        { type: 'separator' },
        { role: 'resetZoom' },
        { role: 'zoomIn' },
        { role: 'zoomOut' },
        { type: 'separator' },
        { role: 'togglefullscreen' },
        { type: 'separator' },
        {
          label: 'Dashboard',
          accelerator: 'CmdOrCtrl+D',
          click: () => {
            mainWindow.webContents.send('menu-action', 'show-dashboard');
          }
        },
        {
          label: 'Performance Monitor',
          accelerator: 'CmdOrCtrl+M',
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
        }
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
      label: 'Help',
      submenu: [
        {
          label: 'Documentation',
          click: () => {
            shell.openExternal('https://github.com/claude-flow/benchmark-suite');
          }
        },
        {
          label: 'Report Issue',
          click: () => {
            shell.openExternal('https://github.com/claude-flow/benchmark-suite/issues');
          }
        },
        { type: 'separator' },
        {
          label: 'About Claude Flow Benchmark Suite',
          click: () => {
            dialog.showMessageBox(mainWindow, {
              type: 'info',
              title: 'About',
              message: 'Claude Flow Benchmark Suite',
              detail: 'Advanced AI benchmarking with swarm intelligence\nVersion 2.0.0\n\nAchieving 84.8%+ SWE-Bench performance with neural coordination.'
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
  }

  const menu = Menu.buildFromTemplate(template);
  Menu.setApplicationMenu(menu);
}

// IPC handlers for communication with renderer process
ipcMain.handle('get-app-version', () => {
  return app.getVersion();
});

ipcMain.handle('get-stored-data', (event, key) => {
  return store.get(key);
});

ipcMain.handle('set-stored-data', (event, key, value) => {
  store.set(key, value);
  return true;
});

ipcMain.handle('show-save-dialog', async (event, options) => {
  const result = await dialog.showSaveDialog(mainWindow, options);
  return result;
});

ipcMain.handle('show-open-dialog', async (event, options) => {
  const result = await dialog.showOpenDialog(mainWindow, options);
  return result;
});

ipcMain.handle('show-message-box', async (event, options) => {
  const result = await dialog.showMessageBox(mainWindow, options);
  return result;
});

// App event handlers
app.whenReady().then(createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

// Security: Prevent new window creation
app.on('web-contents-created', (event, contents) => {
  contents.on('new-window', (event, navigationUrl) => {
    event.preventDefault();
    shell.openExternal(navigationUrl);
  });
});

// Handle certificate errors
app.on('certificate-error', (event, webContents, url, error, certificate, callback) => {
  if (isDev) {
    // In development, ignore certificate errors
    event.preventDefault();
    callback(true);
  } else {
    // In production, use default behavior
    callback(false);
  }
});

// Prevent navigation to external websites
app.on('web-contents-created', (event, contents) => {
  contents.on('will-navigate', (event, navigationUrl) => {
    const parsedUrl = new URL(navigationUrl);
    
    if (parsedUrl.origin !== 'http://localhost:3000' && !navigationUrl.startsWith('file://')) {
      event.preventDefault();
    }
  });
});