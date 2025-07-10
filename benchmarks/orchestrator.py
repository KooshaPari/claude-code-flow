#!/usr/bin/env python3
"""
🏆 Comprehensive SWE Intelligence Benchmarking Orchestrator
Parallel execution of ALL major SWE benchmarks with aggregate scoring.
"""

import asyncio
import json
import logging
import multiprocessing
import time
from concurrent.futures import ProcessPoolExecutor, ThreadPoolExecutor
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Dict, List, Optional, Any, Tuple
import subprocess
import sys
import yaml
from datetime import datetime
import psutil
import docker

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('benchmark_orchestrator.log'),
        logging.StreamHandler(sys.stdout)
    ]
)
logger = logging.getLogger(__name__)

@dataclass
class BenchmarkResult:
    """Single benchmark execution result"""
    name: str
    category: str
    score: float
    max_score: float
    normalized_score: float
    execution_time: float
    details: Dict[str, Any]
    error: Optional[str] = None
    
@dataclass
class AggregateResult:
    """Complete benchmarking session results"""
    session_id: str
    timestamp: datetime
    total_benchmarks: int
    completed_benchmarks: int
    failed_benchmarks: int
    weighted_performance_index: float
    category_scores: Dict[str, float]
    individual_results: List[BenchmarkResult]
    execution_summary: Dict[str, Any]

class BenchmarkOrchestrator:
    """Main orchestrator for parallel benchmark execution"""
    
    # Benchmark weights for WPI calculation
    BENCHMARK_WEIGHTS = {
        'swe_bench': 0.30,
        'humaneval': 0.20,
        'bigcode': 0.15,
        'repoeval': 0.15,
        'devai': 0.10,
        'security': 0.05,
        'codegen': 0.05
    }
    
    # Confidence factors for benchmark maturity
    CONFIDENCE_FACTORS = {
        'swe_bench': 1.0,      # Mature, established
        'humaneval': 1.0,      # Mature, widely used
        'humaneval_plus': 0.8, # Established variant
        'mbpp': 0.8,           # Established
        'ds_1000': 0.8,        # Established
        'multipl_e': 0.8,      # Established
        'repoeval': 0.6,       # Emerging
        'devai_bench': 0.6,    # Emerging
        'cwb_bench': 0.6,      # Emerging
        'codeql': 0.8,         # Established
        'codet5': 0.8,         # Established
        'conala': 0.8          # Established
    }
    
    def __init__(self, config_path: str = "config.yaml"):
        """Initialize orchestrator with configuration"""
        self.config = self._load_config(config_path)
        self.session_id = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.results_dir = Path(f"results/{self.session_id}")
        self.results_dir.mkdir(parents=True, exist_ok=True)
        
        # Initialize Docker client for containerized execution
        try:
            self.docker_client = docker.from_env()
            logger.info("Docker client initialized successfully")
        except Exception as e:
            logger.warning(f"Docker not available: {e}")
            self.docker_client = None
            
        # Track system resources
        self.initial_cpu_count = psutil.cpu_count()
        self.initial_memory = psutil.virtual_memory().total
        
    def _load_config(self, config_path: str) -> Dict[str, Any]:
        """Load configuration from YAML file"""
        try:
            with open(config_path, 'r') as f:
                config = yaml.safe_load(f)
            logger.info(f"Configuration loaded from {config_path}")
            return config
        except FileNotFoundError:
            logger.warning(f"Config file {config_path} not found, using defaults")
            return self._default_config()
    
    def _default_config(self) -> Dict[str, Any]:
        """Default configuration if no config file provided"""
        return {
            'execution': {
                'parallel_workers': min(16, multiprocessing.cpu_count()),
                'timeout_minutes': 120,
                'retry_attempts': 3,
                'use_containers': True
            },
            'benchmarks': {
                'swe_bench': {
                    'enabled': True,
                    'subset': 'lite',
                    'max_instances': 100,
                    'timeout_per_instance': 300
                },
                'humaneval': {
                    'enabled': True,
                    'variants': ['base', 'plus'],
                    'max_problems': 164,
                    'pass_k': [1, 5, 10]
                },
                'bigcode': {
                    'enabled': True,
                    'languages': ['python', 'javascript', 'java'],
                    'tasks': ['multipl_e', 'ds_1000']
                },
                'repoeval': {
                    'enabled': True,
                    'max_repos': 50,
                    'context_window': 8192
                },
                'devai': {
                    'enabled': True,
                    'workflow_tasks': ['debugging', 'testing', 'refactoring']
                },
                'security': {
                    'enabled': True,
                    'frameworks': ['cwe', 'codeql']
                },
                'codegen': {
                    'enabled': True,
                    'tasks': ['conala', 'codet5']
                }
            },
            'output': {
                'detailed_logs': True,
                'save_intermediate': True,
                'generate_report': True
            }
        }

    async def run_all_benchmarks(self) -> AggregateResult:
        """Execute all enabled benchmarks in parallel"""
        logger.info(f"🏆 Starting comprehensive SWE benchmarking session: {self.session_id}")
        start_time = time.time()
        
        # Prepare benchmark tasks
        benchmark_tasks = self._prepare_benchmark_tasks()
        logger.info(f"Prepared {len(benchmark_tasks)} benchmark tasks")
        
        # Execute benchmarks in parallel
        results = await self._execute_parallel_benchmarks(benchmark_tasks)
        
        # Calculate aggregate metrics
        aggregate_result = self._calculate_aggregate_results(results, time.time() - start_time)
        
        # Save results
        await self._save_results(aggregate_result)
        
        # Generate report
        if self.config['output']['generate_report']:
            await self._generate_report(aggregate_result)
        
        logger.info(f"✅ Benchmarking session completed. WPI: {aggregate_result.weighted_performance_index:.2f}")
        return aggregate_result
    
    def _prepare_benchmark_tasks(self) -> List[Dict[str, Any]]:
        """Prepare all benchmark execution tasks"""
        tasks = []
        
        for benchmark_name, benchmark_config in self.config['benchmarks'].items():
            if not benchmark_config.get('enabled', False):
                continue
                
            if benchmark_name == 'swe_bench':
                tasks.extend(self._prepare_swe_bench_tasks(benchmark_config))
            elif benchmark_name == 'humaneval':
                tasks.extend(self._prepare_humaneval_tasks(benchmark_config))
            elif benchmark_name == 'bigcode':
                tasks.extend(self._prepare_bigcode_tasks(benchmark_config))
            elif benchmark_name == 'repoeval':
                tasks.extend(self._prepare_repoeval_tasks(benchmark_config))
            elif benchmark_name == 'devai':
                tasks.extend(self._prepare_devai_tasks(benchmark_config))
            elif benchmark_name == 'security':
                tasks.extend(self._prepare_security_tasks(benchmark_config))
            elif benchmark_name == 'codegen':
                tasks.extend(self._prepare_codegen_tasks(benchmark_config))
        
        return tasks
    
    def _prepare_swe_bench_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare SWE-Bench evaluation tasks"""
        tasks = []
        
        subset = config.get('subset', 'lite')
        max_instances = config.get('max_instances', 100)
        
        # SWE-Bench Lite (300 verified instances)
        if subset in ['lite', 'all']:
            tasks.append({
                'name': 'swe_bench_lite',
                'category': 'swe_bench',
                'type': 'repository_level',
                'script': 'benchmarks/swe_bench/run_swe_bench.py',
                'args': {
                    'dataset': 'princeton-nlp/SWE-bench_Lite',
                    'max_instances': min(max_instances, 300),
                    'timeout_per_instance': config.get('timeout_per_instance', 300)
                },
                'weight': 0.30,
                'confidence': 1.0
            })
        
        # SWE-Bench Full (2,294 instances)
        if subset in ['full', 'all']:
            tasks.append({
                'name': 'swe_bench_full',
                'category': 'swe_bench',
                'type': 'repository_level',
                'script': 'benchmarks/swe_bench/run_swe_bench.py',
                'args': {
                    'dataset': 'princeton-nlp/SWE-bench',
                    'max_instances': max_instances,
                    'timeout_per_instance': config.get('timeout_per_instance', 300)
                },
                'weight': 0.30,
                'confidence': 1.0
            })
        
        return tasks
    
    def _prepare_humaneval_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare HumanEval and variant tasks"""
        tasks = []
        variants = config.get('variants', ['base'])
        pass_k = config.get('pass_k', [1, 5, 10])
        
        for variant in variants:
            if variant == 'base':
                tasks.append({
                    'name': 'humaneval_base',
                    'category': 'humaneval',
                    'type': 'code_generation',
                    'script': 'benchmarks/humaneval/run_humaneval.py',
                    'args': {
                        'dataset': 'openai_humaneval',
                        'pass_k': pass_k,
                        'max_problems': config.get('max_problems', 164)
                    },
                    'weight': 0.20,
                    'confidence': 1.0
                })
            elif variant == 'plus':
                tasks.append({
                    'name': 'humaneval_plus',
                    'category': 'humaneval',
                    'type': 'code_generation',
                    'script': 'benchmarks/humaneval/run_humaneval_plus.py',
                    'args': {
                        'dataset': 'evalplus/humaneval-plus',
                        'pass_k': pass_k,
                        'max_problems': config.get('max_problems', 164)
                    },
                    'weight': 0.15,
                    'confidence': 0.8
                })
            elif variant == 'mbpp':
                tasks.append({
                    'name': 'mbpp',
                    'category': 'humaneval',
                    'type': 'code_generation',
                    'script': 'benchmarks/humaneval/run_mbpp.py',
                    'args': {
                        'dataset': 'mbpp',
                        'pass_k': pass_k,
                        'max_problems': config.get('max_problems', 500)
                    },
                    'weight': 0.15,
                    'confidence': 0.8
                })
        
        return tasks
    
    def _prepare_bigcode_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare BigCode evaluation tasks"""
        tasks = []
        languages = config.get('languages', ['python'])
        task_types = config.get('tasks', ['multipl_e'])
        
        for task_type in task_types:
            if task_type == 'multipl_e':
                for lang in languages:
                    tasks.append({
                        'name': f'multipl_e_{lang}',
                        'category': 'bigcode',
                        'type': 'multilingual_generation',
                        'script': 'benchmarks/bigcode/run_multipl_e.py',
                        'args': {
                            'language': lang,
                            'max_problems': 164
                        },
                        'weight': 0.15 / len(languages),
                        'confidence': 0.8
                    })
            elif task_type == 'ds_1000':
                tasks.append({
                    'name': 'ds_1000',
                    'category': 'bigcode',
                    'type': 'data_science',
                    'script': 'benchmarks/bigcode/run_ds_1000.py',
                    'args': {
                        'max_problems': 1000,
                        'libraries': ['pandas', 'numpy', 'matplotlib', 'scikit-learn']
                    },
                    'weight': 0.10,
                    'confidence': 0.8
                })
        
        return tasks
    
    def _prepare_repoeval_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare repository-level evaluation tasks"""
        return [{
            'name': 'repoeval',
            'category': 'repoeval',
            'type': 'repository_understanding',
            'script': 'benchmarks/repoeval/run_repoeval.py',
            'args': {
                'max_repos': config.get('max_repos', 50),
                'context_window': config.get('context_window', 8192),
                'tasks': ['completion', 'bug_fixing', 'feature_addition']
            },
            'weight': 0.15,
            'confidence': 0.6
        }]
    
    def _prepare_devai_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare DevAI workflow tasks"""
        tasks = []
        workflow_tasks = config.get('workflow_tasks', ['debugging'])
        
        for task in workflow_tasks:
            tasks.append({
                'name': f'devai_{task}',
                'category': 'devai',
                'type': 'workflow_simulation',
                'script': 'benchmarks/devai/run_devai.py',
                'args': {
                    'task_type': task,
                    'max_scenarios': 50
                },
                'weight': 0.10 / len(workflow_tasks),
                'confidence': 0.6
            })
        
        return tasks
    
    def _prepare_security_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare security assessment tasks"""
        tasks = []
        frameworks = config.get('frameworks', ['cwe'])
        
        for framework in frameworks:
            if framework == 'cwe':
                tasks.append({
                    'name': 'cwe_bench',
                    'category': 'security',
                    'type': 'vulnerability_detection',
                    'script': 'benchmarks/security/run_cwe_bench.py',
                    'args': {
                        'max_samples': 200,
                        'vulnerability_types': ['injection', 'xss', 'auth']
                    },
                    'weight': 0.05 / len(frameworks),
                    'confidence': 0.6
                })
            elif framework == 'codeql':
                tasks.append({
                    'name': 'codeql_bench',
                    'category': 'security',
                    'type': 'static_analysis',
                    'script': 'benchmarks/security/run_codeql.py',
                    'args': {
                        'max_repos': 100,
                        'analysis_types': ['security', 'quality']
                    },
                    'weight': 0.05 / len(frameworks),
                    'confidence': 0.8
                })
        
        return tasks
    
    def _prepare_codegen_tasks(self, config: Dict[str, Any]) -> List[Dict[str, Any]]:
        """Prepare code generation specific tasks"""
        tasks = []
        task_types = config.get('tasks', ['conala'])
        
        for task_type in task_types:
            if task_type == 'conala':
                tasks.append({
                    'name': 'conala',
                    'category': 'codegen',
                    'type': 'nl_to_code',
                    'script': 'benchmarks/codegen/run_conala.py',
                    'args': {
                        'max_problems': 500,
                        'metrics': ['bleu', 'exact_match']
                    },
                    'weight': 0.05 / len(task_types),
                    'confidence': 0.8
                })
            elif task_type == 'codet5':
                tasks.append({
                    'name': 'codet5',
                    'category': 'codegen',
                    'type': 'code_summarization',
                    'script': 'benchmarks/codegen/run_codet5.py',
                    'args': {
                        'max_samples': 1000,
                        'metrics': ['bleu', 'rouge', 'meteor']
                    },
                    'weight': 0.05 / len(task_types),
                    'confidence': 0.8
                })
        
        return tasks
    
    async def _execute_parallel_benchmarks(self, tasks: List[Dict[str, Any]]) -> List[BenchmarkResult]:
        """Execute all benchmark tasks in parallel"""
        max_workers = self.config['execution']['parallel_workers']
        timeout_minutes = self.config['execution']['timeout_minutes']
        
        logger.info(f"Executing {len(tasks)} benchmarks with {max_workers} parallel workers")
        
        # Use both process and thread pools for optimal resource utilization
        with ProcessPoolExecutor(max_workers=max_workers // 2) as process_pool, \
             ThreadPoolExecutor(max_workers=max_workers) as thread_pool:
            
            # Create futures for all tasks
            futures = []
            for task in tasks:
                if task['type'] in ['repository_level', 'multilingual_generation']:
                    # CPU-intensive tasks use process pool
                    future = process_pool.submit(self._execute_single_benchmark, task)
                else:
                    # I/O-intensive tasks use thread pool
                    future = thread_pool.submit(self._execute_single_benchmark, task)
                
                futures.append((future, task))
            
            # Collect results as they complete
            results = []
            completed = 0
            
            for future, task in futures:
                try:
                    result = future.result(timeout=timeout_minutes * 60)
                    results.append(result)
                    completed += 1
                    logger.info(f"✅ Completed {task['name']} ({completed}/{len(tasks)})")
                except Exception as e:
                    logger.error(f"❌ Failed {task['name']}: {e}")
                    # Create failed result
                    results.append(BenchmarkResult(
                        name=task['name'],
                        category=task['category'],
                        score=0.0,
                        max_score=100.0,
                        normalized_score=0.0,
                        execution_time=0.0,
                        details={'error': str(e)},
                        error=str(e)
                    ))
        
        return results
    
    def _execute_single_benchmark(self, task: Dict[str, Any]) -> BenchmarkResult:
        """Execute a single benchmark task"""
        start_time = time.time()
        logger.info(f"🚀 Starting {task['name']} ({task['category']})")
        
        try:
            # Determine execution method
            if self.config['execution']['use_containers'] and self.docker_client:
                result = self._execute_in_container(task)
            else:
                result = self._execute_locally(task)
            
            execution_time = time.time() - start_time
            
            # Normalize score to 0-100 scale
            normalized_score = (result['score'] / result['max_score']) * 100 if result['max_score'] > 0 else 0
            
            return BenchmarkResult(
                name=task['name'],
                category=task['category'],
                score=result['score'],
                max_score=result['max_score'],
                normalized_score=normalized_score,
                execution_time=execution_time,
                details=result.get('details', {})
            )
            
        except Exception as e:
            execution_time = time.time() - start_time
            logger.error(f"❌ {task['name']} failed: {e}")
            
            return BenchmarkResult(
                name=task['name'],
                category=task['category'],
                score=0.0,
                max_score=100.0,
                normalized_score=0.0,
                execution_time=execution_time,
                details={'error': str(e)},
                error=str(e)
            )
    
    def _execute_locally(self, task: Dict[str, Any]) -> Dict[str, Any]:
        """Execute benchmark task locally using subprocess"""
        script_path = task['script']
        args = task['args']
        
        # Build command
        cmd = [sys.executable, script_path]
        for key, value in args.items():
            if isinstance(value, list):
                cmd.extend([f'--{key}'] + [str(v) for v in value])
            else:
                cmd.extend([f'--{key}', str(value)])
        
        # Execute with timeout
        timeout = self.config['execution']['timeout_minutes'] * 60
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=timeout,
            cwd=Path(__file__).parent
        )
        
        if result.returncode != 0:
            raise RuntimeError(f"Benchmark failed: {result.stderr}")
        
        # Parse result
        try:
            return json.loads(result.stdout)
        except json.JSONDecodeError:
            # Fallback parsing
            return {
                'score': 0.0,
                'max_score': 100.0,
                'details': {'raw_output': result.stdout}
            }
    
    def _execute_in_container(self, task: Dict[str, Any]) -> Dict[str, Any]:
        """Execute benchmark task in Docker container for isolation"""
        # TODO: Implement containerized execution
        # For now, fall back to local execution
        return self._execute_locally(task)
    
    def _calculate_aggregate_results(self, results: List[BenchmarkResult], total_time: float) -> AggregateResult:
        """Calculate weighted performance index and aggregate metrics"""
        
        # Separate successful and failed results
        successful_results = [r for r in results if r.error is None]
        failed_results = [r for r in results if r.error is not None]
        
        # Calculate category scores
        category_scores = {}
        category_weights = {}
        
        for result in successful_results:
            category = result.category
            confidence = self.CONFIDENCE_FACTORS.get(result.name, 0.5)
            
            if category not in category_scores:
                category_scores[category] = 0.0
                category_weights[category] = 0.0
            
            # Weight by benchmark weight and confidence
            weight = self.BENCHMARK_WEIGHTS.get(category, 0.01) * confidence
            category_scores[category] += result.normalized_score * weight
            category_weights[category] += weight
        
        # Normalize category scores
        for category in category_scores:
            if category_weights[category] > 0:
                category_scores[category] /= category_weights[category]
        
        # Calculate overall WPI
        total_weighted_score = 0.0
        total_weight = 0.0
        
        for category, score in category_scores.items():
            weight = self.BENCHMARK_WEIGHTS.get(category, 0.01)
            total_weighted_score += score * weight
            total_weight += weight
        
        wpi = total_weighted_score / total_weight if total_weight > 0 else 0.0
        
        # Execution summary
        execution_summary = {
            'total_execution_time': total_time,
            'average_time_per_benchmark': total_time / len(results) if results else 0,
            'success_rate': len(successful_results) / len(results) if results else 0,
            'resource_usage': {
                'peak_cpu_percent': psutil.cpu_percent(),
                'peak_memory_percent': psutil.virtual_memory().percent
            }
        }
        
        return AggregateResult(
            session_id=self.session_id,
            timestamp=datetime.now(),
            total_benchmarks=len(results),
            completed_benchmarks=len(successful_results),
            failed_benchmarks=len(failed_results),
            weighted_performance_index=wpi,
            category_scores=category_scores,
            individual_results=results,
            execution_summary=execution_summary
        )
    
    async def _save_results(self, aggregate_result: AggregateResult):
        """Save results to JSON files"""
        
        # Save aggregate results
        aggregate_file = self.results_dir / "aggregate_results.json"
        with open(aggregate_file, 'w') as f:
            json.dump(asdict(aggregate_result), f, indent=2, default=str)
        
        # Save individual results
        individual_file = self.results_dir / "individual_results.json"
        individual_data = [asdict(result) for result in aggregate_result.individual_results]
        with open(individual_file, 'w') as f:
            json.dump(individual_data, f, indent=2, default=str)
        
        logger.info(f"Results saved to {self.results_dir}")
    
    async def _generate_report(self, aggregate_result: AggregateResult):
        """Generate comprehensive HTML report"""
        # TODO: Implement detailed HTML report generation with charts
        
        # For now, create a simple markdown report
        report_file = self.results_dir / "report.md"
        
        with open(report_file, 'w') as f:
            f.write(f"# SWE Intelligence Benchmark Report\n\n")
            f.write(f"**Session ID:** {aggregate_result.session_id}\n")
            f.write(f"**Timestamp:** {aggregate_result.timestamp}\n")
            f.write(f"**Weighted Performance Index:** {aggregate_result.weighted_performance_index:.2f}/100\n\n")
            
            f.write("## Category Scores\n\n")
            for category, score in aggregate_result.category_scores.items():
                f.write(f"- **{category.title()}:** {score:.2f}/100\n")
            
            f.write("\n## Individual Results\n\n")
            for result in aggregate_result.individual_results:
                status = "✅" if result.error is None else "❌"
                f.write(f"{status} **{result.name}** ({result.category}): {result.normalized_score:.1f}/100 "
                       f"({result.execution_time:.1f}s)\n")
            
            f.write(f"\n## Execution Summary\n\n")
            f.write(f"- **Total Time:** {aggregate_result.execution_summary['total_execution_time']:.1f}s\n")
            f.write(f"- **Success Rate:** {aggregate_result.execution_summary['success_rate']:.1%}\n")
            f.write(f"- **Completed:** {aggregate_result.completed_benchmarks}/{aggregate_result.total_benchmarks}\n")
        
        logger.info(f"Report generated: {report_file}")

async def main():
    """Main entry point for benchmark orchestrator"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Comprehensive SWE Intelligence Benchmarking')
    parser.add_argument('--config', default='config.yaml', help='Configuration file path')
    parser.add_argument('--output-dir', default='results', help='Output directory for results')
    parser.add_argument('--workers', type=int, help='Number of parallel workers')
    parser.add_argument('--timeout', type=int, help='Timeout in minutes')
    parser.add_argument('--category', choices=[
        'swe_bench', 'humaneval', 'bigcode', 'repoeval', 'devai', 'security', 'codegen'
    ], help='Run only specific category')
    
    args = parser.parse_args()
    
    # Create orchestrator
    orchestrator = BenchmarkOrchestrator(args.config)
    
    # Override config with command line args
    if args.workers:
        orchestrator.config['execution']['parallel_workers'] = args.workers
    if args.timeout:
        orchestrator.config['execution']['timeout_minutes'] = args.timeout
    
    # Run specific category if requested
    if args.category:
        for category in orchestrator.config['benchmarks']:
            orchestrator.config['benchmarks'][category]['enabled'] = (category == args.category)
    
    # Execute benchmarks
    result = await orchestrator.run_all_benchmarks()
    
    # Print summary
    print(f"\n🏆 Benchmarking Complete!")
    print(f"Weighted Performance Index: {result.weighted_performance_index:.2f}/100")
    print(f"Completed: {result.completed_benchmarks}/{result.total_benchmarks}")
    print(f"Results saved to: {orchestrator.results_dir}")

if __name__ == "__main__":
    asyncio.run(main())