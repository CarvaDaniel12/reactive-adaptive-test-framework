#!/usr/bin/env python
"""
Exemplo mínimo funcional
Demonstra uso básico dos serviços sem necessidade de credenciais reais
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))


def example_risk_analyzer():
    """Exemplo: Analisador de risco"""
    print("\n=== Exemplo: Risk Analyzer ===")
    
    from src.domain.entities.ticket import Ticket
    from src.application.preventivo.risk_analyzer import RiskAnalyzer
    from src.domain.value_objects.risk_level import RiskLevel
    
    # Criar ticket de exemplo
    ticket = Ticket(
        key="PMS-123",
        summary="Implementar nova funcionalidade de reserva",
        description="Adicionar suporte para reservas em grupo com múltiplos quartos",
        issue_type="Story",
        status="To Do",
        components=["booking", "payment"],
        acceptance_criteria=[]  # Sem ACs = maior risco
    )
    
    # Analisar risco
    analyzer = RiskAnalyzer()
    risk_level = analyzer.calculate_risk(ticket)
    
    print(f"Ticket: {ticket.key}")
    print(f"Componentes: {ticket.get_component_string()}")
    print(f"Tem ACs: {ticket.has_acceptance_criteria()}")
    print(f"Risco calculado: {risk_level.value.upper()}")
    print(f"Score: {risk_level.to_score():.2f}")


def example_ac_generator():
    """Exemplo: Gerador de ACs"""
    print("\n=== Exemplo: AC Generator ===")
    
    from src.domain.entities.ticket import Ticket
    from src.application.preventivo.ac_generator import ACGenerator
    
    # Criar ticket sem ACs
    ticket = Ticket(
        key="PMS-456",
        summary="Corrigir bug no checkout",
        description="Sistema não está processando checkout corretamente",
        issue_type="Bug",
        status="To Do",
        components=["checkout"]
    )
    
    # Gerar ACs
    generator = ACGenerator()
    acs = generator.generate_acs(ticket)
    
    print(f"Ticket: {ticket.key}")
    print(f"Tipo: {ticket.issue_type}")
    print(f"\nACs gerados ({len(acs)}):")
    for i, ac in enumerate(acs, 1):
        print(f"  {i}. {ac}")


def example_value_objects():
    """Exemplo: Value Objects"""
    print("\n=== Exemplo: Value Objects ===")
    
    from src.domain.value_objects.risk_level import RiskLevel
    from src.domain.value_objects.test_priority import TestPriority
    
    # Converter score em RiskLevel
    scores = [0.1, 0.4, 0.7, 0.9]
    for score in scores:
        risk = RiskLevel.from_score(score)
        priority = TestPriority.from_risk_level(risk)
        print(f"Score {score:.1f} -> Risco: {risk.value} -> Prioridade: {priority.value}")


def main():
    """Executa exemplos"""
    print("=" * 50)
    print("Exemplos Minimos Funcionais")
    print("=" * 50)
    
    example_value_objects()
    example_risk_analyzer()
    example_ac_generator()
    
    print("\n" + "=" * 50)
    print("[OK] Exemplos executados com sucesso!")
    print("=" * 50)


if __name__ == "__main__":
    main()

