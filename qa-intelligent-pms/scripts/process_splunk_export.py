#!/usr/bin/env python3
"""
Script para processar arquivos exportados do Splunk

Uso:
    python scripts/process_splunk_export.py data/splunk_exports/metricas_completas.csv
    python scripts/process_splunk_export.py data/splunk_exports/  # Processa todos
"""

import sys
from pathlib import Path

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
            endpoint = endpoint_data.get('endpoint', 'N/A') or 'N/A'
            total = endpoint_data.get('total_requests', 0)
            avg_time = endpoint_data.get('avg_response_time')
            clients = endpoint_data.get('unique_clients', 0)
            print(f"{i:2d}. {endpoint}")
            avg_time_str = f"{avg_time:.0f}ms" if avg_time is not None else "N/A"
            print(f"     Requisicoes: {total:,} | Clientes unicos: {clients} | Tempo medio: {avg_time_str}")
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
            total = endpoint_data.get('total_requests', 0)
            print(f"{i:2d}. {endpoint}")
            print(f"     Erros: {errors:,} ({error_rate}%) | Total: {total:,}")
    else:
        print("  Nenhum erro encontrado (otimo!)")
    print()
    
    # Endpoints críticos
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
    if len(sys.argv) < 2:
        print("Uso: python scripts/process_splunk_export.py <arquivo_ou_diretorio>")
        print()
        print("Exemplos:")
        print("  python scripts/process_splunk_export.py data/splunk_exports/metricas_completas.csv")
        print("  python scripts/process_splunk_export.py data/splunk_exports/")
        sys.exit(1)
    
    input_path = Path(sys.argv[1])
    
    try:
        from src.infrastructure.adapters.splunk_file_adapter import SplunkFileAdapter, SplunkFileError
        
        adapter = SplunkFileAdapter()
        
        if input_path.is_file():
            # Processar arquivo único
            print(f"Processando arquivo: {input_path}")
            print()
            metrics = adapter.get_critical_metrics(str(input_path))
            print_metrics(metrics)
            
        elif input_path.is_dir():
            # Processar todos os arquivos do diretório
            files = list(input_path.glob('*.csv')) + list(input_path.glob('*.json'))
            if not files:
                print(f"Nenhum arquivo CSV ou JSON encontrado em {input_path}")
                sys.exit(1)
            
            # Processar arquivo mais recente
            latest_file = max(files, key=lambda f: f.stat().st_mtime)
            print(f"Processando arquivo mais recente: {latest_file}")
            print()
            metrics = adapter.get_critical_metrics(str(latest_file))
            print_metrics(metrics)
            
        else:
            print(f"Erro: {input_path} não é um arquivo ou diretório válido")
            sys.exit(1)
        
        print()
        print("[SUCESSO] Metricas processadas com sucesso!")
        print()
        print("Use essas metricas para:")
        print("  - Priorizar testes nos endpoints mais usados")
        print("  - Investigar endpoints que mais falham")
        print("  - Focar em endpoints criticos (alto uso + alta taxa de erro)")
        
    except SplunkFileError as e:
        print(f"[ERRO] {e}")
        print()
        print("Verifique:")
        print("  - Arquivo existe e está acessível")
        print("  - Formato é CSV ou JSON")
        print("  - Arquivo contém coluna 'endpoint'")
        sys.exit(1)
    except Exception as e:
        print(f"[ERRO] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()

