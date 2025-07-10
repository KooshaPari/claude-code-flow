import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Chip,
  LinearProgress,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Switch,
  FormControlLabel,
  Alert,
  AlertTitle,
  Divider,
  IconButton,
  Tooltip,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Tab,
  Tabs,
  TabPanel
} from '@mui/material';
import {
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  Settings as SettingsIcon,
  Download as DownloadIcon,
  Visibility as ViewIcon,
  Speed as SpeedIcon,
  Code as CodeIcon,
  Psychology as PsychologyIcon,
  Timeline as TimelineIcon
} from '@mui/icons-material';
import { motion, AnimatePresence } from 'framer-motion';

const BenchmarkRunner = ({ onNotification, systemStatus }) => {
  const [activeTab, setActiveTab] = useState(0);
  const [selectedBenchmarks, setSelectedBenchmarks] = useState({
    sweBench: ['verified', 'lite'],
    humanEval: ['base', 'plus'],
    bigCode: ['multipl_e', 'ds_1000']
  });
  const [benchmarkConfig, setBenchmarkConfig] = useState({
    parallel: true,
    maxInstances: 50,
    includeMultimodal: true,
    useNeuralOptimization: true,
    saveResults: true
  });
  const [runningBenchmarks, setRunningBenchmarks] = useState({});
  const [benchmarkResults, setBenchmarkResults] = useState({});
  const [configDialogOpen, setConfigDialogOpen] = useState(false);

  const benchmarkSuites = {
    sweBench: {
      name: 'SWE-Bench Suite',
      description: 'Real GitHub issues & code challenges',
      icon: SpeedIcon,
      color: '#ff7a00',
      targetScore: 84.8, // Ruv's record
      variants: [
        { id: 'verified', name: 'Verified', description: 'Human-validated 500 problems', difficulty: 'High' },
        { id: 'lite', name: 'Lite', description: 'Cost-optimized 300 problems', difficulty: 'Medium' },
        { id: 'full', name: 'Full', description: 'Complete 2,294 problems', difficulty: 'Expert' },
        { id: 'multimodal', name: 'Multimodal', description: 'Visual elements integration', difficulty: 'Expert' },
        { id: 'multilingual', name: 'Multilingual', description: '17 programming languages', difficulty: 'High' },
        { id: 'enterprise', name: 'Enterprise', description: 'Complex enterprise scenarios', difficulty: 'Expert' },
        { id: 'security', name: 'Security', description: 'Security-focused challenges', difficulty: 'High' }
      ]
    },
    humanEval: {
      name: 'HumanEval Ecosystem',
      description: 'Programming competency evaluation',
      icon: CodeIcon,
      color: '#64ffda',
      targetScore: 92.0,
      variants: [
        { id: 'base', name: 'Base', description: '164 hand-written problems', difficulty: 'Medium' },
        { id: 'plus', name: 'Plus', description: '80x more test cases', difficulty: 'High' },
        { id: 'mbpp', name: 'MBPP', description: '974 crowd-sourced problems', difficulty: 'Medium' },
        { id: 'contests', name: 'Code Contests', description: 'Competitive programming', difficulty: 'Expert' },
        { id: 'apps', name: 'APPS', description: 'Program synthesis challenges', difficulty: 'High' },
        { id: 'live', name: 'LiveCodeBench', description: 'Fresh contamination-free problems', difficulty: 'High' },
        { id: 'multilingual', name: 'HumanEval-X', description: '5 programming languages', difficulty: 'High' }
      ]
    },
    bigCode: {
      name: 'BigCode Suite',
      description: 'Large-scale code generation',
      icon: PsychologyIcon,
      color: '#ffb74d',
      targetScore: 87.0,
      variants: [
        { id: 'multipl_e', name: 'MultiPL-E', description: '18 language translation', difficulty: 'High' },
        { id: 'ds_1000', name: 'DS-1000', description: 'Data science problems', difficulty: 'High' },
        { id: 'codexglue', name: 'CodeXGLUE', description: 'General code understanding', difficulty: 'Medium' },
        { id: 'translation', name: 'Code Translation', description: 'Cross-language conversion', difficulty: 'High' },
        { id: 'refinement', name: 'Code Refinement', description: 'Automatic program repair', difficulty: 'Expert' },
        { id: 'multimodal', name: 'Multimodal BigCode', description: 'Visual code generation', difficulty: 'Expert' },
        { id: 'spider', name: 'Spider SQL', description: 'Database query generation', difficulty: 'High' }
      ]
    }
  };

  useEffect(() => {
    loadBenchmarkStatus();
    const interval = setInterval(loadBenchmarkStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  const loadBenchmarkStatus = async () => {
    try {
      // Load running benchmarks status
      const status = await window.electronAPI.getBenchmarkStatus();
      if (status) {
        setRunningBenchmarks(status.running || {});
        setBenchmarkResults(status.results || {});
      }
    } catch (error) {
      console.error('Failed to load benchmark status:', error);
    }
  };

  const handleRunBenchmark = async (suiteKey, variants) => {
    try {
      const config = {
        variants: variants || selectedBenchmarks[suiteKey],
        maxInstances: benchmarkConfig.maxInstances,
        parallel: benchmarkConfig.parallel,
        includeMultimodal: benchmarkConfig.includeMultimodal,
        useNeuralOptimization: benchmarkConfig.useNeuralOptimization
      };

      onNotification(`Starting ${benchmarkSuites[suiteKey].name}...`, 'info');
      
      const result = await window.electronAPI.runBenchmark(suiteKey, config);
      
      if (result.success) {
        onNotification(`${benchmarkSuites[suiteKey].name} started successfully`, 'success');
        setRunningBenchmarks(prev => ({
          ...prev,
          [suiteKey]: { status: 'running', progress: 0, startTime: Date.now() }
        }));
      } else {
        onNotification(`Failed to start ${benchmarkSuites[suiteKey].name}`, 'error');
      }
    } catch (error) {
      console.error('Failed to run benchmark:', error);
      onNotification(`Error starting benchmark: ${error.message}`, 'error');
    }
  };

  const handleStopBenchmark = async (suiteKey) => {
    try {
      await window.electronAPI.stopBenchmark(suiteKey);
      onNotification(`Stopped ${benchmarkSuites[suiteKey].name}`, 'warning');
      setRunningBenchmarks(prev => {
        const updated = { ...prev };
        delete updated[suiteKey];
        return updated;
      });
    } catch (error) {
      console.error('Failed to stop benchmark:', error);
      onNotification(`Error stopping benchmark: ${error.message}`, 'error');
    }
  };

  const handleRunAllBenchmarks = async () => {
    try {
      onNotification('Starting comprehensive benchmark suite...', 'info');
      
      for (const [suiteKey, variants] of Object.entries(selectedBenchmarks)) {
        if (variants.length > 0) {
          await handleRunBenchmark(suiteKey, variants);
          // Small delay between benchmark starts
          await new Promise(resolve => setTimeout(resolve, 1000));
        }
      }
    } catch (error) {
      console.error('Failed to run all benchmarks:', error);
      onNotification('Failed to start comprehensive benchmarks', 'error');
    }
  };

  const handleVariantToggle = (suiteKey, variantId) => {
    setSelectedBenchmarks(prev => ({
      ...prev,
      [suiteKey]: prev[suiteKey].includes(variantId)
        ? prev[suiteKey].filter(id => id !== variantId)
        : [...prev[suiteKey], variantId]
    }));
  };

  const getProgressColor = (progress) => {
    if (progress < 30) return 'error';
    if (progress < 70) return 'warning';
    return 'success';
  };

  const formatDuration = (startTime) => {
    const duration = Date.now() - startTime;
    const minutes = Math.floor(duration / 60000);
    const seconds = Math.floor((duration % 60000) / 1000);
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  const TabPanel = ({ children, value, index, ...other }) => (
    <div
      role="tabpanel"
      hidden={value !== index}
      {...other}
    >
      {value === index && <Box sx={{ pt: 2 }}>{children}</Box>}
    </div>
  );

  const renderBenchmarkSuite = (suiteKey, suite) => {
    const isRunning = runningBenchmarks[suiteKey];
    const result = benchmarkResults[suiteKey];
    const IconComponent = suite.icon;

    return (
      <Card key={suiteKey} sx={{ mb: 3 }}>
        <CardContent>
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 2 }}>
            <Box sx={{ display: 'flex', alignItems: 'center' }}>
              <IconComponent sx={{ mr: 2, color: suite.color }} />
              <Box>
                <Typography variant="h6">{suite.name}</Typography>
                <Typography variant="body2" color="text.secondary">
                  {suite.description}
                </Typography>
              </Box>
            </Box>
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
              {result && (
                <Chip 
                  label={`Score: ${result.score?.toFixed(1)}%`}
                  color={result.score >= suite.targetScore ? 'success' : 'warning'}
                  variant="filled"
                />
              )}
              <Chip 
                label={`Target: ${suite.targetScore}%`}
                variant="outlined"
                size="small"
              />
              {isRunning ? (
                <Button
                  variant="contained"
                  color="error"
                  startIcon={<StopIcon />}
                  onClick={() => handleStopBenchmark(suiteKey)}
                  size="small"
                >
                  Stop
                </Button>
              ) : (
                <Button
                  variant="contained"
                  startIcon={<PlayIcon />}
                  onClick={() => handleRunBenchmark(suiteKey)}
                  disabled={selectedBenchmarks[suiteKey].length === 0}
                  size="small"
                  sx={{ backgroundColor: suite.color }}
                >
                  Run
                </Button>
              )}
            </Box>
          </Box>

          {isRunning && (
            <Box sx={{ mb: 2 }}>
              <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                <Typography variant="body2">
                  Running... {formatDuration(isRunning.startTime)}
                </Typography>
                <Typography variant="body2">
                  {isRunning.progress?.toFixed(1) || 0}%
                </Typography>
              </Box>
              <LinearProgress 
                variant="determinate" 
                value={isRunning.progress || 0}
                color={getProgressColor(isRunning.progress || 0)}
                sx={{ height: 8, borderRadius: 4 }}
              />
            </Box>
          )}

          <Typography variant="subtitle2" sx={{ mb: 1, fontWeight: 600 }}>
            Variants ({selectedBenchmarks[suiteKey].length} selected):
          </Typography>
          
          <Grid container spacing={1}>
            {suite.variants.map((variant) => (
              <Grid item xs={12} sm={6} md={4} key={variant.id}>
                <Card 
                  variant="outlined" 
                  sx={{ 
                    cursor: 'pointer',
                    border: selectedBenchmarks[suiteKey].includes(variant.id) 
                      ? `2px solid ${suite.color}` 
                      : '1px solid rgba(255,255,255,0.1)',
                    '&:hover': {
                      backgroundColor: 'rgba(255,255,255,0.05)'
                    }
                  }}
                  onClick={() => handleVariantToggle(suiteKey, variant.id)}
                >
                  <CardContent sx={{ p: 2 }}>
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 1 }}>
                      <Typography variant="subtitle2" fontWeight={600}>
                        {variant.name}
                      </Typography>
                      <Chip 
                        label={variant.difficulty} 
                        size="small" 
                        color={
                          variant.difficulty === 'Expert' ? 'error' :
                          variant.difficulty === 'High' ? 'warning' :
                          variant.difficulty === 'Medium' ? 'info' : 'success'
                        }
                      />
                    </Box>
                    <Typography variant="caption" color="text.secondary">
                      {variant.description}
                    </Typography>
                  </CardContent>
                </Card>
              </Grid>
            ))}
          </Grid>
        </CardContent>
      </Card>
    );
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
            Benchmark Runner
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Comprehensive AI evaluation with SWE-Bench, HumanEval, and BigCode suites
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <Button
            variant="outlined"
            startIcon={<SettingsIcon />}
            onClick={() => setConfigDialogOpen(true)}
          >
            Configure
          </Button>
          <Button
            variant="contained"
            size="large"
            startIcon={<PlayIcon />}
            onClick={handleRunAllBenchmarks}
            disabled={Object.values(selectedBenchmarks).every(variants => variants.length === 0)}
            sx={{ 
              background: 'linear-gradient(45deg, #ff7a00 30%, #e65100 90%)',
              px: 3
            }}
          >
            Run All Benchmarks
          </Button>
        </Box>
      </Box>

      {/* Performance Target Alert */}
      <Alert severity="info" sx={{ mb: 3 }}>
        <AlertTitle>Performance Target: Maintain Ruv's 84.8% SWE-Bench Record</AlertTitle>
        Current neural optimization and swarm coordination active for maximum performance.
        <Box sx={{ mt: 1, display: 'flex', gap: 2 }}>
          <Chip 
            label={`Neural: ${systemStatus.neuralActive ? 'Active' : 'Inactive'}`}
            color={systemStatus.neuralActive ? 'success' : 'error'}
            size="small"
          />
          <Chip 
            label={`Swarm: ${systemStatus.swarmActive ? 'Active' : 'Inactive'}`}
            color={systemStatus.swarmActive ? 'success' : 'error'}
            size="small"
          />
          <Chip 
            label={`Parallel: ${benchmarkConfig.parallel ? 'Enabled' : 'Disabled'}`}
            color={benchmarkConfig.parallel ? 'success' : 'warning'}
            size="small"
          />
        </Box>
      </Alert>

      {/* Benchmark Suites */}
      <Box>
        {Object.entries(benchmarkSuites).map(([suiteKey, suite]) => 
          renderBenchmarkSuite(suiteKey, suite)
        )}
      </Box>

      {/* Configuration Dialog */}
      <Dialog 
        open={configDialogOpen} 
        onClose={() => setConfigDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Benchmark Configuration</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <FormControlLabel
              control={
                <Switch
                  checked={benchmarkConfig.parallel}
                  onChange={(e) => setBenchmarkConfig(prev => ({ ...prev, parallel: e.target.checked }))}
                />
              }
              label="Parallel Execution (Recommended)"
              sx={{ mb: 2 }}
            />
            
            <FormControlLabel
              control={
                <Switch
                  checked={benchmarkConfig.includeMultimodal}
                  onChange={(e) => setBenchmarkConfig(prev => ({ ...prev, includeMultimodal: e.target.checked }))}
                />
              }
              label="Include Multimodal Benchmarks"
              sx={{ mb: 2 }}
            />
            
            <FormControlLabel
              control={
                <Switch
                  checked={benchmarkConfig.useNeuralOptimization}
                  onChange={(e) => setBenchmarkConfig(prev => ({ ...prev, useNeuralOptimization: e.target.checked }))}
                />
              }
              label="Neural Pattern Optimization"
              sx={{ mb: 2 }}
            />
            
            <FormControl fullWidth sx={{ mb: 2 }}>
              <InputLabel>Max Instances Per Variant</InputLabel>
              <Select
                value={benchmarkConfig.maxInstances}
                onChange={(e) => setBenchmarkConfig(prev => ({ ...prev, maxInstances: e.target.value }))}
              >
                <MenuItem value={25}>25 (Quick Test)</MenuItem>
                <MenuItem value={50}>50 (Standard)</MenuItem>
                <MenuItem value={100}>100 (Comprehensive)</MenuItem>
                <MenuItem value={200}>200 (Full Evaluation)</MenuItem>
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfigDialogOpen(false)}>Cancel</Button>
          <Button onClick={() => setConfigDialogOpen(false)} variant="contained">
            Save Configuration
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default BenchmarkRunner;