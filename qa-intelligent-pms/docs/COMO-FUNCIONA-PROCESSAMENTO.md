# Como Funciona o Processamento de Dados do Splunk

## Localização dos Arquivos

**Pasta de exports**: `qa-intelligent-pms/data/splunk_exports/`

Caminho completo (Windows):
```
C:\Users\User\Desktop\estrategia preventiva-reativa\qa-intelligent-pms\data\splunk_exports\
```

Caminho relativo (do projeto):
```
data/splunk_exports/
```

## Como os Dados São Processados

### Método Atual: CSV/JSON Nativo (Bibliotecas Padrão)

**Bibliotecas usadas**:
- `csv` (biblioteca padrão do Python)
- `json` (biblioteca padrão do Python)

**Vantagens**:
- ✅ Sem dependências extras
- ✅ Leve e rápido
- ✅ Funciona em qualquer ambiente Python
- ✅ Fácil de debugar

**Como funciona**:

1. **CSV**:
   ```python
   # Detecta delimitador automaticamente (vírgula, ponto-e-vírgula, etc)
   # Lê linha por linha
   # Converte valores numéricos automaticamente
   # Normaliza nomes de colunas (case-insensitive)
   ```

2. **JSON**:
   ```python
   # Carrega JSON completo
   # Extrai array de resultados (se estiver em objeto)
   # Normaliza estrutura
   # Converte tipos automaticamente
   ```

### Processamento de Dados

**Fluxo**:

1. **Carregamento**:
   - Lê arquivo CSV ou JSON
   - Detecta formato automaticamente
   - Normaliza nomes de colunas

2. **Normalização**:
   - Converte nomes de colunas para padrão:
     - `endpoint` → `endpoint`
     - `total_requests`, `count`, `total` → `total_requests`
     - `total_errors`, `errors` → `total_errors`
     - `error_rate`, `error_rate_%` → `error_rate`
     - etc.

3. **Conversão de Tipos**:
   - Strings numéricas → int/float
   - Strings booleanas → bool
   - Mantém strings como strings

4. **Agregação**:
   - Ordena por critérios (uso, erros, etc)
   - Filtra endpoints críticos
   - Calcula métricas gerais

### Exemplo de Processamento

**Arquivo CSV de entrada**:
```csv
endpoint,total_requests,total_errors,error_rate
/api/booking,15000,150,1.0
/api/payment,8000,400,5.0
```

**Processamento**:
```python
# 1. Lê CSV
data = adapter.load_from_file("arquivo.csv")

# 2. Normaliza (já feito automaticamente)
# {
#   "endpoint": "/api/booking",
#   "total_requests": 15000,  # Convertido para int
#   "total_errors": 150,      # Convertido para int
#   "error_rate": 1.0         # Convertido para float
# }

# 3. Agrega
metrics = adapter.get_critical_metrics("arquivo.csv")
# Retorna métricas consolidadas
```

## Alternativa: Usar Pandas

**Pandas está disponível** no `requirements.txt`, mas **não está sendo usado** atualmente.

**Se quiser usar Pandas** (mais poderoso):

**Vantagens**:
- ✅ Operações mais complexas (groupby, merge, etc)
- ✅ Melhor para grandes volumes de dados
- ✅ Mais funcionalidades de análise

**Desvantagens**:
- ⚠️ Dependência extra (já está instalada)
- ⚠️ Mais pesado para dados pequenos
- ⚠️ Overhead desnecessário para casos simples

**Como implementar com Pandas**:

```python
import pandas as pd

def _load_csv_pandas(self, file_path: Path) -> List[Dict[str, Any]]:
    """Carrega CSV usando Pandas"""
    df = pd.read_csv(file_path)
    
    # Normalizar colunas
    df.columns = df.columns.str.lower().str.strip()
    
    # Converter para lista de dicionários
    return df.to_dict('records')
```

## Recomendação

**Manter CSV/JSON nativo** porque:
- Dados do Splunk são relativamente simples
- Não precisa de operações complexas
- Mais leve e rápido
- Sem dependências extras

**Usar Pandas apenas se**:
- Dados forem muito grandes (>100k linhas)
- Precisar de operações complexas (groupby, pivot, etc)
- Quiser fazer análises estatísticas avançadas

## Estrutura de Dados Esperada

### CSV

```csv
endpoint,total_requests,total_errors,error_rate,avg_response_time
/api/booking,15000,150,1.0,250.5
/api/payment,8000,400,5.0,300.2
```

### JSON

```json
[
  {
    "endpoint": "/api/booking",
    "total_requests": 15000,
    "total_errors": 150,
    "error_rate": 1.0,
    "avg_response_time": 250.5
  },
  {
    "endpoint": "/api/payment",
    "total_requests": 8000,
    "total_errors": 400,
    "error_rate": 5.0,
    "avg_response_time": 300.2
  }
]
```

## Testando o Processamento

```bash
# Criar arquivo de teste
cat > data/splunk_exports/test.csv << EOF
endpoint,total_requests,total_errors,error_rate
/api/booking,15000,150,1.0
/api/payment,8000,400,5.0
EOF

# Processar
python scripts/process_splunk_export.py data/splunk_exports/test.csv
```

## Troubleshooting

**Erro: "Arquivo não encontrado"**
- Verifique se o caminho está correto
- Use caminho relativo: `data/splunk_exports/arquivo.csv`
- Ou caminho absoluto: `C:/caminho/completo/arquivo.csv`

**Erro: "Formato não suportado"**
- Use apenas `.csv` ou `.json`
- Verifique se o arquivo não está corrompido

**Dados não aparecem corretamente**
- Verifique se tem coluna `endpoint`
- Verifique se nomes de colunas estão corretos
- Veja exemplos no guia de exportação

