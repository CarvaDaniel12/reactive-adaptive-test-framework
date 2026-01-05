#!/usr/bin/env python3
"""
Script para obter métricas essenciais do Splunk para estratégia reativa

Foca em:
- Endpoints mais usados
- Endpoints que mais falham
- Eventos críticos (alto uso + alta taxa de erro)
"""

import sys
from pathlib import Path
from datetime import timedelta

# Adicionar diretório raiz ao path
project_root = Path(__file__).parent.parent
sys.path.insert(0, str(project_root))

def print_metrics(metrics: dict):
    """Imprime métricas formatadas"""
    print("=" * 70)
    print("METRICAS CRITICAS PARA ESTRATEGIA REATIVA")
    print("=" * 70)
    print()
    
    overall = metrics.get('overall_metrics', {})
    print(f"Periodo: {metrics.get('time_range', 'N/A')}")
    print(f"Indice: {metrics.get('index', 'N/A')}")
    print()
    print("METRICAS GERAIS:")
    print(f"  Total de requisicoes: {overall.get('total_requests', 0):,}")
    print(f"  Total de erros: {overall.get('total_errors', 0):,}")
    print(f"  Taxa de erro geral: {overall.get('overall_error_rate', 0)}%")
    print(f"  Endpoints unicos: {overall.get('unique_endpoints', 0)}")
    print()
    
    # Endpoints mais usados
    print("=" * 70)
    print("TOP 10 ENDPOINTS MAIS USADOS (Prioridade de Teste)")
    print("=" * 70)
    most_used = metrics.get('most_used_endpoints', [])
    if most_used:
        for i, endpoint_data in enumerate(most_used[:10], 1):
            endpoint = endpoint_data.get('endpoint', 'N/A')
            total = endpoint_data.get('total_requests', 0)
            avg_time = endpoint_data.get('avg_response_time', 0)
            clients = endpoint_data.get('unique_clients', 0)
            print(f"{i:2d}. {endpoint}")
            print(f"     Requisicoes: {total:,} | Clientes unicos: {clients} | Tempo medio: {avg_time:.0f}ms")
    else:
        print("  Nenhum dado disponivel")
    print()
    
    # Endpoints que mais falham
    print("=" * 70)
    print("TOP 10 ENDPOINTS QUE MAIS FALHAM (Problemas Criticos)")
    print("=" * 70)
    most_failed = metrics.get('most_failed_endpoints', [])
    if most_failed:
        for i, endpoint_data in enumerate(most_failed[:10], 1):
            endpoint = endpoint_data.get('endpoint', 'N/A')
            errors = endpoint_data.get('total_errors', 0)
            error_rate = endpoint_data.get('error_rate', 0)
            total = endpoint_data.get('total', 0)
            print(f"{i:2d}. {endpoint}")
            print(f"     Erros: {errors:,} ({error_rate}%) | Total: {total:,}")
    else:
        print("  Nenhum erro encontrado (otimo!)")
    print()
    
    # Endpoints críticos (alto uso + alta taxa de erro)
    print("=" * 70)
    print("ENDPOINTS CRITICOS (Alto Uso + Alta Taxa de Erro)")
    print("=" * 70)
    critical = metrics.get('critical_endpoints', [])
    if critical:
        for i, endpoint_data in enumerate(critical, 1):
            endpoint = endpoint_data.get('endpoint', 'N/A')
            total = endpoint_data.get('total_requests', 0)
            errors = endpoint_data.get('total_errors', 0)
            error_rate = endpoint_data.get('error_rate', 0)
            print(f"{i:2d}. {endpoint}")
            print(f"     Requisicoes: {total:,} | Erros: {errors:,} | Taxa: {error_rate}%")
            print(f"     [URGENTE] Priorizar testes e investigacao")
    else:
        print("  Nenhum endpoint critico encontrado")
    print()
    
    # Recomendações
    print("=" * 70)
    print("RECOMENDACOES PARA ESTRATEGIA REATIVA")
    print("=" * 70)
    print()
    
    if most_used:
        print("1. PRIORIZAR TESTES NOS ENDPOINTS MAIS USADOS:")
        for endpoint_data in most_used[:5]:
            endpoint = endpoint_data.get('endpoint', 'N/A')
            total = endpoint_data.get('total_requests', 0)
            print(f"   - {endpoint} ({total:,} requisicoes)")
        print()
    
    if most_failed:
        print("2. INVESTIGAR URGENTEMENTE ENDPOINTS COM MAIS FALHAS:")
        for endpoint_data in most_failed[:5]:
            endpoint = endpoint_data.get('endpoint', 'N/A')
            errors = endpoint_data.get('total_errors', 0)
            error_rate = endpoint_data.get('error_rate', 0)
            print(f"   - {endpoint} ({errors:,} erros, {error_rate}% taxa)")
        print()
    
    if critical:
        print("3. ENDPOINTS CRITICOS (Alto uso + Alta taxa de erro):")
        for endpoint_data in critical:
            endpoint = endpoint_data.get('endpoint', 'N/A')
            print(f"   - {endpoint} [CRITICO]")
        print()
    
    print("=" * 70)


def main():
    """Função principal"""
    try:
        from src.infrastructure.adapters.splunk_adapter import SplunkAdapter, SplunkConnectionError
        from src.infrastructure.config.load_config import load_config
        
        print("Carregando configuracoes...")
        config = load_config()
        splunk_config = config.splunk
        
        # Ativar REST API se necessário
        splunk_config['use_rest_api'] = True
        
        print("Inicializando adapter Splunk...")
        adapter = SplunkAdapter(splunk_config)
        
        # Obter métricas críticas (últimos 7 dias por padrão)
        print()
        print("Obtendo metricas criticas...")
        print()
        print("NOTA: Se API REST nao estiver disponivel, o sistema tentara")
        print("      importar de arquivos exportados em data/splunk_exports/")
        print()
        
        try:
            # Tentar obter métricas (pode usar arquivo se API não funcionar)
            metrics = adapter.get_critical_metrics(time_range="-7d@d")
            print_metrics(metrics)
            
            print()
            print("[SUCESSO] Metricas obtidas com sucesso!")
            print()
            print("Use essas metricas para:")
            print("  - Priorizar testes nos endpoints mais usados")
            print("  - Investigar endpoints que mais falham")
            print("  - Focar em endpoints criticos (alto uso + alta taxa de erro)")
            print()
            print("Execute este script periodicamente (ex: a cada ciclo) para")
            print("atualizar a estrategia reativa baseada em dados reais de producao.")
            
            return True
            
        except SplunkConnectionError as e:
            print(f"[ERRO] Falha na conexao: {e}")
            print()
            print("Verifique:")
            print("  - Porta 8089 aberta (pode precisar abrir via support case)")
            print("  - Credenciais corretas")
            print("  - Acesso a API REST")
            return False
            
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        return False


if __name__ == "__main__":
    success = main()
    exit(0 if success else 1)

