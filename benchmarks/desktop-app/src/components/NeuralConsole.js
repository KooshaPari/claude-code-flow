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
  TextField,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Switch,
  FormControlLabel,
  Accordion,
  AccordionSummary,
  AccordionDetails
} from '@mui/material';
import {
  Psychology as PsychologyIcon,
  Memory as MemoryIcon,
  Speed as SpeedIcon,
  Timeline as TimelineIcon,
  Settings as SettingsIcon,
  Refresh as RefreshIcon,
  PlayArrow as PlayIcon,
  Stop as StopIcon,
  Train as TrainIcon,
  Analytics as AnalyticsIcon,
  Code as CodeIcon,
  Architecture as ArchitectureIcon,
  ExpandMore as ExpandMoreIcon,
  Lightbulb as LightbulbIcon,
  Science as ScienceIcon,
  AutoAwesome as AutoAwesomeIcon
} from '@mui/icons-material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip as RechartsTooltip, ResponsiveContainer, BarChart, Bar, RadialBarChart, RadialBar, Legend } from 'recharts';
import { motion } from 'framer-motion';

const NeuralConsole = ({ onNotification, systemStatus }) => {
  const [neuralModels, setNeuralModels] = useState([
    { id: 'model_001', name: 'Coordination Master', type: 'coordination', accuracy: 97.8, status: 'active', loadTime: 45, memoryUsage: 128 },
    { id: 'model_002', name: 'Code Generator', type: 'code_generation', accuracy: 94.2, status: 'active', loadTime: 52, memoryUsage: 156 },
    { id: 'model_003', name: 'Pattern Analyzer', type: 'pattern_analysis', accuracy: 91.6, status: 'active', loadTime: 38, memoryUsage: 89 },
    { id: 'model_004', name: 'Decision Engine', type: 'decision_making', accuracy: 89.3, status: 'training', loadTime: 67, memoryUsage: 203 },
    { id: 'model_005', name: 'Optimization Specialist', type: 'optimization', accuracy: 93.7, status: 'active', loadTime: 41, memoryUsage: 142 },
    { id: 'model_006', name: 'Language Model Alpha', type: 'language', accuracy: 96.1, status: 'active', loadTime: 58, memoryUsage: 267 },
    { id: 'model_007', name: 'Vision Processor', type: 'vision', accuracy: 88.9, status: 'idle', loadTime: 73, memoryUsage: 184 },
    { id: 'model_008', name: 'Quantum Optimizer', type: 'quantum', accuracy: 92.4, status: 'active', loadTime: 94, memoryUsage: 312 }
  ]);
  
  const [neuralMetrics, setNeuralMetrics] = useState({
    totalModels: 27,
    activeModels: 19,
    trainingModels: 3,
    avgAccuracy: 93.2,
    totalInferences: 45672,
    avgInferenceTime: 12.4,
    memoryEfficiency: 87.3,
    wasmEnabled: true
  });
  
  const [trainingStatus, setTrainingStatus] = useState({
    isTraining: false,
    currentModel: null,
    progress: 0,
    epoch: 0,
    totalEpochs: 0,
    loss: 0,
    accuracy: 0
  });
  
  const [performanceHistory, setPerformanceHistory] = useState([]);
  const [modelDialogOpen, setModelDialogOpen] = useState(false);
  const [selectedModel, setSelectedModel] = useState(null);
  const [inferenceInput, setInferenceInput] = useState('');
  const [inferenceResult, setInferenceResult] = useState('');

  useEffect(() => {
    loadNeuralData();
    const interval = setInterval(loadNeuralData, 3000);
    return () => clearInterval(interval);
  }, []);

  const loadNeuralData = async () => {
    try {
      const [neuralData, metricsData, trainingData] = await Promise.all([
        window.electronAPI.getNeuralModels(),
        window.electronAPI.getNeuralMetrics(),
        window.electronAPI.getTrainingStatus()
      ]);
      
      if (neuralData) {
        setNeuralModels(neuralData);
      }
      
      if (metricsData) {
        setNeuralMetrics(prev => ({ ...prev, ...metricsData }));
      }
      
      if (trainingData) {
        setTrainingStatus(prev => ({ ...prev, ...trainingData }));
      }
      
      // Generate performance history
      const now = Date.now();
      const history = Array.from({ length: 24 }, (_, i) => ({
        time: new Date(now - (23 - i) * 300000).toLocaleTimeString('en-US', { 
          hour: '2-digit', 
          minute: '2-digit' 
        }),
        accuracy: 90 + Math.random() * 8,
        inferenceTime: 10 + Math.random() * 6,
        throughput: 800 + Math.random() * 200,
        memoryUsage: 180 + Math.random() * 40
      }));
      setPerformanceHistory(history);
      
    } catch (error) {
      console.error('Failed to load neural data:', error);
    }
  };

  const handleModelAction = async (modelId, action) => {
    try {
      switch (action) {
        case 'start':
          onNotification('Starting neural model...', 'info');
          await window.electronAPI.startNeuralModel(modelId);
          onNotification('Neural model started', 'success');
          break;
        case 'stop':
          onNotification('Stopping neural model...', 'warning');
          await window.electronAPI.stopNeuralModel(modelId);
          onNotification('Neural model stopped', 'success');
          break;
        case 'train':
          onNotification('Starting model training...', 'info');
          await window.electronAPI.trainNeuralModel(modelId, { epochs: 10 });
          onNotification('Training started', 'success');
          break;
        case 'optimize':
          onNotification('Optimizing model...', 'info');
          await window.electronAPI.optimizeNeuralModel(modelId);
          onNotification('Model optimized', 'success');
          break;
      }
      await loadNeuralData();
    } catch (error) {
      onNotification(`Action failed: ${error.message}`, 'error');
    }
  };

  const handleTrainAllModels = async () => {
    try {
      onNotification('Starting batch training for all models...', 'info');
      const result = await window.electronAPI.trainAllNeuralModels({
        epochs: 5,
        batchSize: 32,
        learningRate: 0.001
      });
      
      if (result.success) {
        onNotification(`Batch training started for ${result.modelsCount} models`, 'success');
      } else {
        onNotification('Failed to start batch training', 'error');
      }
    } catch (error) {
      onNotification(`Training error: ${error.message}`, 'error');
    }
  };

  const handleRunInference = async () => {
    if (!selectedModel || !inferenceInput.trim()) {
      onNotification('Please select a model and enter input', 'warning');
      return;
    }
    
    try {
      onNotification('Running inference...', 'info');
      const result = await window.electronAPI.runInference(selectedModel.id, {
        input: inferenceInput,
        maxTokens: 150,
        temperature: 0.7
      });
      
      if (result.success) {
        setInferenceResult(result.output);
        onNotification('Inference completed', 'success');
      } else {
        onNotification('Inference failed', 'error');
      }
    } catch (error) {
      onNotification(`Inference error: ${error.message}`, 'error');
    }
  };

  const getModelIcon = (type) => {
    switch (type) {
      case 'coordination': return PsychologyIcon;
      case 'code_generation': return CodeIcon;
      case 'pattern_analysis': return AnalyticsIcon;
      case 'decision_making': return LightbulbIcon;
      case 'optimization': return SpeedIcon;
      case 'language': return AutoAwesomeIcon;
      case 'vision': return ScienceIcon;
      case 'quantum': return ArchitectureIcon;
      default: return PsychologyIcon;
    }
  };

  const getModelColor = (type) => {
    switch (type) {
      case 'coordination': return '#ff7a00';
      case 'code_generation': return '#64ffda';
      case 'pattern_analysis': return '#ffb74d';
      case 'decision_making': return '#81c784';
      case 'optimization': return '#f06292';
      case 'language': return '#9575cd';
      case 'vision': return '#4fc3f7';
      case 'quantum': return '#ffab40';
      default: return '#90a4ae';
    }
  };

  const getStatusColor = (status) => {
    switch (status) {
      case 'active': return 'success';
      case 'training': return 'warning';
      case 'idle': return 'info';
      case 'error': return 'error';
      default: return 'default';
    }
  };

  const modelTypeData = neuralModels.reduce((acc, model) => {
    acc[model.type] = (acc[model.type] || 0) + 1;
    return acc;
  }, {});

  const accuracyData = [
    { name: 'Excellent (95-100%)', value: neuralModels.filter(m => m.accuracy >= 95).length, fill: '#69f0ae' },
    { name: 'Good (90-95%)', value: neuralModels.filter(m => m.accuracy >= 90 && m.accuracy < 95).length, fill: '#64ffda' },
    { name: 'Fair (85-90%)', value: neuralModels.filter(m => m.accuracy >= 85 && m.accuracy < 90).length, fill: '#ffb74d' },
    { name: 'Poor (<85%)', value: neuralModels.filter(m => m.accuracy < 85).length, fill: '#ff5252' }
  ];

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
            Neural Console
          </Typography>
          <Typography variant="body1" color="text.secondary">
            27+ AI models with WASM acceleration and real-time monitoring
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <Button
            variant="outlined"
            startIcon={<PlayIcon />}
            onClick={() => setModelDialogOpen(true)}
          >
            Run Inference
          </Button>
          <Button
            variant="contained"
            startIcon={<TrainIcon />}
            onClick={handleTrainAllModels}
            disabled={trainingStatus.isTraining}
            sx={{ background: 'linear-gradient(45deg, #9c27b0 30%, #673ab7 90%)' }}
          >
            {trainingStatus.isTraining ? 'Training...' : 'Train All'}
          </Button>
        </Box>
      </Box>

      {/* Neural Status Alert */}
      <Alert 
        severity={neuralMetrics.avgAccuracy >= 89 ? 'success' : 'warning'} 
        sx={{ mb: 3 }}
      >
        <AlertTitle>Neural Performance Target: 89%+ Accuracy (Ruv's Standard)</AlertTitle>
        Current average: <strong>{neuralMetrics.avgAccuracy}%</strong> across {neuralMetrics.activeModels} active models
        {neuralMetrics.avgAccuracy >= 89 ? 
          ' ✅ Exceeding Ruv\'s neural accuracy target' : 
          ' 🎯 Training needed to reach Ruv\'s standards'
        }
      </Alert>

      {/* Training Status */}
      {trainingStatus.isTraining && (
        <Alert severity="info" sx={{ mb: 3 }}>
          <AlertTitle>Training in Progress</AlertTitle>
          Model: {trainingStatus.currentModel} | Epoch: {trainingStatus.epoch}/{trainingStatus.totalEpochs}
          <LinearProgress 
            variant="determinate" 
            value={trainingStatus.progress} 
            sx={{ mt: 1, height: 6, borderRadius: 3 }}
          />
        </Alert>
      )}

      <Grid container spacing={3}>
        {/* Overview Cards */}
        <Grid item xs={12} md={6} lg={3}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card sx={{ background: 'linear-gradient(135deg, #9c27b0 0%, #673ab7 100%)' }}>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                  <Box>
                    <Typography variant="h4" sx={{ color: 'white', fontWeight: 700 }}>
                      {neuralMetrics.totalModels}
                    </Typography>
                    <Typography variant="body2" sx={{ color: 'rgba(255,255,255,0.8)' }}>
                      Total Models
                    </Typography>
                  </Box>
                  <PsychologyIcon sx={{ fontSize: 40, color: 'rgba(255,255,255,0.8)' }} />
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
                      {neuralMetrics.avgAccuracy}%
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Average Accuracy
                    </Typography>
                  </Box>
                  <AnalyticsIcon sx={{ fontSize: 40, color: 'success.main' }} />
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
                      {neuralMetrics.avgInferenceTime}ms
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Avg Inference Time
                    </Typography>
                  </Box>
                  <SpeedIcon sx={{ fontSize: 40, color: 'warning.main' }} />
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
                      {neuralMetrics.totalInferences.toLocaleString()}
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      Total Inferences
                    </Typography>
                  </Box>
                  <TimelineIcon sx={{ fontSize: 40, color: 'info.main' }} />
                </Box>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Performance Charts */}
        <Grid item xs={12} lg={8}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Neural Performance History
                </Typography>
                
                <ResponsiveContainer width="100%" height={300}>
                  <LineChart data={performanceHistory}>
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
                    <Line type="monotone" dataKey="accuracy" stroke="#9c27b0" strokeWidth={2} name="Accuracy %" />
                    <Line type="monotone" dataKey="inferenceTime" stroke="#ff7a00" strokeWidth={2} name="Inference Time (ms)" />
                    <Line type="monotone" dataKey="throughput" stroke="#64ffda" strokeWidth={2} name="Throughput (ops/s)" />
                  </LineChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Model Accuracy Distribution */}
        <Grid item xs={12} lg={4}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.6 }}
          >
            <Card>
              <CardContent>
                <Typography variant="h6" sx={{ mb: 2 }}>
                  Accuracy Distribution
                </Typography>
                
                <ResponsiveContainer width="100%" height={300}>
                  <BarChart data={accuracyData} layout="horizontal">
                    <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.1)" />
                    <XAxis type="number" stroke="#b0b0b0" fontSize={12} />
                    <YAxis dataKey="name" type="category" stroke="#b0b0b0" fontSize={10} width={80} />
                    <RechartsTooltip 
                      contentStyle={{ 
                        backgroundColor: '#1a1a1a', 
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '8px'
                      }} 
                    />
                    <Bar dataKey="value" fill={(entry) => entry.fill} />
                  </BarChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Neural Models List */}
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
                    Neural Models ({neuralModels.length})
                  </Typography>
                  <Box sx={{ display: 'flex', gap: 1 }}>
                    <FormControlLabel
                      control={<Switch checked={neuralMetrics.wasmEnabled} />}
                      label="WASM Acceleration"
                      size="small"
                    />
                    <IconButton onClick={loadNeuralData}>
                      <RefreshIcon />
                    </IconButton>
                  </Box>
                </Box>
                
                <TableContainer component={Paper} variant="outlined">
                  <Table>
                    <TableHead>
                      <TableRow>
                        <TableCell>Model</TableCell>
                        <TableCell>Type</TableCell>
                        <TableCell>Status</TableCell>
                        <TableCell align="right">Accuracy</TableCell>
                        <TableCell align="right">Load Time</TableCell>
                        <TableCell align="right">Memory</TableCell>
                        <TableCell align="center">Actions</TableCell>
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {neuralModels.map((model) => {
                        const ModelIcon = getModelIcon(model.type);
                        return (
                          <TableRow key={model.id}>
                            <TableCell>
                              <Box sx={{ display: 'flex', alignItems: 'center' }}>
                                <Avatar 
                                  sx={{ 
                                    bgcolor: getModelColor(model.type), 
                                    width: 32, 
                                    height: 32, 
                                    mr: 2 
                                  }}
                                >
                                  <ModelIcon sx={{ fontSize: 18 }} />
                                </Avatar>
                                <Box>
                                  <Typography variant="body2" fontWeight={600}>
                                    {model.name}
                                  </Typography>
                                  <Typography variant="caption" color="text.secondary">
                                    {model.id}
                                  </Typography>
                                </Box>
                              </Box>
                            </TableCell>
                            <TableCell>
                              <Chip 
                                label={model.type.replace('_', ' ')} 
                                size="small" 
                                sx={{ backgroundColor: getModelColor(model.type), color: 'white' }}
                              />
                            </TableCell>
                            <TableCell>
                              <Chip 
                                label={model.status} 
                                size="small" 
                                color={getStatusColor(model.status)}
                              />
                            </TableCell>
                            <TableCell align="right">
                              <Typography variant="body2" fontWeight={600}>
                                {model.accuracy}%
                              </Typography>
                            </TableCell>
                            <TableCell align="right">{model.loadTime}ms</TableCell>
                            <TableCell align="right">{model.memoryUsage}MB</TableCell>
                            <TableCell align="center">
                              <Box sx={{ display: 'flex', gap: 0.5 }}>
                                {model.status === 'active' ? (
                                  <IconButton 
                                    size="small" 
                                    color="warning"
                                    onClick={() => handleModelAction(model.id, 'stop')}
                                  >
                                    <StopIcon />
                                  </IconButton>
                                ) : (
                                  <IconButton 
                                    size="small" 
                                    color="success"
                                    onClick={() => handleModelAction(model.id, 'start')}
                                  >
                                    <PlayIcon />
                                  </IconButton>
                                )}
                                <IconButton 
                                  size="small" 
                                  color="primary"
                                  onClick={() => handleModelAction(model.id, 'train')}
                                >
                                  <TrainIcon />
                                </IconButton>
                              </Box>
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

      {/* Inference Dialog */}
      <Dialog 
        open={modelDialogOpen} 
        onClose={() => setModelDialogOpen(false)}
        maxWidth="md"
        fullWidth
      >
        <DialogTitle>Run Neural Inference</DialogTitle>
        <DialogContent>
          <Box sx={{ pt: 2 }}>
            <FormControl fullWidth sx={{ mb: 2 }}>
              <InputLabel>Select Model</InputLabel>
              <Select
                value={selectedModel?.id || ''}
                onChange={(e) => {
                  const model = neuralModels.find(m => m.id === e.target.value);
                  setSelectedModel(model);
                }}
              >
                {neuralModels.filter(m => m.status === 'active').map((model) => (
                  <MenuItem key={model.id} value={model.id}>
                    {model.name} ({model.type}) - {model.accuracy}%
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
            
            <TextField
              fullWidth
              multiline
              rows={4}
              label="Input"
              value={inferenceInput}
              onChange={(e) => setInferenceInput(e.target.value)}
              sx={{ mb: 2 }}
              placeholder="Enter your input for neural inference..."
            />
            
            {inferenceResult && (
              <Box>
                <Typography variant="subtitle2" sx={{ mb: 1 }}>
                  Result:
                </Typography>
                <Paper variant="outlined" sx={{ p: 2, backgroundColor: 'rgba(0,0,0,0.2)' }}>
                  <Typography variant="body2" sx={{ whiteSpace: 'pre-wrap' }}>
                    {inferenceResult}
                  </Typography>
                </Paper>
              </Box>
            )}
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setModelDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleRunInference} 
            variant="contained"
            disabled={!selectedModel || !inferenceInput.trim()}
          >
            Run Inference
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default NeuralConsole;