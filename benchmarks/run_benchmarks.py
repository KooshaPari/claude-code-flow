#!/usr/bin/env python3
"""
🏆 Comprehensive SWE Intelligence Benchmarking Suite
Main entry point for parallel benchmark execution with aggregate scoring.
"""

import asyncio
import sys
from pathlib import Path

# Add benchmarks directory to path
sys.path.insert(0, str(Path(__file__).parent))

from orchestrator import BenchmarkOrchestrator, main

if __name__ == "__main__":
    print("🏆 Starting Comprehensive SWE Intelligence Benchmarking Suite")
    print("=" * 70)
    print()
    
    asyncio.run(main())