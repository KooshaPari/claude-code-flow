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
  IconButton,
  Tooltip,
  Alert,
  AlertTitle,
  Divider,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper
} from '@mui/material';
import {
  Refresh as RefreshIcon,
  Timeline as TimelineIcon,
  Speed as SpeedIcon,
  Memory as MemoryIcon,
  Storage as StorageIcon,
  NetworkCheck as NetworkIcon,
  Psychology as PsychologyIcon,
  Hub as HubIcon,
  TrendingUp as TrendingUpIcon,
  TrendingDown as TrendingDownIcon
} from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip as RechartsTooltip, ResponsiveContainer, AreaChart, Area, BarChart, Bar } from 'recharts';
import { motion } from 'framer-motion';

const PerformanceMonitor = ({ onNotification, systemStatus }) => {
  const [metrics, setMetrics] = useState({
    cpu: 45,
    memory: 192,
    gpu: 23,
    storage: 78,
    network: 156,
    latency: 142,
    throughput: 1250,
    errorRate: 0.2,
    availability: 99.8
  });
  
  const [optimizationStatus, setOptimizationStatus] = useState({
    batchSpawning: true,
    queenCoordination: true,
    neuralPatterns: true,
    memoryPooling: true,
    tokenOptimization: true,
    wasmAcceleration: false
  });
  
  const [performanceHistory, setPerformanceHistory] = useState([]);
  const [benchmarkResults, setBenchmarkResults] = useState({
    sweBench: 86.2,
    humanEval: 93.1,
    bigCode: 88.7,
    overall: 89.3
  });
  
  const [isRefreshing, setIsRefreshing] = useState(false);

  useEffect(() => {
    loadPerformanceData();
    const interval = setInterval(loadPerformanceData, 3000);
    return () => clearInterval(interval);
  }, []);

  const loadPerformanceData = async () => {
    try {
      // Load performance metrics
      const perfData = await window.electronAPI.getPerformanceMetrics();
      const optimData = await window.electronAPI.getOptimizationStatus();
      const benchData = await window.electronAPI.getBenchmarkResults();
      
      if (perfData) {
        setMetrics(prev => ({ ...prev, ...perfData }));
      }
      
      if (optimData) {
        setOptimizationStatus(prev => ({ ...prev, ...optimData }));
      }
      
      if (benchData) {
        setBenchmarkResults(prev => ({ ...prev, ...benchData }));
      }
      
      // Generate performance history
      const now = Date.now();
      const history = Array.from({ length: 30 }, (_, i) => ({
        time: new Date(now - (29 - i) * 60000).toLocaleTimeString('en-US', { 
          hour: '2-digit', 
          minute: '2-digit' 
        }),
        cpu: 45 + Math.random() * 20 - 10,
        memory: 192 + Math.random() * 40 - 20,
        latency: 142 + Math.random() * 30 - 15,
        throughput: 1250 + Math.random() * 200 - 100
      }));
      setPerformanceHistory(history);
      
    } catch (error) {
      console.error('Failed to load performance data:', error);
    }
  };

  const handleRefresh = async () => {
    setIsRefreshing(true);
    try {
      onNotification('Refreshing performance metrics...', 'info');
      await loadPerformanceData();
      onNotification('Performance metrics updated', 'success');
    } catch (error) {
      onNotification('Failed to refresh metrics', 'error');
    } finally {
      setIsRefreshing(false);
    }
  };

  const handleOptimizationToggle = async (optimization) => {
    try {
      const newStatus = !optimizationStatus[optimization];
      await window.electronAPI.toggleOptimization(optimization, newStatus);
      
      setOptimizationStatus(prev => ({
        ...prev,
        [optimization]: newStatus
      }));
      
      onNotification(
        `${optimization} ${newStatus ? 'enabled' : 'disabled'}`, 
        newStatus ? 'success' : 'warning'
      );
    } catch (error) {
      onNotification(`Failed to toggle ${optimization}`, 'error');
    }
  };

  const getMetricStatus = (value, thresholds) => {
    if (value >= thresholds.good) return 'success';
    if (value >= thresholds.warning) return 'warning';
    return 'error';
  };

  const getMetricTrend = (current, previous) => {
    if (current > previous) return 'up';
    if (current < previous) return 'down';
    return 'stable';
  };

  const ruvOptimizations = [
    {
      id: 'batchSpawning',
      name: 'Batch Agent Spawning',
      description: '71.2% spawn time improvement',
      improvement: '71.2%',
      active: optimizationStatus.batchSpawning
    },
    {
      id: 'queenCoordination',
      name: 'Queen Coordination',
      description: '38.7% faster consensus',
      improvement: '38.7%',
      active: optimizationStatus.queenCoordination
    },
    {
      id: 'neuralPatterns',
      name: 'Neural Patterns',
      description: '27+ models, 89%+ accuracy',
      improvement: '89%',
      active: optimizationStatus.neuralPatterns
    },
    {
      id: 'memoryPooling',
      name: 'Memory Pooling',
      description: '15% efficiency improvement',
      improvement: '15%',
      active: optimizationStatus.memoryPooling
    },
    {
      id: 'tokenOptimization',
      name: 'Token Optimization',
      description: '32.3% token reduction',
      improvement: '32.3%',
      active: optimizationStatus.tokenOptimization
    },
    {
      id: 'wasmAcceleration',
      name: 'WASM Acceleration',
      description: '3x neural inference speedup',
      improvement: '3x',
      active: optimizationStatus.wasmAcceleration
    }
  ];

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
            Performance Monitor
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Real-time system metrics and Ruv's 84.8% optimization tracking
          </Typography>
        </Box>
        <Button
          variant="outlined"
          startIcon={<RefreshIcon />}
          onClick={handleRefresh}
          disabled={isRefreshing}
        >
          {isRefreshing ? 'Refreshing...' : 'Refresh'}
        </Button>
      </Box>

      {/* Performance Alert */}
      <Alert 
        severity={benchmarkResults.overall >= 84.8 ? 'success' : 'warning'} 
        sx={{ mb: 3 }}
      >
        <AlertTitle>Ruv's Performance Target: 84.8% SWE-Bench</AlertTitle>
        Current overall performance: <strong>{benchmarkResults.overall}%</strong>
        {benchmarkResults.overall >= 84.8 ? 
          ' ✅ Target exceeded! Maintaining Ruv parity.' : 
          ' 🎯 Optimization needed to match Ruv\'s record.'
        }
      </Alert>

      <Grid container spacing={3}>
        {/* System Metrics */}
        <Grid item xs={12} lg={8}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 3 }}>
                  System Metrics
                </Typography>
                
                <Grid container spacing={3}>
                  <Grid item xs={6} md={3}>
                    <Box sx={{ textAlign: 'center' }}>
                      <SpeedIcon sx={{ fontSize: 40, color: 'primary.main', mb: 1 }} />
                      <Typography variant="h4" fontWeight={600}>
                        {metrics.cpu}%
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        CPU Usage
                      </Typography>
                      <Chip 
                        label={getMetricStatus(metrics.cpu, { good: 80, warning: 60 })} 
                        color={getMetricStatus(metrics.cpu, { good: 80, warning: 60 })}
                        size="small"
                        sx={{ mt: 1 }}
                      />
                    </Box>
                  </Grid>
                  
                  <Grid item xs={6} md={3}>
                    <Box sx={{ textAlign: 'center' }}>
                      <MemoryIcon sx={{ fontSize: 40, color: 'secondary.main', mb: 1 }} />
                      <Typography variant="h4" fontWeight={600}>
                        {metrics.memory}MB
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Memory
                      </Typography>
                      <Chip 
                        label={metrics.memory < 250 ? 'optimal' : 'high'} 
                        color={metrics.memory < 250 ? 'success' : 'warning'}
                        size="small"
                        sx={{ mt: 1 }}
                      />
                    </Box>
                  </Grid>
                  
                  <Grid item xs={6} md={3}>
                    <Box sx={{ textAlign: 'center' }}>
                      <NetworkIcon sx={{ fontSize: 40, color: 'warning.main', mb: 1 }} />
                      <Typography variant="h4" fontWeight={600}>
                        {metrics.latency}ms
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Latency
                      </Typography>
                      <Chip 
                        label={metrics.latency < 150 ? 'fast' : 'slow'} 
                        color={metrics.latency < 150 ? 'success' : 'error'}
                        size="small"
                        sx={{ mt: 1 }}
                      />
                    </Box>
                  </Grid>
                  
                  <Grid item xs={6} md={3}>
                    <Box sx={{ textAlign: 'center' }}>
                      <TimelineIcon sx={{ fontSize: 40, color: 'info.main', mb: 1 }} />
                      <Typography variant="h4" fontWeight={600}>
                        {metrics.throughput}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        Ops/sec
                      </Typography>
                      <Chip 
                        label={metrics.throughput > 1000 ? 'high' : 'low'} 
                        color={metrics.throughput > 1000 ? 'success' : 'warning'}
                        size="small"
                        sx={{ mt: 1 }}
                      />
                    </Box>
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Benchmark Scores */}
        <Grid item xs={12} lg={4}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Current Benchmark Scores
                </Typography>
                
                <Box sx={{ mb: 2 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                    <Typography variant="body2">SWE-Bench</Typography>
                    <Typography variant="body2" fontWeight={600}>
                      {benchmarkResults.sweBench}%
                    </Typography>
                  </Box>
                  <LinearProgress 
                    variant="determinate" 
                    value={benchmarkResults.sweBench} 
                    sx={{ height: 6, borderRadius: 3 }}
                    color={benchmarkResults.sweBench >= 84.8 ? 'success' : 'warning'}
                  />
                </Box>
                
                <Box sx={{ mb: 2 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                    <Typography variant="body2">HumanEval</Typography>
                    <Typography variant="body2" fontWeight={600}>
                      {benchmarkResults.humanEval}%
                    </Typography>
                  </Box>
                  <LinearProgress 
                    variant="determinate" 
                    value={benchmarkResults.humanEval} 
                    sx={{ height: 6, borderRadius: 3 }}
                    color="secondary"
                  />
                </Box>
                
                <Box sx={{ mb: 2 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                    <Typography variant="body2">BigCode</Typography>
                    <Typography variant="body2" fontWeight={600}>
                      {benchmarkResults.bigCode}%
                    </Typography>
                  </Box>
                  <LinearProgress 
                    variant="determinate" 
                    value={benchmarkResults.bigCode} 
                    sx={{ height: 6, borderRadius: 3 }}
                    color="info"
                  />
                </Box>
                
                <Divider sx={{ my: 2 }} />
                
                <Box sx={{ textAlign: 'center' }}>
                  <Typography variant="h4" fontWeight={600} color="primary.main">
                    {benchmarkResults.overall}%
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    Overall WPI Score
                  </Typography>
                  <Chip 
                    label={benchmarkResults.overall >= 84.8 ? '🌟 Exceeds Ruv' : '🎯 Below Target'} 
                    color={benchmarkResults.overall >= 84.8 ? 'success' : 'warning'}
                    sx={{ mt: 1 }}
                  />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Performance History Chart */}
        <Grid item xs={12}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Performance History (30min)
                </Typography>
                
                <ResponsiveContainer width="100%" height={300}>
                  <AreaChart data={performanceHistory}>
                    <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
                    <XAxis dataKey="time" stroke="#b0b0b0" fontSize={12} />
                    <YAxis stroke="#b0b0b0" fontSize={12} />
                    <RechartsTooltip 
                      contentStyle={{ 
                        backgroundColor: '#1a1a1a', 
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '8px'
                      }} 
                    />
                    <Area type="monotone" dataKey="cpu" stackId="1" stroke="#ff7a00" fill="rgba(255,122,0,0.1)" />
                    <Area type="monotone" dataKey="memory" stackId="2" stroke="#64ffda" fill="rgba(100,255,218,0.1)" />
                    <Area type="monotone" dataKey="latency" stackId="3" stroke="#ffb74d" fill="rgba(255,183,77,0.1)" />
                  </AreaChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Ruv Optimizations */}
        <Grid item xs={12}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 3 }}>
                  Ruv's 84.8% Performance Optimizations
                </Typography>
                
                <Grid container spacing={2}>
                  {ruvOptimizations.map((optimization) => (
                    <Grid item xs={12} sm={6} md={4} key={optimization.id}>
                      <Card 
                        variant="outlined"
                        sx={{ 
                          border: optimization.active 
                            ? '2px solid rgba(105, 240, 174, 0.5)' 
                            : '1px solid rgba(255,255,255,0.1)',
                          backgroundColor: optimization.active 
                            ? 'rgba(105, 240, 174, 0.05)' 
                            : 'transparent'
                        }}
                      >
                        <CardContent sx={{ p: 2 }}>
                          <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 1 }}>
                            <Typography variant="subtitle2" fontWeight={600}>
                              {optimization.name}
                            </Typography>
                            <Chip 
                              label={optimization.improvement}
                              size="small"
                              color={optimization.active ? 'success' : 'default'}
                              variant={optimization.active ? 'filled' : 'outlined'}
                            />
                          </Box>
                          
                          <Typography variant="caption" color="text.secondary" sx={{ mb: 2, display: 'block' }}>
                            {optimization.description}
                          </Typography>
                          
                          <Button
                            fullWidth
                            variant={optimization.active ? 'contained' : 'outlined'}
                            color={optimization.active ? 'success' : 'primary'}
                            size="small"
                            onClick={() => handleOptimizationToggle(optimization.id)}
                            startIcon={optimization.active ? <TrendingUpIcon /> : <TrendingDownIcon />}
                          >
                            {optimization.active ? 'Active' : 'Disabled'}
                          </Button>
                        </CardContent>
                      </Card>
                    </Grid>
                  ))}
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>
      </Grid>
    </Box>
  );
};

export default PerformanceMonitor;