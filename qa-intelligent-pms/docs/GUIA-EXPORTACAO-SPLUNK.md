# Guia Manual: Exportar Métricas do Splunk

## Visão Geral

Como a API REST não está disponível, usamos um processo manual validado:
1. Você exporta os dados do Splunk via interface web usando a query padrão
2. Salva em arquivo CSV/JSON (exporte como **Events** em **Verbose Mode**)
3. O sistema processa e gera as métricas para estratégia reativa

**Método Padrão Validado**: A query na seção 4 foi testada e validada, capturando:
- Status codes HTTP (4XX e 5XX)
- Palavras-chave de erro
- Métricas completas por endpoint

## Passo a Passo

### 1. Acessar Splunk

1. Acesse: `https://hostfully.splunkcloud.com/en-GB/app/search/search`
2. Faça login com suas credenciais

### 2. Executar Query para Endpoints Mais Usados

**Query SPL** (ajuste o campo `endpoint` conforme seus logs):
```splunk
index=main earliest=-24h@h latest=now()
| stats count as total_requests, 
         dc(client_ip) as unique_clients,
         avg(response_time) as avg_response_time,
         max(response_time) as max_response_time
         by endpoint
| sort -total_requests
| head 20
```

**Nota**: Se o campo não for `endpoint`, substitua por `path`, `uri`, `url`, `route`, etc.

**Como executar**:
1. Cole a query no campo de busca
2. Clique em "Search" ou pressione Enter
3. Aguarde os resultados carregarem
4. Clique em "Export" → "Export as CSV" ou "Export as JSON"
5. Salve como: `endpoints_mais_usados.csv` ou `endpoints_mais_usados.json`

### 3. Executar Query para Endpoints Que Mais Falham

**Query SPL** (ajuste o campo `endpoint` conforme seus logs):
```splunk
index=main earliest=-24h@h latest=now()
| stats count as total_requests,
         count(eval(status>=400)) as total_errors,
         avg(response_time) as avg_response_time
         by endpoint
| eval error_rate = round((total_errors/total_requests)*100, 2)
| where total_errors > 0
| sort -total_errors
| head 20
```

**Nota**: Se o campo não for `endpoint`, substitua por `path`, `uri`, `url`, `route`, etc.

**Como executar**:
1. Cole a query no campo de busca
2. Clique em "Search"
3. Aguarde os resultados
4. Exporte como: `endpoints_mais_falhas.csv` ou `endpoints_mais_falhas.json`

### 4. Executar Query Consolidada ÚNICA (MÉTODO PADRÃO - Validado e Funcional)

**IMPORTANTE**: Esta é a query padrão validada que retorna todas as métricas necessárias em uma única execução.

**Query ÚNICA Padrão** (usa logs de acesso HTTP com status codes):
```splunk
index=main earliest=-6h@h latest=now() sourcetype=logs source=*localhost_access_log*
| rex field=_raw "\"(?<method>[A-Z]+)\s+(?<path>[^\s\"]+)\s+HTTP/1\.1\"\s+(?<status>\d+)" 
| eval is_4xx = if(status>=400 AND status<500, 1, 0)
| eval is_5xx = if(status>=500, 1, 0)
| eval has_error_keyword = if(
    match(_raw, "(?i)\\b(ERROR|FAIL|FAILED|EXCEPTION|TIMEOUT|FATAL|CRITICAL|UNAVAILABLE|REJECTED|DENIED|FORBIDDEN|UNAUTHORIZED|BAD_REQUEST|INTERNAL_ERROR|SERVICE_UNAVAILABLE|GATEWAY_TIMEOUT|CONNECTION_REFUSED|OUT_OF_MEMORY|STACK_TRACE|THROWABLE)\\b"), 1, 0)
| eval is_error = if(is_4xx=1 OR is_5xx=1 OR has_error_keyword=1, 1, 0)
| stats count as total_requests,
         sum(is_error) as total_errors,
         sum(is_4xx) as client_errors_4xx,
         sum(is_5xx) as server_errors_5xx,
         sum(has_error_keyword) as keyword_errors,
         avg(response_time) as avg_response_time,
         max(response_time) as max_response_time,
         min(response_time) as min_response_time,
         dc(client_ip) as unique_clients
         by path
| rename path as endpoint
| eval error_rate = round((total_errors/total_requests)*100, 2)
| eval importance_score = (total_requests * 0.4) + (total_errors * 0.6)
| eval is_critical = if((error_rate > 5 AND total_requests > 100) OR server_errors_5xx > 0, 1, 0)
| sort -importance_score
```

**Características da Query:**
- Usa logs de acesso HTTP (`localhost_access_log`) que contêm status codes
- Extrai `method`, `path` e `status` do padrão HTTP: `"GET /path HTTP/1.1" 200`
- Detecta erros por:
  - **4XX** (client errors): 400-499
  - **5XX** (server errors): 500-599 (mais críticos)
  - **Palavras-chave**: ERROR, FAIL, EXCEPTION, etc. (fallback quando não há status code)

**Palavras-chave detectadas:**
- ERROR, FAIL, FAILED, EXCEPTION
- TIMEOUT, FATAL, CRITICAL
- UNAVAILABLE, REJECTED, DENIED
- FORBIDDEN, UNAUTHORIZED, BAD_REQUEST
- INTERNAL_ERROR, SERVICE_UNAVAILABLE
- GATEWAY_TIMEOUT, CONNECTION_REFUSED
- OUT_OF_MEMORY, STACK_TRACE, THROWABLE

**Configurações importantes:**
- **Intervalo**: 6 horas (`earliest=-6h@h`) - ajuste se necessário:
  - `earliest=-3h@h` - Últimas 3 horas (mais rápido)
  - `earliest=-1h@h` - Última 1 hora (muito rápido)
  - `earliest=-30m@m` - Últimos 30 minutos (rápido para teste)
- **Mode**: Use **Verbose Mode** para garantir todos os dados
- **Export**: Exporte como **Events** (não Statistics) para dados completos

**Como executar**:
1. Cole a query no campo de busca do Splunk
2. Selecione **Verbose Mode** (ícone de documento com linhas)
3. Clique em "Search"
4. Aguarde os resultados carregarem
5. Clique em "Export" → **"Export as JSON"** (preferível) ou "Export as CSV"
6. Salve como: `complete_6h.json` (recomendado) ou `complete_6h.csv`

**Nota sobre formatos:**
- **JSON** (recomendado): Melhor para dados estruturados, preserva tipos, mais fácil de processar
- **CSV**: Também funciona, mas pode ter problemas com caracteres especiais
- **XML**: Não suportado pelo processador atual

### 5. Salvar Arquivos

Salve os arquivos exportados em:
```
qa-intelligent-pms/data/splunk_exports/
```

**Caminho completo (Windows)**:
```
C:\Users\User\Desktop\estrategia preventiva-reativa\qa-intelligent-pms\data\splunk_exports\
```

**Estrutura recomendada**:
```
data/
  splunk_exports/
    2024-01-15_metricas_completas.csv
    2024-01-15_endpoints_mais_usados.csv
    2024-01-15_endpoints_mais_falhas.csv
```

**Dica**: Use data no nome do arquivo para histórico (ex: `2024-01-15_metricas_completas.csv`)

**Nota**: A pasta é criada automaticamente quando você processa o primeiro arquivo.

## Processar Arquivos Exportados

Após exportar, execute:

```bash
# Processar arquivo específico
python scripts/process_splunk_export.py data/splunk_exports/complete_6h.csv

# Ou processar arquivo mais recente do diretório
python scripts/process_splunk_export.py data/splunk_exports/
```

**Como funciona o processamento**:
- Usa bibliotecas padrão do Python (`csv` e `json`)
- Não precisa de Pandas (mas está disponível se quiser)
- Detecta formato automaticamente (CSV ou JSON)
- Normaliza nomes de colunas automaticamente
- Converte tipos automaticamente (números, booleanos)

Veja detalhes em: `docs/COMO-FUNCIONA-PROCESSAMENTO.md`

## Formato Esperado dos Arquivos

### Formato Recomendado: JSON

**JSON é o formato preferido** porque:
- Preserva tipos de dados corretamente
- Melhor para dados estruturados
- Mais fácil de processar
- Evita problemas com caracteres especiais

O JSON deve ser um array de objetos:
```json
[
  {
    "endpoint": "/api/booking",
    "total_requests": 15000,
    "total_errors": 150,
    "error_rate": 1.0,
    "avg_response_time": 250.5,
    "unique_clients": 500,
    "is_critical": 0
  },
  ...
]
```

### CSV (Alternativa)

O CSV também funciona, mas pode ter problemas com caracteres especiais. Deve ter colunas como:
- `endpoint` (obrigatório)
- `total_requests` ou `count` (obrigatório)
- `total_errors` ou `errors` (opcional)
- `client_errors_4xx` (opcional) - erros 4XX detectados
- `server_errors_5xx` (opcional) - erros 5XX detectados
- `keyword_errors` (opcional) - erros detectados por palavras-chave
- `error_rate` (opcional)
- `avg_response_time` (opcional)
- `unique_clients` (opcional)
- `is_critical` (opcional)

### JSON

O JSON deve ser um array de objetos:
```json
[
  {
    "endpoint": "/api/booking",
    "total_requests": 15000,
    "total_errors": 150,
    "error_rate": 1.0,
    "avg_response_time": 250.5,
    "unique_clients": 500,
    "is_critical": 0
  },
  ...
]
```

## Frequência Recomendada

Execute este processo:
- **A cada ciclo/sprint** - Para atualizar estratégia reativa
- **Semanalmente** - Para acompanhar tendências
- **Após incidentes** - Para análise pós-incidente

**Dica**: Use intervalo curto (6h ou menos) para ser rápido e não gastar muito compute.
Os padrões de uso e erro geralmente se repetem, então 6 horas é suficiente para identificar tendências.

## Troubleshooting

**Problema**: Arquivo não é reconhecido
- Verifique se o formato é CSV ou JSON
- Verifique se tem a coluna `endpoint`

**Problema**: Dados não fazem sentido
- Verifique se a query foi executada corretamente
- Verifique o range de tempo (`earliest=-7d@d`)

**Problema**: Query demora muito
- Reduza o range de tempo (ex: `-24h@h` em vez de `-7d@d`)
- Adicione filtros específicos (ex: `index=main AND status>=400`)

## Queries Alternativas por Índice

Se você usar outros índices, ajuste a query:

**Para índice `test`**:
```splunk
search index=test earliest=-7d@d latest=now()
...
```

**Para índice `sandbox`**:
```splunk
search index=sandbox earliest=-7d@d latest=now()
...
```

**Para múltiplos índices**:
```splunk
search index=main OR index=test earliest=-7d@d latest=now()
...
```

## Próximos Passos

Após exportar e processar:
1. Execute `python scripts/get_reactive_metrics.py` (agora lê arquivos exportados)
2. Revise as métricas geradas
3. Use para priorizar testes na estratégia reativa
4. Documente insights no Jira

