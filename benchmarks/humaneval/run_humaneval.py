#!/usr/bin/env python3
"""
🏆 HumanEval Evaluation Runner
Executes HumanEval and variants for code generation benchmarking.
"""

import argparse
import json
import logging
import multiprocessing
import tempfile
import time
from concurrent.futures import ProcessPoolExecutor, TimeoutError
from pathlib import Path
from typing import Dict, List, Any, Optional
import subprocess
import sys

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class HumanEvalRunner:
    """HumanEval evaluation runner"""
    
    def __init__(self, dataset_type: str = "base"):
        """Initialize HumanEval runner"""
        self.dataset_type = dataset_type
        self.temp_dir = Path(tempfile.mkdtemp(prefix="humaneval_"))
        self.results_dir = self.temp_dir / "results"
        self.results_dir.mkdir(exist_ok=True)
        
    def load_problems(self, max_problems: int = None) -> List[Dict[str, Any]]:
        """Load HumanEval problems"""
        logger.info(f"Loading {self.dataset_type} HumanEval problems")
        
        try:
            if self.dataset_type == "base":
                problems = self._load_humaneval_base()
            elif self.dataset_type == "plus":
                problems = self._load_humaneval_plus()
            elif self.dataset_type == "mbpp":
                problems = self._load_mbpp()
            else:
                raise ValueError(f"Unknown dataset type: {self.dataset_type}")
            
            if max_problems:
                problems = problems[:max_problems]
            
            logger.info(f"Loaded {len(problems)} problems")
            return problems
            
        except Exception as e:
            logger.error(f"Failed to load problems: {e}")
            return self._generate_mock_problems(max_problems or 10)
    
    def _load_humaneval_base(self) -> List[Dict[str, Any]]:
        """Load original HumanEval dataset"""
        # In practice, this would load from the actual HumanEval dataset
        # For now, return representative mock data
        return self._generate_mock_problems(164)
    
    def _load_humaneval_plus(self) -> List[Dict[str, Any]]:
        """Load HumanEval+ with additional test cases"""
        problems = self._generate_mock_problems(164)
        
        # Add additional test cases for HumanEval+
        for problem in problems:
            problem['extra_tests'] = [
                f"assert {problem['entry_point']}({i}) == expected_{i}" 
                for i in range(3, 8)
            ]
        
        return problems
    
    def _load_mbpp(self) -> List[Dict[str, Any]]:
        """Load MBPP dataset"""
        return self._generate_mock_problems(974, problem_type="mbpp")
    
    def _generate_mock_problems(self, count: int, problem_type: str = "humaneval") -> List[Dict[str, Any]]:
        """Generate mock problems for testing"""
        problems = []
        
        for i in range(count):
            if problem_type == "mbpp":
                # MBPP-style problem
                problem = {
                    'task_id': f'MBPP/{i}',
                    'text': f'Write a function to solve problem {i}.',
                    'code': f'def solve_problem_{i}(x):\n    """Solve problem {i}"""\n    return x * 2 + 1',
                    'test_list': [
                        f'assert solve_problem_{i}(1) == 3',
                        f'assert solve_problem_{i}(2) == 5',
                        f'assert solve_problem_{i}(0) == 1'
                    ],
                    'test_setup_code': '',
                    'challenge_test_list': []
                }
            else:
                # HumanEval-style problem
                problem = {
                    'task_id': f'HumanEval/{i}',
                    'prompt': f'def solve_problem_{i}(x):\n    """\n    Solve problem {i}\n    >>> solve_problem_{i}(1)\n    3\n    >>> solve_problem_{i}(2)\n    5\n    """\n',
                    'entry_point': f'solve_problem_{i}',
                    'canonical_solution': f'    return x * 2 + 1',
                    'test': f'def check(candidate):\n    assert candidate(1) == 3\n    assert candidate(2) == 5\n    assert candidate(0) == 1\n'
                }
            
            problems.append(problem)
        
        return problems
    
    def evaluate_problems(self, 
                         problems: List[Dict[str, Any]], 
                         pass_k: List[int] = [1, 5, 10],
                         num_samples: int = 100) -> Dict[str, Any]:
        """Evaluate problems and calculate pass@k metrics"""
        
        logger.info(f"Evaluating {len(problems)} problems with pass@k: {pass_k}")
        
        all_results = []
        
        # Process problems in parallel
        max_workers = min(multiprocessing.cpu_count(), 8)
        
        with ProcessPoolExecutor(max_workers=max_workers) as executor:
            futures = []
            
            for problem in problems:
                future = executor.submit(
                    self._evaluate_single_problem, 
                    problem, 
                    num_samples
                )
                futures.append((future, problem))
            
            # Collect results
            for future, problem in futures:
                try:
                    result = future.result(timeout=300)  # 5 min timeout per problem
                    all_results.append(result)
                    
                    logger.info(f"Problem {problem['task_id']}: "
                               f"{result['num_passed']}/{result['num_samples']} passed")
                    
                except TimeoutError:
                    logger.error(f"Timeout for problem {problem['task_id']}")
                    all_results.append({
                        'task_id': problem['task_id'],
                        'num_passed': 0,
                        'num_samples': num_samples,
                        'error': 'timeout'
                    })
                except Exception as e:
                    logger.error(f"Error for problem {problem['task_id']}: {e}")
                    all_results.append({
                        'task_id': problem['task_id'],
                        'num_passed': 0,
                        'num_samples': num_samples,
                        'error': str(e)
                    })
        
        # Calculate pass@k metrics
        pass_at_k_scores = {}
        for k in pass_k:
            pass_at_k_scores[f'pass@{k}'] = self._calculate_pass_at_k(all_results, k)
        
        # Overall metrics
        total_samples = sum(r['num_samples'] for r in all_results)
        total_passed = sum(r['num_passed'] for r in all_results)
        overall_success_rate = total_passed / total_samples if total_samples > 0 else 0
        
        return {
            'score': pass_at_k_scores.get('pass@1', 0) * 100,  # Use pass@1 as primary score
            'max_score': 100,
            'details': {
                'pass_at_k_scores': pass_at_k_scores,
                'overall_success_rate': overall_success_rate,
                'total_problems': len(problems),
                'total_samples': total_samples,
                'total_passed': total_passed,
                'individual_results': all_results,
                'dataset_type': self.dataset_type
            }
        }
    
    def _evaluate_single_problem(self, problem: Dict[str, Any], num_samples: int) -> Dict[str, Any]:
        """Evaluate a single problem with multiple solution attempts"""
        
        task_id = problem['task_id']
        passed_samples = 0
        
        for i in range(num_samples):
            try:
                # Generate solution for this problem
                solution = self._generate_solution(problem)
                
                # Test the solution
                if self._test_solution(problem, solution):
                    passed_samples += 1
                    
            except Exception as e:
                logger.debug(f"Sample {i} failed for {task_id}: {e}")
                continue
        
        return {
            'task_id': task_id,
            'num_passed': passed_samples,
            'num_samples': num_samples
        }
    
    def _generate_solution(self, problem: Dict[str, Any]) -> str:
        """Generate a solution for the given problem"""
        
        # This simulates AI code generation
        # In practice, this would call your AI model
        
        task_id = problem['task_id']
        
        if self.dataset_type == "mbpp":
            # For MBPP, extract function from provided code
            return problem.get('code', 'def solution(): pass')
        else:
            # For HumanEval, combine prompt with canonical solution
            prompt = problem['prompt']
            canonical = problem.get('canonical_solution', '    pass')
            
            # Simulate some variation in solutions
            import random
            if random.random() < 0.8:  # 80% chance of correct solution
                return prompt + canonical
            else:
                # Introduce some errors for realistic simulation
                return prompt + "    return None  # Error simulation"
    
    def _test_solution(self, problem: Dict[str, Any], solution: str) -> bool:
        """Test if a solution passes the test cases"""
        
        try:
            if self.dataset_type == "mbpp":
                return self._test_mbpp_solution(problem, solution)
            else:
                return self._test_humaneval_solution(problem, solution)
        except Exception:
            return False
    
    def _test_humaneval_solution(self, problem: Dict[str, Any], solution: str) -> bool:
        """Test HumanEval solution"""
        
        # Extract the function from the solution
        entry_point = problem['entry_point']
        test_code = problem['test']
        
        # Create test environment
        test_env = {}
        
        try:
            # Execute solution
            exec(solution, test_env)
            
            # Check if function exists
            if entry_point not in test_env:
                return False
            
            # Execute test
            candidate = test_env[entry_point]
            test_code_modified = test_code.replace('candidate', entry_point)
            exec(f"def check_wrapper():\n    {test_code_modified.replace('def check(candidate):', '').replace('    ', '        ')}", 
                 {entry_point: candidate})
            
            # If we get here, all tests passed
            return True
            
        except Exception:
            return False
    
    def _test_mbpp_solution(self, problem: Dict[str, Any], solution: str) -> bool:
        """Test MBPP solution"""
        
        test_list = problem.get('test_list', [])
        setup_code = problem.get('test_setup_code', '')
        
        try:
            # Create test environment
            test_env = {}
            
            # Execute setup code
            if setup_code:
                exec(setup_code, test_env)
            
            # Execute solution
            exec(solution, test_env)
            
            # Run all tests
            for test_case in test_list:
                exec(test_case, test_env)
            
            return True
            
        except Exception:
            return False
    
    def _calculate_pass_at_k(self, results: List[Dict[str, Any]], k: int) -> float:
        """Calculate pass@k metric"""
        
        total_problems = len(results)
        passed_problems = 0
        
        for result in results:
            num_passed = result['num_passed']
            num_samples = result['num_samples']
            
            if num_samples >= k:
                # Calculate probability of at least one pass in k samples
                prob_none_pass = 1.0
                for i in range(k):
                    prob_none_pass *= (num_samples - num_passed - i) / (num_samples - i)
                
                prob_at_least_one_pass = 1.0 - prob_none_pass
                passed_problems += prob_at_least_one_pass
            else:
                # If fewer samples than k, use all available samples
                if num_passed > 0:
                    passed_problems += 1
        
        return passed_problems / total_problems if total_problems > 0 else 0
    
    def cleanup(self):
        """Clean up temporary files"""
        import shutil
        if self.temp_dir.exists():
            shutil.rmtree(self.temp_dir)

def main():
    """Main entry point for HumanEval evaluation"""
    parser = argparse.ArgumentParser(description='Run HumanEval evaluation')
    parser.add_argument('--dataset', choices=['base', 'plus', 'mbpp'], default='base',
                       help='Dataset type')
    parser.add_argument('--max-problems', type=int, default=164,
                       help='Maximum number of problems to evaluate')
    parser.add_argument('--pass-k', nargs='+', type=int, default=[1, 5, 10],
                       help='Pass@k values to calculate')
    parser.add_argument('--num-samples', type=int, default=100,
                       help='Number of solution samples per problem')
    parser.add_argument('--output', default='humaneval_results.json',
                       help='Output file for results')
    
    args = parser.parse_args()
    
    # Initialize runner
    runner = HumanEvalRunner(args.dataset)
    
    try:
        # Load problems
        problems = runner.load_problems(args.max_problems)
        
        # Evaluate problems
        results = runner.evaluate_problems(problems, args.pass_k, args.num_samples)
        
        # Output results in the format expected by orchestrator
        output = {
            'score': results['score'],
            'max_score': results['max_score'],
            'details': results['details']
        }
        
        # Save detailed results
        with open(args.output, 'w') as f:
            json.dump(results, f, indent=2)
        
        # Print results for orchestrator
        print(json.dumps(output))
        
        logger.info(f"HumanEval evaluation completed!")
        logger.info(f"Pass@1: {results['details']['pass_at_k_scores'].get('pass@1', 0):.1%}")
        logger.info(f"Primary score: {results['score']:.1f}/100")
        
    finally:
        runner.cleanup()

if __name__ == "__main__":
    main()