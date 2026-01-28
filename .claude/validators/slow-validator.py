#!/usr/bin/env python3
"""Slow validator that times out"""
import time
import sys

# Sleep for 10 seconds (longer than the 5 second timeout)
time.sleep(10)
sys.exit(0)
