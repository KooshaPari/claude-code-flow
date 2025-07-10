import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Button,
  Switch,
  FormControl,
  FormControlLabel,
  InputLabel,
  Select,
  MenuItem,
  TextField,
  Divider,
  Alert,
  AlertTitle,
  Accordion,
  AccordionSummary,
  AccordionDetails,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  ListItemSecondaryAction,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Chip,
  Slider,
  IconButton,
  Tooltip
} from '@mui/material';
import {
  Settings as SettingsIcon,
  Save as SaveIcon,
  RestoreFromTrash as RestoreIcon,
  Security as SecurityIcon,
  Speed as SpeedIcon,
  Memory as MemoryIcon,
  Notifications as NotificationsIcon,
  Palette as PaletteIcon,
  Code as CodeIcon,
  Psychology as PsychologyIcon,
  Hub as HubIcon,
  ExpandMore as ExpandMoreIcon,
  Info as InfoIcon,
  Warning as WarningIcon,
  CheckCircle as CheckCircleIcon,
  Refresh as RefreshIcon,
  Download as DownloadIcon,
  Upload as UploadIcon,
  Folder as FolderIcon
} from '@mui/icons-material';
import { motion } from 'framer-motion';

const Settings = ({ onNotification, systemStatus }) => {
  const [settings, setSettings] = useState({
    // Performance Settings
    maxParallelBenchmarks: 5,
    benchmarkTimeout: 300000,
    enableNeuralOptimization: true,
    enableSwarmCoordination: true,
    wasmAcceleration: true,
    memoryPooling: true,
    tokenOptimization: true,
    
    // UI Settings
    darkMode: true,
    animationsEnabled: true,
    autoRefreshInterval: 3000,
    showAdvancedMetrics: true,
    notificationsEnabled: true,
    soundEnabled: false,
    
    // Swarm Settings
    defaultTopology: 'hierarchical',
    defaultCoordination: 'queen',
    maxAgents: 15,
    autoSpawnAgents: true,
    agentIdleTimeout: 300000,
    
    // Benchmark Settings
    defaultBenchmarkVariants: {
      sweBench: ['verified', 'lite'],
      humanEval: ['base', 'plus'],
      bigCode: ['multipl_e', 'ds_1000']
    },
    includeMultimodal: true,
    saveResults: true,
    exportFormat: 'json',
    
    // Quality Settings
    qualityTargets: {
      codeQuality: 90.0,
      testCoverage: 95.0,
      documentation: 85.0,
      security: 95.0,
      performance: 84.8
    },
    
    // Neural Settings
    neuralModelCount: 27,
    neuralAccuracyTarget: 89.0,
    enableAutoTraining: true,
    trainingEpochs: 10,
    batchSize: 32,
    learningRate: 0.001,
    
    // Export/Import Settings
    exportPath: '',
    autoBackup: true,
    backupInterval: 86400000, // 24 hours
    retainBackups: 7
  });
  
  const [tempSettings, setTempSettings] = useState(settings);
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);
  const [resetDialogOpen, setResetDialogOpen] = useState(false);
  const [exportDialogOpen, setExportDialogOpen] = useState(false);
  const [importDialogOpen, setImportDialogOpen] = useState(false);

  useEffect(() => {
    loadSettings();
  }, []);

  useEffect(() => {
    const hasChanges = JSON.stringify(settings) !== JSON.stringify(tempSettings);
    setHasUnsavedChanges(hasChanges);
  }, [settings, tempSettings]);

  const loadSettings = async () => {
    try {
      const savedSettings = await window.electronAPI.getSettings();
      if (savedSettings) {
        setSettings(savedSettings);
        setTempSettings(savedSettings);
      }
    } catch (error) {
      console.error('Failed to load settings:', error);
      onNotification('Failed to load settings', 'error');
    }
  };

  const handleSaveSettings = async () => {
    try {
      onNotification('Saving settings...', 'info');
      const result = await window.electronAPI.saveSettings(tempSettings);
      
      if (result.success) {
        setSettings(tempSettings);
        setHasUnsavedChanges(false);
        onNotification('Settings saved successfully', 'success');
      } else {
        onNotification('Failed to save settings', 'error');
      }
    } catch (error) {
      onNotification(`Save error: ${error.message}`, 'error');
    }
  };

  const handleResetSettings = async () => {
    try {
      onNotification('Resetting to defaults...', 'info');
      const defaultSettings = await window.electronAPI.getDefaultSettings();
      
      if (defaultSettings) {
        setTempSettings(defaultSettings);
        onNotification('Settings reset to defaults', 'success');
      } else {
        onNotification('Failed to reset settings', 'error');
      }
    } catch (error) {
      onNotification(`Reset error: ${error.message}`, 'error');
    } finally {
      setResetDialogOpen(false);
    }
  };

  const handleExportSettings = async () => {
    try {
      const result = await window.electronAPI.showSaveDialog({
        title: 'Export Settings',
        defaultPath: `claude-flow-settings-${new Date().toISOString().split('T')[0]}.json`,
        filters: [
          { name: 'JSON Files', extensions: ['json'] },
          { name: 'All Files', extensions: ['*'] }
        ]
      });

      if (!result.canceled) {
        await window.electronAPI.exportSettings(result.filePath, tempSettings);
        onNotification('Settings exported successfully', 'success');
      }
    } catch (error) {
      onNotification(`Export error: ${error.message}`, 'error');
    } finally {
      setExportDialogOpen(false);
    }
  };

  const handleImportSettings = async (filePath) => {
    try {
      const importedSettings = await window.electronAPI.importSettings(filePath);
      
      if (importedSettings) {
        setTempSettings(importedSettings);
        onNotification('Settings imported successfully', 'success');
      } else {
        onNotification('Failed to import settings', 'error');
      }
    } catch (error) {
      onNotification(`Import error: ${error.message}`, 'error');
    } finally {
      setImportDialogOpen(false);
    }
  };

  const handleSelectImportFile = async () => {
    try {
      const result = await window.electronAPI.showOpenDialog({
        title: 'Import Settings',
        filters: [
          { name: 'JSON Files', extensions: ['json'] },
          { name: 'All Files', extensions: ['*'] }
        ],
        properties: ['openFile']
      });

      if (!result.canceled && result.filePaths.length > 0) {
        await handleImportSettings(result.filePaths[0]);
      }
    } catch (error) {
      onNotification(`File selection error: ${error.message}`, 'error');
    }
  };

  const updateSetting = (path, value) => {
    const keys = path.split('.');
    const newSettings = { ...tempSettings };
    let current = newSettings;
    
    for (let i = 0; i < keys.length - 1; i++) {
      if (!current[keys[i]]) current[keys[i]] = {};
      current = current[keys[i]];
    }
    
    current[keys[keys.length - 1]] = value;
    setTempSettings(newSettings);
  };

  const getSetting = (path) => {
    const keys = path.split('.');
    let current = tempSettings;
    
    for (const key of keys) {
      if (current && typeof current === 'object' && key in current) {
        current = current[key];
      } else {
        return undefined;
      }
    }
    
    return current;
  };

  return (
    <Box sx={{ flexGrow: 1 }}>
      {/* Header */}
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Box>
          <Typography variant="h3" sx={{ mb: 1, fontWeight: 600 }}>
            Settings
          </Typography>
          <Typography variant="body1" color="text.secondary">
            Configure performance, UI, swarm, and benchmark settings
          </Typography>
        </Box>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <Button
            variant="outlined"
            startIcon={<UploadIcon />}
            onClick={handleSelectImportFile}
          >
            Import
          </Button>
          <Button
            variant="outlined"
            startIcon={<DownloadIcon />}
            onClick={() => setExportDialogOpen(true)}
          >
            Export
          </Button>
          <Button
            variant="outlined"
            startIcon={<RestoreIcon />}
            onClick={() => setResetDialogOpen(true)}
            color="warning"
          >
            Reset
          </Button>
          <Button
            variant="contained"
            startIcon={<SaveIcon />}
            onClick={handleSaveSettings}
            disabled={!hasUnsavedChanges}
            sx={{ background: hasUnsavedChanges ? 'linear-gradient(45deg, #ff7a00 30%, #e65100 90%)' : undefined }}
          >
            {hasUnsavedChanges ? 'Save Changes' : 'Saved'}
          </Button>
        </Box>
      </Box>

      {/* Unsaved Changes Alert */}
      {hasUnsavedChanges && (
        <Alert severity="warning" sx={{ mb: 3 }}>
          <AlertTitle>Unsaved Changes</AlertTitle>
          You have unsaved configuration changes. Click "Save Changes" to apply them.
        </Alert>
      )}

      <Grid container spacing={3}>
        {/* Performance Settings */}
        <Grid item xs={12} lg={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.1 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <SpeedIcon sx={{ mr: 1, color: 'primary.main' }} />
                  <Typography variant="h6">Performance Settings</Typography>
                </Box>
                
                <Grid container spacing={2}>
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('enableNeuralOptimization')}
                          onChange={(e) => updateSetting('enableNeuralOptimization', e.target.checked)}
                        />
                      }
                      label="Neural Pattern Optimization"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('enableSwarmCoordination')}
                          onChange={(e) => updateSetting('enableSwarmCoordination', e.target.checked)}
                        />
                      }
                      label="Swarm Coordination (Ruv's Queen Mode)"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('wasmAcceleration')}
                          onChange={(e) => updateSetting('wasmAcceleration', e.target.checked)}
                        />
                      }
                      label="WASM Acceleration"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('memoryPooling')}
                          onChange={(e) => updateSetting('memoryPooling', e.target.checked)}
                        />
                      }
                      label="Memory Pooling"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('tokenOptimization')}
                          onChange={(e) => updateSetting('tokenOptimization', e.target.checked)}
                        />
                      }
                      label="Token Optimization (32.3% reduction)"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <Typography gutterBottom>Max Parallel Benchmarks</Typography>
                    <Slider
                      value={getSetting('maxParallelBenchmarks')}
                      onChange={(e, value) => updateSetting('maxParallelBenchmarks', value)}
                      min={1}
                      max={10}
                      marks
                      valueLabelDisplay="auto"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <TextField
                      fullWidth
                      label="Benchmark Timeout (ms)"
                      type="number"
                      value={getSetting('benchmarkTimeout')}
                      onChange={(e) => updateSetting('benchmarkTimeout', parseInt(e.target.value))}
                    />
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* UI Settings */}
        <Grid item xs={12} lg={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <PaletteIcon sx={{ mr: 1, color: 'secondary.main' }} />
                  <Typography variant="h6">UI Settings</Typography>
                </Box>
                
                <Grid container spacing={2}>
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('darkMode')}
                          onChange={(e) => updateSetting('darkMode', e.target.checked)}
                        />
                      }
                      label="Dark Mode (Claude-desktop style)"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('animationsEnabled')}
                          onChange={(e) => updateSetting('animationsEnabled', e.target.checked)}
                        />
                      }
                      label="Animations"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('showAdvancedMetrics')}
                          onChange={(e) => updateSetting('showAdvancedMetrics', e.target.checked)}
                        />
                      }
                      label="Show Advanced Metrics"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('notificationsEnabled')}
                          onChange={(e) => updateSetting('notificationsEnabled', e.target.checked)}
                        />
                      }
                      label="Notifications"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('soundEnabled')}
                          onChange={(e) => updateSetting('soundEnabled', e.target.checked)}
                        />
                      }
                      label="Sound Effects"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <Typography gutterBottom>Auto Refresh Interval (ms)</Typography>
                    <Slider
                      value={getSetting('autoRefreshInterval')}
                      onChange={(e, value) => updateSetting('autoRefreshInterval', value)}
                      min={1000}
                      max={10000}
                      step={1000}
                      marks={[
                        { value: 1000, label: '1s' },
                        { value: 5000, label: '5s' },
                        { value: 10000, label: '10s' }
                      ]}
                      valueLabelDisplay="auto"
                      valueLabelFormat={(value) => `${value/1000}s`}
                    />
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Swarm Settings */}
        <Grid item xs={12} lg={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <HubIcon sx={{ mr: 1, color: 'warning.main' }} />
                  <Typography variant="h6">Swarm Settings</Typography>
                </Box>
                
                <Grid container spacing={2}>
                  <Grid item xs={12} sm={6}>
                    <FormControl fullWidth>
                      <InputLabel>Default Topology</InputLabel>
                      <Select
                        value={getSetting('defaultTopology')}
                        onChange={(e) => updateSetting('defaultTopology', e.target.value)}
                      >
                        <MenuItem value="hierarchical">Hierarchical (Ruv's Choice)</MenuItem>
                        <MenuItem value="mesh">Mesh</MenuItem>
                        <MenuItem value="ring">Ring</MenuItem>
                        <MenuItem value="star">Star</MenuItem>
                      </Select>
                    </FormControl>
                  </Grid>
                  
                  <Grid item xs={12} sm={6}>
                    <FormControl fullWidth>
                      <InputLabel>Coordination Mode</InputLabel>
                      <Select
                        value={getSetting('defaultCoordination')}
                        onChange={(e) => updateSetting('defaultCoordination', e.target.value)}
                      >
                        <MenuItem value="queen">Queen (38.7% faster)</MenuItem>
                        <MenuItem value="consensus">Consensus</MenuItem>
                        <MenuItem value="democratic">Democratic</MenuItem>
                        <MenuItem value="distributed">Distributed</MenuItem>
                      </Select>
                    </FormControl>
                  </Grid>
                  
                  <Grid item xs={12}>
                    <Typography gutterBottom>Maximum Agents</Typography>
                    <Slider
                      value={getSetting('maxAgents')}
                      onChange={(e, value) => updateSetting('maxAgents', value)}
                      min={5}
                      max={25}
                      step={5}
                      marks={[
                        { value: 5, label: '5' },
                        { value: 15, label: '15' },
                        { value: 25, label: '25' }
                      ]}
                      valueLabelDisplay="auto"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('autoSpawnAgents')}
                          onChange={(e) => updateSetting('autoSpawnAgents', e.target.checked)}
                        />
                      }
                      label="Auto-spawn Agents"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <TextField
                      fullWidth
                      label="Agent Idle Timeout (ms)"
                      type="number"
                      value={getSetting('agentIdleTimeout')}
                      onChange={(e) => updateSetting('agentIdleTimeout', parseInt(e.target.value))}
                    />
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Neural Settings */}
        <Grid item xs={12} lg={6}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.4 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <PsychologyIcon sx={{ mr: 1, color: 'info.main' }} />
                  <Typography variant="h6">Neural Settings</Typography>
                </Box>
                
                <Grid container spacing={2}>
                  <Grid item xs={12}>
                    <Typography gutterBottom>Neural Model Count</Typography>
                    <Slider
                      value={getSetting('neuralModelCount')}
                      onChange={(e, value) => updateSetting('neuralModelCount', value)}
                      min={10}
                      max={50}
                      step={1}
                      marks={[
                        { value: 10, label: '10' },
                        { value: 27, label: '27 (Ruv)' },
                        { value: 50, label: '50' }
                      ]}
                      valueLabelDisplay="auto"
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <TextField
                      fullWidth
                      label="Neural Accuracy Target (%)"
                      type="number"
                      inputProps={{ min: 80, max: 100, step: 0.1 }}
                      value={getSetting('neuralAccuracyTarget')}
                      onChange={(e) => updateSetting('neuralAccuracyTarget', parseFloat(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12}>
                    <FormControlLabel
                      control={
                        <Switch
                          checked={getSetting('enableAutoTraining')}
                          onChange={(e) => updateSetting('enableAutoTraining', e.target.checked)}
                        />
                      }
                      label="Auto-training"
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={4}>
                    <TextField
                      fullWidth
                      label="Training Epochs"
                      type="number"
                      value={getSetting('trainingEpochs')}
                      onChange={(e) => updateSetting('trainingEpochs', parseInt(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={4}>
                    <TextField
                      fullWidth
                      label="Batch Size"
                      type="number"
                      value={getSetting('batchSize')}
                      onChange={(e) => updateSetting('batchSize', parseInt(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={4}>
                    <TextField
                      fullWidth
                      label="Learning Rate"
                      type="number"
                      inputProps={{ step: 0.0001 }}
                      value={getSetting('learningRate')}
                      onChange={(e) => updateSetting('learningRate', parseFloat(e.target.value))}
                    />
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>

        {/* Quality Targets */}
        <Grid item xs={12}>
          <motion.div
            initial={{ opacity: 0, y: 20 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.5 }}
          >
            <Card>
              <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', mb: 2 }}>
                  <CheckCircleIcon sx={{ mr: 1, color: 'success.main' }} />
                  <Typography variant="h6">Quality Targets</Typography>
                </Box>
                
                <Grid container spacing={2}>
                  <Grid item xs={12} sm={6} md={2.4}>
                    <TextField
                      fullWidth
                      label="Code Quality (%)"
                      type="number"
                      inputProps={{ min: 70, max: 100, step: 0.1 }}
                      value={getSetting('qualityTargets.codeQuality')}
                      onChange={(e) => updateSetting('qualityTargets.codeQuality', parseFloat(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={6} md={2.4}>
                    <TextField
                      fullWidth
                      label="Test Coverage (%)"
                      type="number"
                      inputProps={{ min: 70, max: 100, step: 0.1 }}
                      value={getSetting('qualityTargets.testCoverage')}
                      onChange={(e) => updateSetting('qualityTargets.testCoverage', parseFloat(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={6} md={2.4}>
                    <TextField
                      fullWidth
                      label="Documentation (%)"
                      type="number"
                      inputProps={{ min: 70, max: 100, step: 0.1 }}
                      value={getSetting('qualityTargets.documentation')}
                      onChange={(e) => updateSetting('qualityTargets.documentation', parseFloat(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={6} md={2.4}>
                    <TextField
                      fullWidth
                      label="Security (%)"
                      type="number"
                      inputProps={{ min: 70, max: 100, step: 0.1 }}
                      value={getSetting('qualityTargets.security')}
                      onChange={(e) => updateSetting('qualityTargets.security', parseFloat(e.target.value))}
                    />
                  </Grid>
                  
                  <Grid item xs={12} sm={6} md={2.4}>
                    <TextField
                      fullWidth
                      label="Performance (%) - Ruv's 84.8%"
                      type="number"
                      inputProps={{ min: 70, max: 100, step: 0.1 }}
                      value={getSetting('qualityTargets.performance')}
                      onChange={(e) => updateSetting('qualityTargets.performance', parseFloat(e.target.value))}
                    />
                  </Grid>
                </Grid>
              </CardContent>
            </Card>
          </motion.div>
        </Grid>
      </Grid>

      {/* Reset Dialog */}
      <Dialog open={resetDialogOpen} onClose={() => setResetDialogOpen(false)}>
        <DialogTitle>Reset Settings</DialogTitle>
        <DialogContent>
          <Typography>
            Are you sure you want to reset all settings to their default values? 
            This action cannot be undone.
          </Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setResetDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleResetSettings} color="warning" variant="contained">
            Reset All Settings
          </Button>
        </DialogActions>
      </Dialog>

      {/* Export Dialog */}
      <Dialog open={exportDialogOpen} onClose={() => setExportDialogOpen(false)}>
        <DialogTitle>Export Settings</DialogTitle>
        <DialogContent>
          <Typography sx={{ mb: 2 }}>
            Export your current settings to a JSON file for backup or sharing.
          </Typography>
          <Alert severity="info">
            <AlertTitle>What's Included</AlertTitle>
            All performance, UI, swarm, neural, and quality settings will be exported.
          </Alert>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setExportDialogOpen(false)}>Cancel</Button>
          <Button onClick={handleExportSettings} variant="contained">
            Export Settings
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default Settings;