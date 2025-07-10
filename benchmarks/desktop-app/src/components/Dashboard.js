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
  AlertTitle
} from '@mui/material';
import {
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  Refresh as RefreshIcon,
  TrendingUp as TrendingUpIcon,
  Speed as SpeedIcon,
  Psychology as PsychologyIcon,
  Hub as HubIcon,
  Assessment as AssessmentIcon
} from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip as RechartsTooltip, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts';
import { motion } from 'framer-motion';

const Dashboard = ({ onNotification, systemStatus }) => {
  const [metrics, setMetrics] = useState({
    sweBenchScore: 84.8,
    humanEvalScore: 92.3,
    bigCodeScore: 87.1,
    overallWPI: 88.4,
    activeAgents: 15,
    completedTasks: 147,
    totalTasks: 165,
    avgResponseTime: 152,
    memoryUsage: 192,
    neuralAccuracy: 89.3
  });

  const [recentActivity, setRecentActivity] = useState([]);
  const [performanceHistory, setPerformanceHistory] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    loadDashboardData();
    const interval = setInterval(loadDashboardData, 5000); // Update every 5 seconds
    return () => clearInterval(interval);
  }, []);

  const loadDashboardData = async () => {
    try {
      const [metricsData, qualityData, perfData] = await Promise.all([
        window.electronAPI.getPerformanceMetrics(),
        window.electronAPI.getQualityMetrics(),
        window.electronAPI.getSwarmStatus()
      ]);

      if (metricsData) {
        setMetrics(prev => ({ ...prev, ...metricsData }));
      }

      // Generate mock performance history for demo
      const now = Date.now();
      const history = Array.from({ length: 24 }, (_, i) => ({
        time: new Date(now - (23 - i) * 3600000).toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' }),
        wpi: 84.8 + Math.random() * 8 - 4,
        sweBench: 84.8 + Math.random() * 6 - 3,
        humanEval: 92.3 + Math.random() * 4 - 2,
        bigCode: 87.1 + Math.random() * 5 - 2.5
      }));
      setPerformanceHistory(history);

      // Mock recent activity
      setRecentActivity([
        { time: '2 minutes ago', event: 'SWE-Bench Verified completed', status: 'success', score: 86.2 },
        { time: '5 minutes ago', event: 'Neural patterns updated', status: 'info', accuracy: 91.4 },
        { time: '8 minutes ago', event: 'Swarm optimization completed', status: 'success', improvement: '12%' },
        { time: '15 minutes ago', event: 'HumanEval Plus benchmark started', status: 'running', progress: 75 },
        { time: '23 minutes ago', event: 'Agent coordination improved', status: 'success', latency: '142ms' }
      ]);

    } catch (error) {
      console.error('Failed to load dashboard data:', error);
      onNotification('Failed to load dashboard data', 'error');
    }
  };

  const handleQuickAction = async (action) => {
    setIsLoading(true);
    try {
      switch (action) {
        case 'run-swe-bench':
          onNotification('Starting SWE-Bench suite...', 'info');
          await window.electronAPI.runBenchmark('swe_bench', { variants: ['verified', 'lite'] });
          break;
        case 'optimize-swarm':
          onNotification('Optimizing swarm performance...', 'info');
          await window.electronAPI.optimizeSwarm();
          break;
        case 'train-neural':
          onNotification('Training neural patterns...', 'info');
          await window.electronAPI.trainNeuralPatterns({ epochs: 10 });
          break;
        case 'refresh-metrics':
          onNotification('Refreshing metrics...', 'info');
          await loadDashboardData();
          break;
        default:
          console.log('Unknown action:', action);
      }
    } catch (error) {
      console.error('Action failed:', error);
      onNotification(`Action failed: ${error.message}`, 'error');
    } finally {
      setIsLoading(false);
    }
  };

  const getScoreColor = (score) => {
    if (score >= 90) return 'success';
    if (score >= 80) return 'warning';
    if (score >= 70) return 'info';
    return 'error';
  };

  const getScoreGrade = (score) => {
    if (score >= 90) return '🌟 EXCEPTIONAL';
    if (score >= 80) return '🥇 EXCELLENT';
    if (score >= 70) return '🥈 GOOD';
    if (score >= 60) return '🥉 FAIR';
    return '❌ POOR';
  };

  const benchmarkData = [
    { name: 'SWE-Bench', score: metrics.sweBenchScore, target: 85, color: '#ff7a00' },
    { name: 'HumanEval', score: metrics.humanEvalScore, target: 90, color: '#64ffda' },
    { name: 'BigCode', score: metrics.bigCodeScore, target: 85, color: '#ffb74d' }
  ];

  const RADIAN = Math.PI / 180;
  const renderCustomizedLabel = ({ cx, cy, midAngle, innerRadius, outerRadius, percent }) => {
    const radius = innerRadius + (outerRadius - innerRadius) * 0.5;
    const x = cx + radius * Math.cos(-midAngle * RADIAN);
    const y = cy + radius * Math.sin(-midAngle * RADIAN);

    return (
      <text x={x} y={y} fill="white" textAnchor={x > cx ? 'start' : 'end'} dominantBaseline="central">
        {`${(percent * 100).toFixed(0)}%`}
      </text>
    );
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ mb: 4 }}>
        <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
          Dashboard
        </Typography>
        <Typography variant="body1" color="text.secondary">
          Real-time performance monitoring and quick actions
        </Typography>
      </Box>

      {/* Alert for Ruv's benchmark score */}
      <Alert severity="info" sx={{ mb: 3 }}>
        <AlertTitle>Performance Target</AlertTitle>
        Maintaining Ruv's record: <strong>84.8% SWE-Bench score</strong> - Current: <strong>{metrics.sweBenchScore}%</strong>
        {metrics.sweBenchScore >= 84.8 ? ' ✅ Target achieved!' : ' 🎯 Optimization needed'}
      </Alert>

      <Grid container spacing={3}>
        {/* Overall Performance Card */}
        <Grid item xs={12} md={6} lg={4}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card sx={{ height: '100%', background: 'linear-gradient(135deg, #ff7a00 0%, #e65100 100%)' }}>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <TrendingUpIcon sx={{ mr: 1, color: 'white' }} />
                  <Typography variant="h6" sx={{ color: 'white' }}>
                    Overall WPI Score
                  </Typography>
                </Box>
                <Typography variant="h2" sx={{ color: 'white', fontWeight: 700 }}>
                  {metrics.overallWPI.toFixed(1)}
                </Typography>
                <Typography variant="body2" sx={{ color: 'rgba(255,255,255,0.8)', mb: 1 }}>
                  {getScoreGrade(metrics.overallWPI)}
                </Typography>
                <Chip 
                  label={`+${(metrics.overallWPI - 84.8).toFixed(1)} vs Ruv's record`}
                  size="small"
                  sx={{ 
                    backgroundColor: 'rgba(255,255,255,0.2)', 
                    color: 'white',
                    fontWeight: 500
                  }}
                />
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Quick Actions */}
        <Grid item xs={12} md={6} lg={8}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <Card sx={{ height: '100%' }}>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Quick Actions
                </Typography>
                <Grid container spacing={2}>
                  <Grid item xs={6} sm={3}>
                    <Button
                      fullWidth
                      variant="outlined"
                      startIcon={<SpeedIcon />}
                      onClick={() => handleQuickAction('run-swe-bench')}
                      disabled={isLoading}
                      sx={{ p: 1.5 }}
                    >
                      Run SWE-Bench
                    </Button>
                  </Grid>
                  <Grid item xs={6} sm={3}>
                    <Button
                      fullWidth
                      variant="outlined"
                      startIcon={<HubIcon />}
                      onClick={() => handleQuickAction('optimize-swarm')}
                      disabled={isLoading}
                      sx={{ p: 1.5 }}
                    >
                      Optimize Swarm
                    </Button>
                  </Grid>
                  <Grid item xs={6} sm={3}>
                    <Button
                      fullWidth
                      variant="outlined"
                      startIcon={<PsychologyIcon />}
                      onClick={() => handleQuickAction('train-neural')}
                      disabled={isLoading}
                      sx={{ p: 1.5 }}
                    >
                      Train Neural
                    </Button>
                  </Grid>
                  <Grid item xs={6} sm={3}>
                    <Button
                      fullWidth
                      variant="outlined"
                      startIcon={<RefreshIcon />}
                      onClick={() => handleQuickAction('refresh-metrics')}
                      disabled={isLoading}
                      sx={{ p: 1.5 }}
                    >
                      Refresh
                    </Button>
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Benchmark Scores */}
        <Grid item xs={12} lg={8}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 3 }}>
                  Benchmark Performance
                </Typography>
                {benchmarkData.map((benchmark, index) => (
                  <Box key={benchmark.name} sx={{ mb: 2 }}>
                    <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                      <Typography variant="body2">{benchmark.name}</Typography>
                      <Typography variant="body2" fontWeight={600}>
                        {benchmark.score.toFixed(1)}% / {benchmark.target}%
                      </Typography>
                    </Box>
                    <LinearProgress
                      variant="determinate"
                      value={(benchmark.score / 100) * 100}
                      sx={{
                        height: 8,
                        borderRadius: 4,
                        backgroundColor: 'rgba(255,255,255,0.1)',
                        '& .MuiLinearProgress-bar': {
                          backgroundColor: benchmark.color,
                          borderRadius: 4,
                        }
                      }}
                    />
                  </Box>
                ))}
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* System Status */}
        <Grid item xs={12} lg={4}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  System Status
                </Typography>
                <Box sx={{ space: 2 }}>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', py: 1 }}>
                    <Typography variant="body2">Active Agents</Typography>
                    <Chip 
                      label={metrics.activeAgents} 
                      color={metrics.activeAgents > 10 ? 'success' : 'warning'} 
                      size="small" 
                    />
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', py: 1 }}>
                    <Typography variant="body2">Response Time</Typography>
                    <Chip 
                      label={`${metrics.avgResponseTime}ms`} 
                      color={metrics.avgResponseTime < 200 ? 'success' : 'warning'} 
                      size="small" 
                    />
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', py: 1 }}>
                    <Typography variant="body2">Memory Usage</Typography>
                    <Chip 
                      label={`${metrics.memoryUsage}MB`} 
                      color={metrics.memoryUsage < 250 ? 'success' : 'error'} 
                      size="small" 
                    />
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', py: 1 }}>
                    <Typography variant="body2">Neural Accuracy</Typography>
                    <Chip 
                      label={`${metrics.neuralAccuracy}%`} 
                      color={metrics.neuralAccuracy > 85 ? 'success' : 'warning'} 
                      size="small" 
                    />
                  </Box>
                  <Box sx={{ display: 'flex', justifyContent: 'space-between', py: 1 }}>
                    <Typography variant="body2">Task Progress</Typography>
                    <Chip 
                      label={`${metrics.completedTasks}/${metrics.totalTasks}`} 
                      color="info" 
                      size="small" 
                    />
                  </Box>
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Performance History Chart */}
        <Grid item xs={12} lg={8}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Performance History (24h)
                </Typography>
                <ResponsiveContainer width="100%" height={300}>
                  <LineChart data={performanceHistory}>
                    <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
                    <XAxis dataKey="time" stroke="#b0b0b0" fontSize={12} />
                    <YAxis domain={[70, 100]} stroke="#b0b0b0" fontSize={12} />
                    <RechartsTooltip 
                      contentStyle={{ 
                        backgroundColor: '#1a1a1a', 
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '8px'
                      }} 
                    />
                    <Line type="monotone" dataKey="wpi" stroke="#ff7a00" strokeWidth={3} dot={false} name="WPI Score" />
                    <Line type="monotone" dataKey="sweBench" stroke="#64ffda" strokeWidth={2} dot={false} name="SWE-Bench" />
                    <Line type="monotone" dataKey="humanEval" stroke="#ffb74d" strokeWidth={2} dot={false} name="HumanEval" />
                  </LineChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Recent Activity */}
        <Grid item xs={12} lg={4}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.6 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Recent Activity
                </Typography>
                <Box sx={{ maxHeight: 300, overflow: 'auto' }}>
                  {recentActivity.map((activity, index) => (
                    <Box key={index} sx={{ py: 1, borderBottom: index < recentActivity.length - 1 ? '1px solid rgba(255,255,255,0.1)' : 'none' }}>
                      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                        <Typography variant="body2" sx={{ fontWeight: 500 }}>
                          {activity.event}
                        </Typography>
                        <Chip 
                          label={activity.status} 
                          size="small" 
                          color={
                            activity.status === 'success' ? 'success' : 
                            activity.status === 'running' ? 'warning' : 'info'
                          }
                        />
                      </Box>
                      <Typography variant="caption" color="text.secondary">
                        {activity.time}
                      </Typography>
                      {activity.score && (
                        <Typography variant="caption" sx={{ ml: 2, color: 'success.main' }}>
                          Score: {activity.score}%
                        </Typography>
                      )}
                    </Box>
                  ))}
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>
      </Grid>
    </Box>
  );
};

export default Dashboard;