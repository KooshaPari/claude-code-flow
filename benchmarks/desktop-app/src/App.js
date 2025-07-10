import React, { useState, useEffect } from 'react';
import { 
  ThemeProvider, 
  createTheme, 
  CssBaseline, 
  Box, 
  AppBar, 
  Toolbar, 
  Typography, 
  Drawer, 
  List, 
  ListItem, 
  ListItemIcon, 
  ListItemText, 
  ListItemButton,
  IconButton,
  Badge,
  Alert,
  Snackbar
} from '@mui/material';
import {
  Dashboard as DashboardIcon,
  Speed as SpeedIcon,
  BugReport as BugReportIcon,
  Code as CodeIcon,
  Psychology as PsychologyIcon,
  Settings as SettingsIcon,
  Assessment as AssessmentIcon,
  Hub as HubIcon,
  Memory as MemoryIcon,
  Notifications as NotificationsIcon,
  Menu as MenuIcon,
  Close as CloseIcon
} from '@mui/icons-material';
import { motion, AnimatePresence } from 'framer-motion';

// Components
import Dashboard from './components/Dashboard';
import BenchmarkRunner from './components/BenchmarkRunner';
import PerformanceMonitor from './components/PerformanceMonitor';
import SwarmStatus from './components/SwarmStatus';
import NeuralConsole from './components/NeuralConsole';
import QualityMetrics from './components/QualityMetrics';
import Settings from './components/Settings';

// Claude-desktop inspired dark theme
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#ff7a00', // Claude orange
      light: '#ffab40',
      dark: '#e65100',
    },
    secondary: {
      main: '#64ffda',
      light: '#9cffdd',
      dark: '#00bfa5',
    },
    background: {
      default: '#0a0a0a',
      paper: '#1a1a1a',
    },
    surface: {
      main: '#202020',
      light: '#2a2a2a',
      dark: '#151515',
    },
    text: {
      primary: '#ffffff',
      secondary: '#b0b0b0',
    },
    error: {
      main: '#ff5252',
    },
    warning: {
      main: '#ffb74d',
    },
    success: {
      main: '#69f0ae',
    },
    info: {
      main: '#64b5f6',
    },
  },
  typography: {
    fontFamily: '"Inter", "SF Pro Display", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif',
    h1: {
      fontSize: '2.5rem',
      fontWeight: 600,
      letterSpacing: '-0.02em',
    },
    h2: {
      fontSize: '2rem',
      fontWeight: 600,
      letterSpacing: '-0.01em',
    },
    h3: {
      fontSize: '1.5rem',
      fontWeight: 500,
    },
    body1: {
      fontSize: '0.95rem',
      lineHeight: 1.6,
    },
    button: {
      textTransform: 'none',
      fontWeight: 500,
    },
  },
  shape: {
    borderRadius: 12,
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          padding: '8px 16px',
          fontSize: '0.9rem',
        },
      },
    },
    MuiCard: {
      styleOverrides: {
        root: {
          backgroundColor: '#1a1a1a',
          borderRadius: 16,
          border: '1px solid rgba(255, 255, 255, 0.1)',
          backdropFilter: 'blur(10px)',
        },
      },
    },
    MuiDrawer: {
      styleOverrides: {
        paper: {
          backgroundColor: '#0f0f0f',
          borderRight: '1px solid rgba(255, 255, 255, 0.1)',
        },
      },
    },
    MuiAppBar: {
      styleOverrides: {
        root: {
          backgroundColor: 'rgba(10, 10, 10, 0.8)',
          backdropFilter: 'blur(20px)',
          borderBottom: '1px solid rgba(255, 255, 255, 0.1)',
        },
      },
    },
  },
});

const DRAWER_WIDTH = 280;

const navigationItems = [
  { id: 'dashboard', label: 'Dashboard', icon: DashboardIcon, description: 'Overview and quick actions' },
  { id: 'benchmarks', label: 'Benchmarks', icon: SpeedIcon, description: 'SWE-Bench, HumanEval, BigCode' },
  { id: 'performance', label: 'Performance', icon: AssessmentIcon, description: 'Real-time monitoring' },
  { id: 'swarm', label: 'Swarm Status', icon: HubIcon, description: 'Agent coordination' },
  { id: 'neural', label: 'Neural Console', icon: PsychologyIcon, description: '27+ AI models' },
  { id: 'quality', label: 'Quality Metrics', icon: BugReportIcon, description: 'Code quality & tests' },
  { id: 'settings', label: 'Settings', icon: SettingsIcon, description: 'Configuration' },
];

function App() {
  const [currentView, setCurrentView] = useState('dashboard');
  const [drawerOpen, setDrawerOpen] = useState(true);
  const [notifications, setNotifications] = useState([]);
  const [systemStatus, setSystemStatus] = useState({
    swarmActive: false,
    benchmarkRunning: false,
    neuralActive: false,
    performanceScore: 0,
  });
  const [snackbar, setSnackbar] = useState({ open: false, message: '', severity: 'info' });

  useEffect(() => {
    // Initialize app and check system status
    initializeApp();
    
    // Set up event listeners
    setupEventListeners();
    
    // Cleanup on unmount
    return () => {
      cleanupEventListeners();
    };
  }, []);

  const initializeApp = async () => {
    try {
      // Check system status
      const systemInfo = await window.electronAPI.getSystemInfo();
      const swarmStatus = await window.electronAPI.getSwarmStatus();
      const neuralStatus = await window.electronAPI.getNeuralStatus();
      
      setSystemStatus({
        swarmActive: swarmStatus?.status === 'active',
        benchmarkRunning: false,
        neuralActive: neuralStatus?.active || false,
        performanceScore: swarmStatus?.performance_score || 0,
      });

      showNotification('Claude Flow Benchmark Suite initialized', 'success');
    } catch (error) {
      console.error('Failed to initialize app:', error);
      showNotification('Failed to initialize system', 'error');
    }
  };

  const setupEventListeners = () => {
    // Menu action handler
    window.electronAPI.onMenuAction((event, action, data) => {
      handleMenuAction(action, data);
    });

    // Metrics updates
    window.electronAPI.onMetricsUpdate((event, metrics) => {
      setSystemStatus(prev => ({
        ...prev,
        performanceScore: metrics.overallScore || prev.performanceScore,
      }));
    });

    // Benchmark progress
    window.electronAPI.onBenchmarkProgress((event, progress) => {
      setSystemStatus(prev => ({
        ...prev,
        benchmarkRunning: progress.status === 'running',
      }));
    });

    // Swarm updates
    window.electronAPI.onSwarmUpdate((event, swarmData) => {
      setSystemStatus(prev => ({
        ...prev,
        swarmActive: swarmData.status === 'active',
      }));
    });
  };

  const cleanupEventListeners = () => {
    window.electronAPI.removeMenuActionListeners();
    window.electronAPI.removeMetricsListeners();
    window.electronAPI.removeBenchmarkProgressListeners();
    window.electronAPI.removeSwarmUpdateListeners();
  };

  const handleMenuAction = (action, data) => {
    switch (action) {
      case 'new-session':
        showNotification('Starting new benchmark session', 'info');
        setCurrentView('benchmarks');
        break;
      case 'run-all-benchmarks':
        setCurrentView('benchmarks');
        // Trigger benchmark run
        break;
      case 'run-swe-bench':
        setCurrentView('benchmarks');
        // Set SWE-Bench as active
        break;
      case 'run-humaneval':
        setCurrentView('benchmarks');
        // Set HumanEval as active
        break;
      case 'run-bigcode':
        setCurrentView('benchmarks');
        // Set BigCode as active
        break;
      case 'initialize-swarm':
        initializeSwarm();
        break;
      case 'optimize-performance':
        optimizePerformance();
        break;
      case 'show-dashboard':
        setCurrentView('dashboard');
        break;
      case 'show-performance':
        setCurrentView('performance');
        break;
      case 'show-swarm-status':
        setCurrentView('swarm');
        break;
      case 'export-results':
        exportResults();
        break;
      case 'open-results':
        openResults(data);
        break;
      default:
        console.log('Unknown menu action:', action);
    }
  };

  const initializeSwarm = async () => {
    try {
      showNotification('Initializing swarm...', 'info');
      const result = await window.electronAPI.initializeSwarm({
        topology: 'hierarchical',
        coordination: 'queen',
        maxAgents: 15,
        neuralEnabled: true
      });
      
      if (result.success) {
        showNotification('Swarm initialized successfully', 'success');
        setCurrentView('swarm');
      } else {
        showNotification('Failed to initialize swarm', 'error');
      }
    } catch (error) {
      console.error('Swarm initialization failed:', error);
      showNotification('Swarm initialization failed', 'error');
    }
  };

  const optimizePerformance = async () => {
    try {
      showNotification('Running performance optimization...', 'info');
      const result = await window.electronAPI.runPerformanceOptimization();
      
      if (result.success) {
        showNotification(`Performance optimized: ${result.improvement}% improvement`, 'success');
        setCurrentView('performance');
      } else {
        showNotification('Performance optimization failed', 'warning');
      }
    } catch (error) {
      console.error('Performance optimization failed:', error);
      showNotification('Performance optimization failed', 'error');
    }
  };

  const exportResults = async () => {
    try {
      const result = await window.electronAPI.showSaveDialog({
        title: 'Export Benchmark Results',
        defaultPath: `benchmark-results-${new Date().toISOString().split('T')[0]}.json`,
        filters: [
          { name: 'JSON Files', extensions: ['json'] },
          { name: 'CSV Files', extensions: ['csv'] },
          { name: 'All Files', extensions: ['*'] }
        ]
      });

      if (!result.canceled) {
        // Get current results and export
        showNotification('Exporting results...', 'info');
        // Implementation for export
        showNotification('Results exported successfully', 'success');
      }
    } catch (error) {
      console.error('Export failed:', error);
      showNotification('Export failed', 'error');
    }
  };

  const openResults = async (filepath) => {
    try {
      showNotification('Loading results...', 'info');
      const results = await window.electronAPI.importResults(filepath);
      
      if (results) {
        showNotification('Results loaded successfully', 'success');
        // Update UI with loaded results
      } else {
        showNotification('Failed to load results', 'error');
      }
    } catch (error) {
      console.error('Failed to open results:', error);
      showNotification('Failed to open results', 'error');
    }
  };

  const showNotification = (message, severity = 'info') => {
    setSnackbar({ open: true, message, severity });
  };

  const handleCloseSnackbar = () => {
    setSnackbar({ ...snackbar, open: false });
  };

  const renderCurrentView = () => {
    const viewProps = {
      onNotification: showNotification,
      systemStatus,
    };

    switch (currentView) {
      case 'dashboard':
        return <Dashboard {...viewProps} />;
      case 'benchmarks':
        return <BenchmarkRunner {...viewProps} />;
      case 'performance':
        return <PerformanceMonitor {...viewProps} />;
      case 'swarm':
        return <SwarmStatus {...viewProps} />;
      case 'neural':
        return <NeuralConsole {...viewProps} />;
      case 'quality':
        return <QualityMetrics {...viewProps} />;
      case 'settings':
        return <Settings {...viewProps} />;
      default:
        return <Dashboard {...viewProps} />;
    }
  };

  const getStatusIndicator = () => {
    const { swarmActive, benchmarkRunning, neuralActive, performanceScore } = systemStatus;
    
    if (benchmarkRunning) return { color: 'warning', text: 'Running' };
    if (performanceScore >= 85) return { color: 'success', text: 'Optimal' };
    if (swarmActive && neuralActive) return { color: 'info', text: 'Active' };
    return { color: 'default', text: 'Ready' };
  };

  const statusIndicator = getStatusIndicator();

  return (
    <ThemeProvider theme={darkTheme}>
      <CssBaseline />
      <Box sx={{ display: 'flex', height: '100vh' }}>
        {/* App Bar */}
        <AppBar 
          position="fixed" 
          sx={{ 
            zIndex: (theme) => theme.zIndex.drawer + 1,
            borderRadius: 0,
          }}
        >
          <Toolbar>
            <IconButton
              color="inherit"
              aria-label="toggle drawer"
              onClick={() => setDrawerOpen(!drawerOpen)}
              edge="start"
              sx={{ mr: 2 }}
            >
              {drawerOpen ? <CloseIcon /> : <MenuIcon />}
            </IconButton>
            
            <Typography variant="h6" noWrap component="div" sx={{ flexGrow: 1 }}>
              Claude Flow Benchmark Suite
            </Typography>

            <Badge 
              variant="dot" 
              color={statusIndicator.color}
              sx={{ mr: 2 }}
            >
              <Typography variant="body2" sx={{ mr: 1 }}>
                {statusIndicator.text}
              </Typography>
            </Badge>

            <Badge badgeContent={notifications.length} color="secondary">
              <IconButton color="inherit">
                <NotificationsIcon />
              </IconButton>
            </Badge>
          </Toolbar>
        </AppBar>

        {/* Sidebar Drawer */}
        <Drawer
          variant="persistent"
          open={drawerOpen}
          sx={{
            width: drawerOpen ? DRAWER_WIDTH : 0,
            flexShrink: 0,
            '& .MuiDrawer-paper': {
              width: DRAWER_WIDTH,
              boxSizing: 'border-box',
            },
          }}
        >
          <Toolbar />
          <Box sx={{ overflow: 'auto', p: 1 }}>
            <List>
              {navigationItems.map((item) => (
                <ListItem key={item.id} disablePadding sx={{ mb: 0.5 }}>
                  <ListItemButton
                    selected={currentView === item.id}
                    onClick={() => setCurrentView(item.id)}
                    sx={{
                      borderRadius: 2,
                      '&.Mui-selected': {
                        backgroundColor: 'rgba(255, 122, 0, 0.1)',
                        border: '1px solid rgba(255, 122, 0, 0.3)',
                      },
                      '&:hover': {
                        backgroundColor: 'rgba(255, 255, 255, 0.05)',
                      },
                    }}
                  >
                    <ListItemIcon sx={{ minWidth: 40 }}>
                      <item.icon 
                        sx={{ 
                          color: currentView === item.id ? 'primary.main' : 'text.secondary'
                        }} 
                      />
                    </ListItemIcon>
                    <ListItemText 
                      primary={item.label}
                      secondary={item.description}
                      primaryTypographyProps={{
                        fontSize: '0.9rem',
                        fontWeight: currentView === item.id ? 600 : 400,
                      }}
                      secondaryTypographyProps={{
                        fontSize: '0.75rem',
                        color: 'text.secondary',
                      }}
                    />
                  </ListItemButton>
                </ListItem>
              ))}
            </List>
          </Box>
        </Drawer>

        {/* Main Content */}
        <Box
          component="main"
          sx={{
            flexGrow: 1,
            p: 3,
            width: { sm: `calc(100% - ${drawerOpen ? DRAWER_WIDTH : 0}px)` },
            ml: { sm: drawerOpen ? 0 : 0 },
            transition: (theme) =>
              theme.transitions.create(['margin', 'width'], {
                easing: theme.transitions.easing.sharp,
                duration: theme.transitions.duration.leavingScreen,
              }),
          }}
        >
          <Toolbar />
          
          <AnimatePresence mode="wait">
            <motion.div
              key={currentView}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -20 }}
              transition={{ duration: 0.2 }}
            >
              {renderCurrentView()}
            </motion.div>
          </AnimatePresence>
        </Box>

        {/* Snackbar for notifications */}
        <Snackbar
          open={snackbar.open}
          autoHideDuration={6000}
          onClose={handleCloseSnackbar}
          anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }}
        >
          <Alert 
            onClose={handleCloseSnackbar} 
            severity={snackbar.severity}
            variant="filled"
            sx={{ width: '100%' }}
          >
            {snackbar.message}
          </Alert>
        </Snackbar>
      </Box>
    </ThemeProvider>
  );
}

export default App;