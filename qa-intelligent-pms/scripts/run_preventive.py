#!/usr/bin/env python
"""
Script para executar análise preventiva
"""

import sys
from pathlib import Path

# Adicionar src ao path
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.presentation.cli import main

if __name__ == "__main__":
    main()

