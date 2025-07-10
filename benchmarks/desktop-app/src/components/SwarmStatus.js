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
  Paper,
  Avatar,
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  FormControl,
  InputLabel,
  Select,
  MenuItem
} from '@mui/material';
import {
  Hub as HubIcon,
  Psychology as PsychologyIcon,
  Code as CodeIcon,
  Analytics as AnalyticsIcon,
  BugReport as BugReportIcon,
  Architecture as ArchitectureIcon,
  Add as AddIcon,
  Remove as RemoveIcon,
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  Settings as SettingsIcon,
  Refresh as RefreshIcon,
  AccountTree as TopologyIcon,
  Speed as SpeedIcon,
  Memory as MemoryIcon,
  Timeline as TimelineIcon
} from '@mui/icons-material';
import { PieChart, Pie, Cell, ResponsiveContainer, BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip as RechartsTooltip, LineChart, Line } from 'recharts';
import { motion } from 'framer-motion';

const SwarmStatus = ({ onNotification, systemStatus }) => {
  const [swarmConfig, setSwarmConfig] = useState({
    topology: 'hierarchical',
    coordination: 'queen',
    maxAgents: 15,
    neuralEnabled: true,
    autoOptimize: true
  });
  
  const [swarmMetrics, setSwarmMetrics] = useState({
    activeAgents: 12,
    totalTasks: 847,
    completedTasks: 678,
    failedTasks: 23,
    avgResponseTime: 142,
    coordinationEfficiency: 94.2,
    consensusTime: 89,
    memoryUsage: 187
  });
  
  const [agents, setAgents] = useState([
    { id: 'agent_001', type: 'coordinator', name: 'Queen Agent', status: 'active', tasks: 156, accuracy: 97.2, uptime: '2h 34m' },
    { id: 'agent_002', type: 'researcher', name: 'Research Alpha', status: 'active', tasks: 89, accuracy: 91.8, uptime: '2h 31m' },
    { id: 'agent_003', type: 'coder', name: 'Code Ninja', status: 'active', tasks: 124, accuracy: 89.4, uptime: '2h 29m' },
    { id: 'agent_004', type: 'analyst', name: 'Data Wizard', status: 'active', tasks: 67, accuracy: 93.1, uptime: '2h 27m' },
    { id: 'agent_005', type: 'tester', name: 'QA Master', status: 'active', tasks: 78, accuracy: 95.6, uptime: '2h 25m' },
    { id: 'agent_006', type: 'architect', name: 'System Designer', status: 'busy', tasks: 45, accuracy: 88.9, uptime: '2h 23m' },
    { id: 'agent_007', type: 'researcher', name: 'Research Beta', status: 'active', tasks: 52, accuracy: 87.3, uptime: '2h 21m' },
    { id: 'agent_008', type: 'coder', name: 'Code Samurai', status: 'idle', tasks: 34, accuracy: 92.7, uptime: '2h 19m' }
  ]);
  
  const [topologyHistory, setTopologyHistory] = useState([]);
  const [configDialogOpen, setConfigDialogOpen] = useState(false);
  const [isOptimizing, setIsOptimizing] = useState(false);

  useEffect(() => {
    loadSwarmData();
    const interval = setInterval(loadSwarmData, 2000);
    return () => clearInterval(interval);
  }, []);

  const loadSwarmData = async () => {
    try {
      const [swarmData, metricsData, agentData] = await Promise.all([
        window.electronAPI.getSwarmStatus(),
        window.electronAPI.getSwarmMetrics(),
        window.electronAPI.getAgentList()
      ]);
      
      if (swarmData) {
        setSwarmConfig(prev => ({ ...prev, ...swarmData.config }));
      }
      
      if (metricsData) {
        setSwarmMetrics(prev => ({ ...prev, ...metricsData }));
      }
      
      if (agentData) {
        setAgents(agentData);
      }
      
      // Generate topology performance history
      const now = Date.now();
      const history = Array.from({ length: 20 }, (_, i) => ({
        time: new Date(now - (19 - i) * 30000).toLocaleTimeString('en-US', { 
          hour: '2-digit', 
          minute: '2-digit', 
          second: '2-digit' 
        }),
        efficiency: 90 + Math.random() * 8,
        latency: 140 + Math.random() * 30 - 15,
        throughput: 850 + Math.random() * 100 - 50
      }));
      setTopologyHistory(history);
      
    } catch (error) {
      console.error('Failed to load swarm data:', error);
    }
  };

  const handleSpawnAgent = async (agentType) => {
    try {
      onNotification(`Spawning ${agentType} agent...`, 'info');
      const result = await window.electronAPI.spawnAgent({
        type: agentType,
        config: swarmConfig
      });
      
      if (result.success) {
        onNotification(`${agentType} agent spawned successfully`, 'success');
        await loadSwarmData();
      } else {
        onNotification(`Failed to spawn ${agentType} agent`, 'error');
      }
    } catch (error) {
      onNotification(`Error spawning agent: ${error.message}`, 'error');
    }
  };

  const handleTerminateAgent = async (agentId) => {
    try {
      onNotification('Terminating agent...', 'warning');
      const result = await window.electronAPI.terminateAgent(agentId);
      
      if (result.success) {
        onNotification('Agent terminated', 'success');
        await loadSwarmData();
      } else {
        onNotification('Failed to terminate agent', 'error');
      }
    } catch (error) {
      onNotification(`Error terminating agent: ${error.message}`, 'error');
    }
  };

  const handleOptimizeSwarm = async () => {
    setIsOptimizing(true);
    try {
      onNotification('Optimizing swarm topology...', 'info');
      const result = await window.electronAPI.optimizeSwarm({
        target: 'performance',
        strategy: 'ruver_record'
      });
      
      if (result.success) {
        onNotification(`Swarm optimized: ${result.improvement}% improvement`, 'success');
        await loadSwarmData();
      } else {
        onNotification('Swarm optimization failed', 'warning');
      }
    } catch (error) {
      onNotification(`Optimization error: ${error.message}`, 'error');
    } finally {
      setIsOptimizing(false);
    }
  };

  const handleConfigUpdate = async () => {
    try {
      const result = await window.electronAPI.updateSwarmConfig(swarmConfig);
      if (result.success) {
        onNotification('Swarm configuration updated', 'success');
        setConfigDialogOpen(false);
      } else {
        onNotification('Failed to update configuration', 'error');
      }
    } catch (error) {
      onNotification(`Configuration error: ${error.message}`, 'error');
    }
  };

  const getAgentIcon = (type) => {
    switch (type) {
      case 'coordinator': return HubIcon;
      case 'researcher': return AnalyticsIcon;
      case 'coder': return CodeIcon;
      case 'analyst': return PsychologyIcon;
      case 'tester': return BugReportIcon;
      case 'architect': return ArchitectureIcon;
      default: return HubIcon;
    }
  };

  const getAgentColor = (type) => {
    switch (type) {
      case 'coordinator': return '#ff7a00';
      case 'researcher': return '#64ffda';
      case 'coder': return '#ffb74d';
      case 'analyst': return '#81c784';
      case 'tester': return '#f06292';
      case 'architect': return '#9575cd';
      default: return '#90a4ae';
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'active': return 'success';
      case 'busy': return 'warning';
      case 'idle': return 'info';
      case 'error': return 'error';
      default: return 'default';
    }
  };

  const agentTypeData = agents.reduce((acc, agent) => {
    acc[agent.type] = (acc[agent.type] || 0) + 1;
    return acc;
  }, {});

  const pieData = Object.entries(agentTypeData).map(([type, count]) => ({
    name: type,
    value: count,
    color: getAgentColor(type)
  }));

  const COLORS = ['#ff7a00', '#64ffda', '#ffb74d', '#81c784', '#f06292', '#9575cd'];

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
            Swarm Status
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Real-time agent coordination and topology management
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
            startIcon={<SpeedIcon />}
            onClick={handleOptimizeSwarm}
            disabled={isOptimizing}
            sx={{ background: 'linear-gradient(45deg, #ff7a00 30%, #e65100 90%)' }}
          >
            {isOptimizing ? 'Optimizing...' : 'Optimize Swarm'}
          </Button>
        </Box>
      </Box>

      {/* Swarm Status Alert */}
      <Alert 
        severity={swarmMetrics.coordinationEfficiency >= 90 ? 'success' : 'warning'} 
        sx={{ mb: 3 }}
      >
        <AlertTitle>Swarm Performance</AlertTitle>
        {swarmMetrics.activeAgents} agents active with {swarmMetrics.coordinationEfficiency}% efficiency
        {swarmMetrics.coordinationEfficiency >= 90 ? 
          ' ✅ Operating at optimal performance' : 
          ' 🎯 Optimization recommended'
        }
      </Alert>

      <Grid container spacing={3}>
        {/* Overview Cards */}
        <Grid item xs={12} md={6} lg={3}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card sx={{ background: 'linear-gradient(135deg, #ff7a00 0%, #e65100 100%)' }}>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" sx={{ color: 'white', fontWeight: 700 }}>
                      {swarmMetrics.activeAgents}
                    </Typography>
                    <Typography variant="body2" sx={{ color: 'rgba(255,255,255,0.8)' }}>
                      Active Agents
                    </Typography>
                  </Box>
                  <HubIcon sx={{ fontSize: 40, color: 'rgba(255,255,255,0.8)' }} />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        <Grid item xs={12} md={6} lg={3}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" fontWeight={700}>
                      {swarmMetrics.coordinationEfficiency}%
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Coordination Efficiency
                    </Typography>
                  </Box>
                  <TopologyIcon sx={{ fontSize: 40, color: 'secondary.main' }} />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        <Grid item xs={12} md={6} lg={3}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" fontWeight={700}>
                      {swarmMetrics.completedTasks}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Tasks Completed
                    </Typography>
                  </Box>
                  <TimelineIcon sx={{ fontSize: 40, color: 'info.main' }} />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        <Grid item xs={12} md={6} lg={3}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" fontWeight={700}>
                      {swarmMetrics.avgResponseTime}ms
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Avg Response Time
                    </Typography>
                  </Box>
                  <SpeedIcon sx={{ fontSize: 40, color: 'warning.main' }} />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Agent Distribution */}
        <Grid item xs={12} md={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Agent Distribution
                </Typography>
                
                <ResponsiveContainer width="100%" height={250}>
                  <PieChart>
                    <Pie
                      data={pieData}
                      cx="50%"
                      cy="50%"
                      outerRadius={80}
                      fill="#8884d8"
                      dataKey="value"
                      label={({ name, value }) => `${name}: ${value}`}
                    >
                      {pieData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <RechartsTooltip />
                  </PieChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Performance History */}
        <Grid item xs={12} md={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.6 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Coordination Performance
                </Typography>
                
                <ResponsiveContainer width="100%" height={250}>
                  <LineChart data={topologyHistory}>
                    <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
                    <XAxis dataKey="time" stroke="#b0b0b0" fontSize={10} />
                    <YAxis stroke="#b0b0b0" fontSize={12} />
                    <RechartsTooltip 
                      contentStyle={{ 
                        backgroundColor: '#1a1a1a', 
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '8px'
                      }} 
                    />
                    <Line type="monotone" dataKey="efficiency" stroke="#ff7a00" strokeWidth={2} name="Efficiency %" />
                    <Line type="monotone" dataKey="latency" stroke="#64ffda" strokeWidth={2} name="Latency (ms)" />
                  </LineChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Agent List */}
        <Grid item xs={12}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.7 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
                  <Typography variant="h6">
                    Active Agents ({agents.length})
                  </Typography>
                  <Box sx={{ display: 'flex', gap: 1 }}>
                    <Button
                      size="small"
                      startIcon={<AddIcon />}
                      onClick={() => handleSpawnAgent('researcher')}
                    >
                      Spawn Agent
                    </Button>
                    <IconButton onClick={loadSwarmData}>
                      <RefreshIcon />
                    </IconButton>
                  </Box>
                </Box>
                
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell>Agent</TableCell>
                        <TableCell>Type</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="right">Tasks</TableCell>
                        <TableCell align="right">Accuracy</TableCell>
                        <TableCell align="right">Uptime</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {agents.map((agent) => {
                        const AgentIcon = getAgentIcon(agent.type);
                        return (
                          <TableRow key={agent.id}>
                            <TableCell>
                              <Box sx={{ display: 'flex', alignItems: 'center' }}>
                                <Avatar 
                                  sx={{ 
                                    bgcolor: getAgentColor(agent.type), 
                                    width: 32, 
                                    height: 32, 
                                    mr: 2 
                                  }}
                                >
                                  <AgentIcon sx={{ fontSize: 18 }} />
                                </Avatar>
                                <Box>
                                  <Typography variant="body2" fontWeight={600}>
                                    {agent.name}
                                  </Typography>
                                  <Typography variant="caption" color="text.secondary">
                                    {agent.id}
                                  </Typography>
                                </Box>
                              </Box>
                            </TableCell>
                            <TableCell>
                              <Chip 
                                label={agent.type} 
                                size="small" 
                                sx={{ backgroundColor: getAgentColor(agent.type), color: 'white' }}
                              />
                            </TableCell>
                            <TableCell>
                              <Chip 
                                label={agent.status} 
                                size="small" 
                                color={getStatusColor(agent.status)}
                              />
                            </TableCell>
                            <TableCell align="right">{agent.tasks}</TableCell>
                            <TableCell align="right">{agent.accuracy}%</TableCell>
                            <TableCell align="right">{agent.uptime}</TableCell>
                            <TableCell align="center">
                              <IconButton 
                                size="small" 
                                color="error"
                                onClick={() => handleTerminateAgent(agent.id)}
                              >
                                <StopIcon />
                              </IconButton>
                            </TableCell>
                          </TableRow>
                        );
                      })}
                    </TableBody>
                  </Table>
                </TableContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>
      </Grid>

      {/* Configuration Dialog */}
      <Dialog 
        open={configDialogOpen} 
        onClose={() => setConfigDialogOpen(false)}
        maxWidth="sm"
        fullWidth
      >
        <DialogTitle>Swarm Configuration</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <Grid container spacing={2}>
              <Grid item xs={12} sm={6}>
                <FormControl fullWidth>
                  <InputLabel>Topology</InputLabel>
                  <Select
                    value={swarmConfig.topology}
                    onChange={(e) => setSwarmConfig(prev => ({ ...prev, topology: e.target.value }))}
                  >
                    <MenuItem value="hierarchical">Hierarchical</MenuItem>
                    <MenuItem value="mesh">Mesh</MenuItem>
                    <MenuItem value="ring">Ring</MenuItem>
                    <MenuItem value="star">Star</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} sm={6}>
                <FormControl fullWidth>
                  <InputLabel>Coordination</InputLabel>
                  <Select
                    value={swarmConfig.coordination}
                    onChange={(e) => setSwarmConfig(prev => ({ ...prev, coordination: e.target.value }))}
                  >
                    <MenuItem value="queen">Queen (Ruv's Choice)</MenuItem>
                    <MenuItem value="consensus">Consensus</MenuItem>
                    <MenuItem value="democratic">Democratic</MenuItem>
                    <MenuItem value="distributed">Distributed</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
              
              <Grid item xs={12} sm={6}>
                <FormControl fullWidth>
                  <InputLabel>Max Agents</InputLabel>
                  <Select
                    value={swarmConfig.maxAgents}
                    onChange={(e) => setSwarmConfig(prev => ({ ...prev, maxAgents: e.target.value }))}
                  >
                    <MenuItem value={5}>5 (Minimal)</MenuItem>
                    <MenuItem value={10}>10 (Standard)</MenuItem>
                    <MenuItem value={15}>15 (Optimal)</MenuItem>
                    <MenuItem value={25}>25 (Maximum)</MenuItem>
                  </Select>
                </FormControl>
              </Grid>
            </Grid>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setConfigDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleConfigUpdate} variant="contained">
            Update Configuration
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default SwarmStatus;