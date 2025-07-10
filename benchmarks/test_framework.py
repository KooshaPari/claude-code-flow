#!/usr/bin/env python3
"""
🏆 Simplified Benchmark Framework Test
Tests the benchmarking system without external dependencies.
"""

import asyncio
import json
import logging
import multiprocessing
import subprocess
import sys
import time
from concurrent.futures import ProcessPoolExecutor
from dataclasses import dataclass, asdict
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Any, Optional

logging.basicConfig(level=logging.INFO, format='%(levelname)s - %(message)s')
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

class SimpleBenchmarkRunner:
    """Simplified benchmark runner for testing"""
    
    BENCHMARK_WEIGHTS = {
        'swe_bench': 0.30,
        'humaneval': 0.20,
        'bigcode': 0.15,
        'repoeval': 0.15,
        'devai': 0.10,
        'security': 0.05,
        'codegen': 0.05
    }
    
    def __init__(self):
        self.session_id = datetime.now().strftime("%Y%m%d_%H%M%S")
        self.benchmarks_dir = Path(__file__).parent
        
    async def run_test_benchmarks(self) -> AggregateResult:
        """Run a subset of benchmarks for testing"""
        
        logger.info(f"🏆 Starting benchmark test session: {self.session_id}")
        start_time = time.time()
        
        # Define test benchmarks
        test_tasks = [
            {
                'name': 'humaneval_base',
                'category': 'humaneval',
                'script': 'humaneval/run_humaneval.py',
                'args': {'dataset': 'base', 'max-problems': 10, 'num-samples': 5},
                'weight': 0.20
            },
            {
                'name': 'multipl_e_python',
                'category': 'bigcode',
                'script': 'bigcode/run_multipl_e.py',
                'args': {'language': 'python', 'max-problems': 10},
                'weight': 0.15
            },
            {
                'name': 'repoeval_completion',
                'category': 'repoeval',
                'script': 'repoeval/run_repoeval.py',
                'args': {'max-repos': 5, 'tasks': ['completion']},
                'weight': 0.15
            },
            {
                'name': 'devai_debugging',
                'category': 'devai',
                'script': 'devai/run_devai.py',
                'args': {'task-type': 'debugging', 'max-scenarios': 10},
                'weight': 0.10
            },
            {
                'name': 'cwe_security',
                'category': 'security',
                'script': 'security/run_cwe_bench.py',
                'args': {'max-samples': 20, 'vulnerability-types': ['injection', 'xss']},
                'weight': 0.05
            }
        ]
        
        # Execute benchmarks in parallel
        results = await self._execute_parallel_benchmarks(test_tasks)
        
        # Calculate aggregate results
        aggregate_result = self._calculate_aggregate_results(results, time.time() - start_time)
        
        logger.info(f"✅ Test completed. WPI: {aggregate_result.weighted_performance_index:.2f}")
        return aggregate_result
    
    async def _execute_parallel_benchmarks(self, tasks: List[Dict[str, Any]]) -> List[BenchmarkResult]:
        """Execute benchmarks in parallel"""
        
        max_workers = min(4, multiprocessing.cpu_count())
        logger.info(f"Executing {len(tasks)} benchmarks with {max_workers} workers")
        
        with ProcessPoolExecutor(max_workers=max_workers) as executor:
            futures = []
            
            for task in tasks:
                future = executor.submit(self._execute_single_benchmark, task)
                futures.append((future, task))
            
            results = []
            for future, task in futures:
                try:
                    result = future.result(timeout=120)  # 2 minute timeout
                    results.append(result)
                    logger.info(f"✅ {task['name']}: {result.normalized_score:.1f}/100 "
                               f"({result.execution_time:.1f}s)")
                except Exception as e:
                    logger.error(f"❌ {task['name']}: {e}")
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
        """Execute a single benchmark"""
        
        start_time = time.time()
        script_path = self.benchmarks_dir / task['script']
        
        # Build command
        cmd = [sys.executable, str(script_path)]
        for key, value in task['args'].items():
            if isinstance(value, list):
                cmd.extend([f'--{key}'] + [str(v) for v in value])
            else:
                cmd.extend([f'--{key}', str(value)])
        
        try:
            # Execute benchmark
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=60,  # 1 minute timeout per benchmark
                cwd=self.benchmarks_dir
            )
            
            execution_time = time.time() - start_time
            
            if result.returncode != 0:
                raise RuntimeError(f"Benchmark failed: {result.stderr}")
            
            # Parse JSON result
            try:
                benchmark_output = json.loads(result.stdout.strip())
                score = benchmark_output.get('score', 0.0)
                max_score = benchmark_output.get('max_score', 100.0)
                details = benchmark_output.get('details', {})
            except json.JSONDecodeError:
                # Fallback if JSON parsing fails
                score = 0.0
                max_score = 100.0
                details = {'raw_output': result.stdout}
            
            normalized_score = (score / max_score) * 100 if max_score > 0 else 0
            
            return BenchmarkResult(
                name=task['name'],
                category=task['category'],
                score=score,
                max_score=max_score,
                normalized_score=normalized_score,
                execution_time=execution_time,
                details=details
            )
            
        except Exception as e:
            execution_time = time.time() - start_time
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
    
    def _calculate_aggregate_results(self, results: List[BenchmarkResult], total_time: float) -> AggregateResult:
        """Calculate weighted performance index"""
        
        successful_results = [r for r in results if r.error is None]
        failed_results = [r for r in results if r.error is not None]
        
        # Calculate category scores
        category_scores = {}
        category_weights = {}
        
        for result in successful_results:
            category = result.category
            
            if category not in category_scores:
                category_scores[category] = 0.0
                category_weights[category] = 0.0
            
            weight = self.BENCHMARK_WEIGHTS.get(category, 0.01)
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
        
        return AggregateResult(
            session_id=self.session_id,
            timestamp=datetime.now(),
            total_benchmarks=len(results),
            completed_benchmarks=len(successful_results),
            failed_benchmarks=len(failed_results),
            weighted_performance_index=wpi,
            category_scores=category_scores,
            individual_results=results
        )

def print_results(result: AggregateResult):
    """Print formatted benchmark results"""
    
    print("\n" + "="*70)
    print("🏆 COMPREHENSIVE SWE INTELLIGENCE BENCHMARK RESULTS")
    print("="*70)
    print(f"Session ID: {result.session_id}")
    print(f"Timestamp: {result.timestamp}")
    print(f"Completed: {result.completed_benchmarks}/{result.total_benchmarks}")
    print()
    
    # Overall score
    wpi = result.weighted_performance_index
    if wpi >= 90:
        grade = "🌟 EXCEPTIONAL (SOTA Level)"
    elif wpi >= 80:
        grade = "🥇 EXCELLENT (Production Ready)"
    elif wpi >= 70:
        grade = "🥈 GOOD (Competitive)"
    elif wpi >= 60:
        grade = "🥉 FAIR (Needs Improvement)"
    else:
        grade = "❌ POOR (Significant Gaps)"
    
    print(f"🎯 WEIGHTED PERFORMANCE INDEX: {wpi:.2f}/100")
    print(f"📊 GRADE: {grade}")
    print()
    
    # Category breakdown
    print("📋 CATEGORY SCORES:")
    print("-" * 40)
    for category, score in result.category_scores.items():
        print(f"  {category.upper():12}: {score:6.1f}/100")
    print()
    
    # Individual results
    print("📝 INDIVIDUAL BENCHMARK RESULTS:")
    print("-" * 70)
    for r in result.individual_results:
        status = "✅" if r.error is None else "❌"
        print(f"{status} {r.name:20} ({r.category:8}): {r.normalized_score:6.1f}/100 ({r.execution_time:5.1f}s)")
    
    print("\n" + "="*70)

async def main():
    """Main test function"""
    
    print("🏆 Starting Comprehensive SWE Intelligence Benchmark Test")
    print("This will test all major benchmark categories in parallel")
    print()
    
    runner = SimpleBenchmarkRunner()
    result = await runner.run_test_benchmarks()
    
    # Print results
    print_results(result)
    
    # Save results
    results_file = Path(f"test_results_{result.session_id}.json")
    with open(results_file, 'w') as f:
        json.dump(asdict(result), f, indent=2, default=str)
    
    print(f"\n💾 Detailed results saved to: {results_file}")
    print("\n🎉 Benchmark test completed successfully!")
    
    return result

if __name__ == "__main__":
    asyncio.run(main())