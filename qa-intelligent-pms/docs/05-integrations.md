# Guia de Integrações

Este documento detalha como configurar e usar as integrações com Jira, Splunk e Postman.

## Status das Conexões

- ✅ **Jira**: Conectado e funcionando
- ✅ **Postman**: Conectado e funcionando (52 collections encontradas)
  - ✅ Busca de matches na interface web
  - ✅ Matching de endpoints
  - ✅ Visualização de requests
- ✅ **Testmo**: Conectado e funcionando
  - ✅ Criação de test cases
  - ✅ Atualização de test cases
  - ✅ Verificação de existência
  - ✅ Sistema de herança (Base → Reativo)
  - ✅ Sincronização via interface web
- ⚠️ **Splunk API**: Requer configuração de rede adicional (veja seção abaixo)
- ✅ **Splunk File**: Funcionando (processamento de CSV/JSON via interface web)

## Integração com Jira

**Status**: ✅ Funcionando e testado

### Pré-requisitos

- Acesso ao Jira (Cloud ou Server)
- Permissões para ler tickets e boards
- API Token (para autenticação)

### Obter API Token

1. Acesse: https://id.atlassian.com/manage-profile/security/api-tokens
2. Clique em "Create API token"
3. Dê um nome descritivo (ex: "QA-testing-plan-documentation")
4. Copie o token gerado (você só verá uma vez)

**Nota**: O token configurado está funcionando corretamente.

### Configuração

Edite `configs/jira_config.yaml`:

```yaml
jira:
  base_url: "https://seu-dominio.atlassian.net"
  api_version: "3"
  authentication:
    type: "basic"
    username: "seu-email@exemplo.com"
    api_token: "${JIRA_API_TOKEN}"  # Use variável de ambiente
  default_fields:
    - "summary"
    - "description"
    - "components"
    - "status"
    - "issuetype"
    - "assignee"
    - "reporter"
  timeout: 30
```

Configure no `.env`:

```bash
JIRA_API_TOKEN=seu-token-aqui
```

### Exemplos de Uso

#### Buscar Tickets de uma Sprint

```python
from src.infrastructure.adapters.jira_adapter import JiraAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = JiraAdapter(config.jira)

# Buscar tickets de uma Sprint
sprint_id = "123"  # ID da Sprint
tickets = adapter.get_sprint_tickets(sprint_id)

for ticket in tickets:
    print(f"{ticket.key}: {ticket.summary}")
    print(f"  Status: {ticket.status}")
    print(f"  Componentes: {', '.join(ticket.components)}")
```

#### Buscar Ticket Específico

```python
ticket_key = "PMS-123"
ticket = adapter.get_ticket(ticket_key)

print(f"Título: {ticket.summary}")
print(f"Descrição: {ticket.description}")
print(f"ACs existentes: {len(ticket.acceptance_criteria)}")
```

#### Buscar Tickets por Componente

```python
component = "Booking"
tickets = adapter.get_tickets_by_component(component)

print(f"Tickets do componente {component}: {len(tickets)}")
```

### Queries Úteis

#### JQL para Tickets da Sprint Ativa

```jql
sprint in openSprints() AND project = PMS
```

#### JQL para Tickets sem ACs

```jql
project = PMS AND type = Story AND description !~ "Acceptance Criteria"
```

#### JQL para Tickets de Alto Risco

```jql
project = PMS AND component in (Booking, Payment) AND status != Done
```

### Tratamento de Erros

```python
try:
    tickets = adapter.get_sprint_tickets(sprint_id)
except JiraConnectionError as e:
    print(f"Erro de conexão: {e}")
except JiraAuthenticationError as e:
    print(f"Erro de autenticação: {e}")
except Exception as e:
    print(f"Erro inesperado: {e}")
```

## Integração com Splunk

**Status**: ✅ Funcional via Importação Manual de Arquivos (método recomendado)

### Solução Implementada: Importação Manual de Arquivos ✅

**Problema original**:
- ❌ API REST não acessível (porta 8089 fechada)
- ❌ Não há acesso admin para abrir portas
- ❌ Web scraping pode ser mal visto/violar políticas
- ✅ Mas você tem acesso à interface web do Splunk

**Solução**: Processo Manual de Exportação/Importação ✅

**Como funciona**:
1. Você exporta dados do Splunk via interface web (CSV/JSON)
2. Salva os arquivos em `data/splunk_exports/`
3. O sistema processa e gera as métricas automaticamente

**Vantagens**:
- ✅ Transparente e aceitável pela empresa
- ✅ Não viola políticas (você exporta manualmente)
- ✅ Controle total sobre os dados
- ✅ Funciona sem acesso admin
- ✅ Mesma qualidade de dados

**Como funciona**:
1. Tenta conectar via API REST (se configurado)
2. Se falhar, usa web scraping automaticamente
3. Faz login na interface web
4. Executa queries SPL
5. Extrai resultados da tabela

**Baseado na documentação oficial do Splunk:**
- URL correta para Splunk Cloud: `https://<deployment-name>.splunkcloud.com:8089`
- Endpoint de autenticação: `/services/auth/login`
- **Importante**: Pode precisar abrir porta 8089 via support case no Splunk Support Portal
- **Importante**: Free trial accounts **não podem** acessar REST API
- O caminho `/en-GB` é parte da interface web, não da API REST

**Possíveis causas dos erros 404:**
1. **Porta 8089 não está aberta** (precisa abrir via Splunk Support Portal) - **Mais provável**
2. Conta é free trial (não tem acesso à REST API) - **Menos provável** (você tem acesso aos apps)
3. Precisa de VPN ou acesso especial
4. Hostname incorreto (deve ser formato `<deployment-name>.splunkcloud.com`)
5. Firewall bloqueando conexão

### Alternativas de Acesso

#### Opção 1: Importação Manual de Arquivos (✅ RECOMENDADO - Já Implementado)

**Esta é a solução atual e funcional!**

**Processo Manual**:

1. **Exporte dados do Splunk**:
   - Acesse: `https://hostfully.splunkcloud.com/en-GB/app/search/search`
   - Execute a query (veja `docs/GUIA-EXPORTACAO-SPLUNK.md`)
   - Exporte como CSV ou JSON

2. **Salve o arquivo**:
   ```
   data/splunk_exports/metricas_completas.csv
   ```

3. **Processe**:
   ```bash
   python scripts/process_splunk_export.py data/splunk_exports/metricas_completas.csv
   ```

**Configuração**:

```yaml
# configs/splunk_config.yaml
splunk:
  use_file_import: true  # Importar de arquivos (já está ativo)
```

**Uso Programático**:

```python
from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = SplunkAdapter(config.splunk)

# Usa arquivo se disponível, senão tenta API
metrics = adapter.get_critical_metrics(
    file_path="data/splunk_exports/metricas_completas.csv"
)
```

**Vantagens**:
- ✅ Transparente e aceitável pela empresa
- ✅ Não viola políticas
- ✅ Controle total sobre os dados
- ✅ Funciona sem acesso admin
- ✅ Mesma qualidade de dados

**Guia completo**: Veja `docs/GUIA-EXPORTACAO-SPLUNK.md`

#### Opção 2: Abrir Porta 8089 via Support Case (Futuro - Se Quiser API REST)

**Segundo a documentação oficial do Splunk:**
> "If necessary, submit a support case using the Splunk Support Portal to open port 8089 on your deployment."

**Status atual:**
- ✅ Você tem acesso aos apps do Splunk (confirmado)
- ✅ Você tem permissões adequadas
- ❌ Porta 8089 não está acessível externamente (erro 404)

**Ação necessária:**
1. Acesse o Splunk Support Portal: https://support.splunk.com/
2. Abra um support case solicitando:
   - **Abertura da porta 8089 para REST API**
   - **Acesso externo à API REST do Splunk Cloud**
   - Mencione que você precisa para automação/integração via API
3. Após abertura, teste novamente a conexão com:
   ```bash
   python scripts/test_splunk_rest_api.py
   ```

**URL baseada na documentação oficial:**
- Splunk Cloud: `https://<deployment-name>.splunkcloud.com:8089`
- No seu caso: `https://hostfully.splunkcloud.com:8089`

**Nota**: Como você já tem acesso aos apps, o problema é especificamente de acesso à API REST externa, não de permissões.

#### Opção 2: Verificar Tipo de Conta

**Importante**: Free trial accounts do Splunk Cloud **não podem** acessar REST API.

**Ação necessária:**
1. Verifique se sua conta é free trial
2. Se for free trial, considere upgrade ou use alternativas (exportar dados manualmente)

#### Opção 3: Verificar Permissões via Interface Web

**Enquanto a API REST não está acessível, você pode verificar permissões via interface web:**

1. Acesse o Splunk Cloud: `https://hostfully.splunkcloud.com/en-GB/app/search/search`
2. Vá em **Settings > Access Control > Users**
3. Procure seu usuário (`daniel@hostfully.com`)
4. Verifique:
   - **Roles** atribuídas ao usuário
   - **Capabilities** (permissões específicas)
   - **Indexes** que você tem permissão para acessar

**Endpoints da API para verificar permissões (quando a conexão funcionar):**
- `/services/auth/current-context` - Informações do usuário atual
- `/services/authentication/users/current` - Detalhes do usuário
- `/services/authorization/roles` - Roles disponíveis
- `/services/authorization/capabilities` - Capabilities do usuário

**Script de teste de permissões:**
```bash
python scripts/check_splunk_permissions.py
```

Este script tenta acessar vários endpoints para verificar quais permissões você tem.

#### Opção 4: Verificar com Time de Infra

**Ação necessária**: Contatar time de infra para:
1. Verificar se a API REST do Splunk Cloud está acessível externamente
2. Verificar se precisa de VPN ou acesso especial
3. Verificar hostname correto (formato `<deployment-name>.splunkcloud.com`)
4. Verificar se porta 8089 está aberta
5. Verificar permissões do usuário para acessar REST API

#### Opção 4: REST API Direta (Atualizado com Documentação Oficial)

O adapter foi atualizado com base na documentação oficial. Configure:

```yaml
splunk:
  host: "hostfully.splunkcloud.com"  # Formato: <deployment-name>.splunkcloud.com
  port: 8089  # Porta padrão Splunk Cloud (pode precisar abrir via support case)
  scheme: "https"
  use_rest_api: true  # Ativa modo REST API
  authentication:
    type: "basic"
    username: "${SPLUNK_USERNAME}"
    password: "${SPLUNK_PASSWORD}"
  default_index: "main"
  timeout: 300
  verify_ssl: true
```

**Nota**: O adapter agora tenta automaticamente portas 8089 e 443, e fornece mensagens de erro mais detalhadas baseadas na documentação oficial.

#### Opção 3: Usar Token de Autenticação (Se Disponível)

Se conseguir gerar token no Splunk Cloud:

1. Acesse o Splunk Cloud
2. Vá em Settings > Tokens (ou API Tokens)
3. Crie um novo token com permissões de leitura
4. Configure no `.env`:
   ```bash
   SPLUNK_TOKEN=seu-token-aqui
   ```

E no `splunk_config.yaml`:
```yaml
authentication:
  type: "token"
  token: "${SPLUNK_TOKEN}"
```

### Pré-requisitos

- Acesso ao Splunk Cloud
- Username/password ou token de autenticação
- Permissões para executar queries
- Conhecimento do índice onde os logs estão (configurado: `main`, `test`, `sandbox`)

### Configuração

Edite `configs/splunk_config.yaml`:

```yaml
splunk:
  host: "seu-splunk.com"
  port: 8089
  scheme: "https"
  authentication:
    type: "token"
    token: "${SPLUNK_TOKEN}"  # Use variável de ambiente
  default_index: "pms_logs"
  timeout: 300
  verify_ssl: true
```

Configure no `.env`:

```bash
SPLUNK_TOKEN=seu-token-aqui
```

### Exemplos de Uso

#### Métricas Essenciais para Estratégia Reativa

**Objetivo**: Obter métricas críticas a cada ciclo para moldar a estratégia de teste reativa.

```python
from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = SplunkAdapter(config.splunk)

# Métricas críticas consolidadas (essencial para estratégia reativa)
metrics = adapter.get_critical_metrics(time_range="-7d@d")

# Endpoints mais usados (priorizar testes)
most_used = metrics['most_used_endpoints']
print("Endpoints mais usados:")
for endpoint_data in most_used[:10]:
    print(f"  {endpoint_data['endpoint']}: {endpoint_data['total_requests']:,} requisições")

# Endpoints que mais falham (investigar urgentemente)
most_failed = metrics['most_failed_endpoints']
print("\nEndpoints que mais falham:")
for endpoint_data in most_failed[:10]:
    print(f"  {endpoint_data['endpoint']}: {endpoint_data['total_errors']:,} erros ({endpoint_data['error_rate']}%)")

# Endpoints críticos (alto uso + alta taxa de erro)
critical = metrics['critical_endpoints']
print("\nEndpoints críticos:")
for endpoint_data in critical:
    print(f"  {endpoint_data['endpoint']}: {endpoint_data['total_requests']:,} req, {endpoint_data['error_rate']}% erro")
```

#### Endpoints Mais Usados

```python
# Obter endpoints mais usados (por volume)
most_used = adapter.get_most_used_endpoints(time_range="-7d@d", limit=20)

for endpoint_data in most_used:
    endpoint = endpoint_data['endpoint']
    total = endpoint_data['total_requests']
    clients = endpoint_data['unique_clients']
    avg_time = endpoint_data['avg_response_time']
    
    print(f"{endpoint}: {total:,} requisições, {clients} clientes, {avg_time:.0f}ms média")
```

#### Endpoints Que Mais Falham

```python
# Obter endpoints que mais falham
most_failed = adapter.get_most_failed_endpoints(time_range="-7d@d", limit=20)

for endpoint_data in most_failed:
    endpoint = endpoint_data['endpoint']
    errors = endpoint_data['total_errors']
    error_rate = endpoint_data['error_rate']
    
    print(f"{endpoint}: {errors:,} erros ({error_rate}% taxa)")
```

#### Saúde de um Endpoint Específico

```python
# Análise detalhada de um endpoint
health = adapter.get_endpoint_health(
    endpoint="/api/booking",
    time_range="-7d@d"
)

print(f"Endpoint: {health['endpoint']}")
print(f"Status: {health['health_status']}")
print(f"Total: {health['total_requests']:,}")
print(f"Erros: {health['total_errors']:,} ({health['error_rate']}%)")
print(f"Tempo médio: {health['avg_response_time']:.0f}ms")
print(f"P95: {health['p95_response_time']:.0f}ms")
print(f"P99: {health['p99_response_time']:.0f}ms")
```

#### Script de Métricas Reativas

Execute o script dedicado para obter todas as métricas essenciais:

```bash
python scripts/get_reactive_metrics.py
```

Este script mostra:
- Endpoints mais usados (prioridade de teste)
- Endpoints que mais falham (problemas críticos)
- Endpoints críticos (alto uso + alta taxa de erro)
- Recomendações para estratégia reativa

#### Análise de Taxa de Erro

```python
query = """
search index=pms_logs earliest=-7d@d latest=now()
| eval has_error = if(status>=400, 1, 0)
| stats count as total, sum(has_error) as errors by endpoint
| eval error_rate = errors/total
| where total > 10
| sort -error_rate
"""

results = adapter.execute_query(query)
for result in results:
    endpoint = result['endpoint']
    error_rate = float(result['error_rate'])
    print(f"{endpoint}: {error_rate:.2%} de erro")
```

#### Identificar Padrões de Erro

```python
query = """
search index=pms_logs earliest=-24h@h latest=now() status>=400
| stats count by endpoint, error_message
| where count > 5
| sort -count
"""

results = adapter.execute_query(query)
for result in results:
    print(f"{result['endpoint']}: {result['error_message']} ({result['count']} vezes)")
```

### Queries SPL Úteis

#### Top 20 Endpoints com Mais Erros

```splunk
index=pms_logs earliest=-7d@d latest=now() status>=400
| stats count by endpoint
| sort -count
| head 20
```

#### Análise de Latência

```splunk
index=pms_logs earliest=-24h@h latest=now()
| stats avg(response_time) as avg_time, max(response_time) as max_time by endpoint
| where avg_time > 2000
| sort -avg_time
```

#### Comparação Entre Períodos

```splunk
index=pms_logs earliest=-14d@d latest=-7d@d
| eval period="anterior"
| append [search index=pms_logs earliest=-7d@d latest=now() | eval period="atual"]
| stats count by endpoint, period
| xyseries endpoint period count
```

### Tratamento de Erros

```python
try:
    results = adapter.execute_query(query)
except SplunkConnectionError as e:
    print(f"Erro de conexão: {e}")
except SplunkQueryError as e:
    print(f"Erro na query: {e}")
except SplunkTimeoutError as e:
    print(f"Query expirou: {e}")
except Exception as e:
    print(f"Erro inesperado: {e}")
```

## Integração com Postman

**Status**: ✅ Funcionando e testado (52 collections encontradas)

### Pré-requisitos

- Conta no Postman
- API Key
- Workspace ID

### Obter API Key

1. Acesse: https://www.postman.com/settings/me/api-keys
2. Clique em "Generate API Key"
3. Dê um nome descritivo
4. Copie a API Key

**Nota**: A API Key configurada está funcionando corretamente.

### Obter Workspace ID

1. Acesse seu workspace no Postman
2. A URL terá o formato: `https://hostfully.postman.co/workspace/Team-Workspace~{workspace-id}/overview`
3. Copie o workspace-id (ex: `f17f099b-6a03-442f-8c8e-8ac905447030`)

**Nota**: O Workspace ID configurado está funcionando corretamente.

### Configuração

Edite `configs/postman_config.yaml`:

```yaml
postman:
  api_key: "${POSTMAN_API_KEY}"  # Use variável de ambiente
  workspace_id: "seu-workspace-id"
  default_collection_name: "Testes Gerados"
  base_url_variable: "{{base_url}}"
  timeout: 30
```

Configure no `.env`:

```bash
POSTMAN_API_KEY=seu-api-key-aqui
```

### Exemplos de Uso

#### Criar Collection

```python
from src.infrastructure.adapters.postman_adapter import PostmanAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = PostmanAdapter(config.postman)

# Criar nova collection
collection_data = {
    "info": {
        "name": "Testes - Sprint 123",
        "description": "Testes gerados automaticamente"
    },
    "item": []
}

collection = adapter.create_collection(collection_data)
print(f"Collection criada: {collection['id']}")
```

#### Adicionar Request à Collection

```python
request_data = {
    "name": "GET /api/booking",
    "request": {
        "method": "GET",
        "header": [],
        "url": {
            "raw": "{{base_url}}/api/booking",
            "host": ["{{base_url}}"],
            "path": ["api", "booking"]
        }
    },
    "event": [{
        "listen": "test",
        "script": {
            "exec": [
                "pm.test('Status code is 200', function () {",
                "    pm.response.to.have.status(200);",
                "});"
            ]
        }
    }]
}

adapter.add_request_to_collection(collection_id, request_data)
```

#### Executar Collection

```python
# Executar collection via Newman (CLI)
from subprocess import run

run([
    "newman",
    "run",
    f"https://api.getpostman.com/collections/{collection_id}",
    "--api-key", config.postman.api_key,
    "--environment", "environment.json"
])
```

### Estrutura de Collection

```json
{
  "info": {
    "name": "Collection Name",
    "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
  },
  "item": [
    {
      "name": "Request Name",
      "request": {
        "method": "GET",
        "header": [],
        "url": {
          "raw": "{{base_url}}/endpoint",
          "host": ["{{base_url}}"],
          "path": ["endpoint"]
        }
      },
      "response": []
    }
  ],
  "variable": [
    {
      "key": "base_url",
      "value": "https://api.exemplo.com"
    }
  ]
}
```

### Tratamento de Erros

```python
try:
    collection = adapter.create_collection(collection_data)
except PostmanAuthenticationError as e:
    print(f"Erro de autenticação: {e}")
except PostmanAPIError as e:
    print(f"Erro na API: {e}")
except Exception as e:
    print(f"Erro inesperado: {e}")
```

### Integração via Interface Web

A interface web fornece integração completa com Postman e Testmo:

**Status**: ✅ Funcionando

**Funcionalidades**:
- Busca automática de matches no Postman para endpoints identificados
- Visualização de matches com informações detalhadas
- Sincronização direta com Testmo
- Sistema de herança (Base → Reativo)

**Como usar**:
1. Processe um arquivo CSV/JSON do Splunk na interface web
2. Após processamento, clique em "Buscar Matches no Postman"
3. Revise os matches encontrados
4. Selecione os test cases desejados
5. Clique em "Sincronizar com Testmo"

Veja [Guia da Interface Web](08-interface-web.md) para detalhes completos.

## Integração com Testmo

**Status**: ✅ Funcionando e testado

### Pré-requisitos

- Conta no Testmo
- API Token
- Project ID

### Obter API Token

1. Acesse: Settings → API Tokens no Testmo
2. Clique em "Create Token"
3. Copie o token gerado

### Configuração

Edite `configs/testmo_config.yaml`:

```yaml
testmo:
  url: "https://seu-instance.testmo.net"
  token: "${TESTMO_TOKEN}"  # Use variável de ambiente
  default_project_id: 1
  timeout: 30
```

Configure no `.env`:

```bash
TESTMO_TOKEN=seu-token-aqui
```

### Funcionalidades Implementadas

- ✅ Criação de test cases
- ✅ Atualização de test cases
- ✅ Busca de test cases existentes
- ✅ Sistema de herança (Base → Reativo)
- ✅ Criação de estrutura de pastas
- ✅ Comparação de test cases
- ✅ Sincronização via interface web

### Integração via Interface Web

A interface web oferece sincronização completa:

**Endpoint**: `/reactive/sync-test-cases`

**Fluxo**:
1. Buscar matches no Postman (`/reactive/find-postman-matches/<snapshot_id>`)
2. Verificar existência no Testmo
3. Comparar e sugerir atualizações
4. Sincronizar selecionados
5. Criar estrutura reativa com herança do Base

**Recursos**:
- Verificação automática de test cases existentes
- Sugestão de atualizações quando diferentes
- Criação de estrutura organizada (Base/Reativo)
- Herança de casos base para casos reativos
- Estatísticas de sincronização (criados, atualizados, herdados, erros)

Veja [TESTMO-CLI-INTEGRACAO.md](TESTMO-CLI-INTEGRACAO.md) e [TESTMO-NOMENCLATURA-ESTRUTURA.md](TESTMO-NOMENCLATURA-ESTRUTURA.md) para detalhes sobre estrutura e nomenclatura.

## Integração com Playwright

### Instalação

```bash
pip install playwright
playwright install chromium
```

### Configuração

Não requer configuração especial, mas pode configurar opções:

```python
from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch(
        headless=False,  # Ver browser durante execução
        slow_mo=1000     # Desacelerar ações (útil para debug)
    )
    page = browser.new_page()
    # ... usar page
    browser.close()
```

### Exemplos de Uso

#### Gravar Ações

```python
from src.infrastructure.adapters.playwright_engine import PlaywrightEngine

engine = PlaywrightEngine()

# Iniciar gravação
engine.start_recording("session_123")

# Navegar
engine.navigate("https://app.exemplo.com")

# Clicar
engine.click("button#submit")

# Preencher
engine.fill("input#username", "usuario")

# Parar gravação
script = engine.stop_recording()
print(script)  # Script Playwright gerado
```

#### Gerar Script

```python
# O engine gera automaticamente scripts Playwright
script = """
from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch()
    page = browser.new_page()
    page.goto('https://app.exemplo.com')
    page.click('button#submit')
    page.fill('input#username', 'usuario')
    browser.close()
"""
```

## Integração Completa - Exemplo

Exemplo de uso integrado de todas as ferramentas:

```python
from src.application.preventivo import PreventiveService
from src.application.reativo import ReactiveService
from src.infrastructure.config.load_config import load_config

config = load_config()

# 1. Análise preventiva
preventive = PreventiveService()
sprint_analysis = preventive.analyze_upcoming_sprint("123")

# 2. Análise reativa
reactive = ReactiveService()
log_analysis = reactive.analyze_production_logs(timedelta(days=7))

# 3. Correlacionar
for ticket in sprint_analysis.tickets:
    # Buscar padrões relacionados ao componente do ticket
    related_patterns = [
        p for p in log_analysis.patterns
        if any(c in p.affected_endpoints for c in ticket.components)
    ]
    
    if related_patterns:
        # Ajustar risco baseado em padrões reais
        ticket.risk_level = adjust_risk_based_on_patterns(
            ticket.risk_level,
            related_patterns
        )

# 4. Gerar testes baseados em tudo
test_cases = generate_test_cases(
    sprint_analysis,
    log_analysis
)

# 5. Criar collection Postman
from src.infrastructure.adapters.postman_adapter import PostmanAdapter
postman = PostmanAdapter(config.postman)
collection = postman.create_collection_from_test_cases(test_cases)
```

## Troubleshooting

### Jira

**Erro 401 (Unauthorized)**
- Verifique se o API token está correto
- Verifique se o username está correto
- Verifique permissões no Jira

**Erro 403 (Forbidden)**
- Verifique permissões do usuário no projeto
- Verifique se tem acesso ao board/Sprint

### Splunk

**Timeout na porta 8089**
- **Ação**: Abra um support case no Splunk Support Portal para abrir porta 8089
- Tente usar REST API alternativa: configure `use_rest_api: true` no `splunk_config.yaml`
- Verifique se precisa de VPN ou acesso especial
- Teste porta 443 em vez de 8089 (adapter tenta automaticamente)

**Erro 404 (Endpoint não encontrado)**
- **Causa mais comum**: Porta 8089 não está aberta
- **Ação**: Abra um support case no Splunk Support Portal
- Verifique se a conta não é free trial (free trial não tem acesso à REST API)
- Verifique se o hostname está correto (formato: `<deployment-name>.splunkcloud.com`)

**Erro 401 (Unauthorized)**
- Verifique se username/password estão corretos
- **Importante**: Free trial accounts não podem acessar REST API
- Verifique permissões do usuário no Splunk Cloud

**Erro 403 (Forbidden)**
- Verifique permissões do usuário no Splunk Cloud
- Verifique se o usuário tem permissão para executar queries

**REST API alternativa não funciona**
- Verifique se username/password estão corretos
- Verifique se o host está correto (formato: `<deployment-name>.splunkcloud.com`)
- Verifique se porta 8089 está aberta (pode precisar abrir via support case)
- Verifique se a conta não é free trial
- Consulte mensagens de erro detalhadas do adapter

### Postman

**Erro 401 (Unauthorized)**
- Verifique se a API key está correta
- Verifique se a API key não expirou

**Erro ao criar collection**
- Verifique se o workspace_id está correto
- Verifique permissões no workspace

## Integração com Testmo

**Status**: ✅ Funcional (API REST + CLI)

### Visão Geral

O Testmo serve como repositório central de test cases, com três camadas de integração:

1. **API REST**: Gerenciamento de dados (CRUD completo)
2. **Testmo CLI**: Automação e submissão de resultados
3. **Interface Web**: Configuração e visualização (manual)

### Configuração

Edite `configs/testmo_config.yaml`:

```yaml
testmo:
  base_url: "https://seu-dominio.testmo.net"
  api_key: "${TESTMO_API_KEY}"
  default_project_id: 1
```

Configure no `.env`:

```bash
TESTMO_API_KEY=sua-chave-aqui
```

### Estrutura de Repositórios

O sistema implementa dois repositórios principais:

#### Repositório Base (Reutilizável)
- **Estrutura**: `Base/{Component}/{METHOD}_{Endpoint}/{METHOD}_{TestType}_{Description}`
- **Uso**: Testes reutilizáveis organizados por componente
- **Exemplo**: `Base/Booking/POST_api-v3-quotes/POST_CreateQuote_ValidRequest`

#### Repositório Reativo (Sprint-based)
- **Estrutura**: `Reativo/{Date}_{Priority}_{Trend}/{METHOD}_{Endpoint}/{METHOD}_{Endpoint}_{Priority}_{Date}`
- **Uso**: Testes focados em problemas atuais
- **Exemplo**: `Reativo/2025-01-15_Critical_Degrading/POST_api-v3-quotes/POST_api-v3-quotes_Critical_2025-01-15`

**Documentação completa**: Veja `docs/TESTMO-NOMENCLATURA-ESTRUTURA.md`

### Convenções de Nomenclatura

**Test Cases**: `{METHOD}_{TestType}_{Description}`
- Ex: `POST_CreateQuote_ValidRequest`
- Ex: `GET_GetQuote_NotFound`

**Pastas Endpoint**: `{METHOD}_{NormalizedPath}`
- Ex: `POST_api-v3-quotes`

**Pastas Reativas**: `{YYYY-MM-DD}_{Priority}_{Trend}`
- Ex: `2025-01-15_Critical_Degrading`

### Exemplos de Uso

#### Criar Test Case no Base

```python
from src.infrastructure.adapters.testmo_adapter import TestmoAdapter
from src.application.shared.testmo_structure_service import TestmoStructureService

testmo = TestmoAdapter(config.testmo)
structure = TestmoStructureService(testmo)

# Garantir estrutura
base_structure = structure.ensure_base_structure(
    project_id=1,
    component="Booking",
    endpoint="/api/v3/quotes",
    method="POST"
)

# Criar test case
test_case = testmo.create_test_case(
    title="POST_CreateQuote_ValidRequest",
    description="Test creating a valid quote",
    project_id=1,
    priority="high",
    steps=["1. Send POST request", "2. Verify status 200"],
    expected_result="Quote created successfully"
)
```

#### Herdar Caso do Base para Reativo

```python
from src.application.reativo.test_case_inheritance_service import TestCaseInheritanceService

inheritance = TestCaseInheritanceService(testmo, structure)

# Buscar caso no Base
base_case = inheritance.find_base_case(
    project_id=1,
    component="Booking",
    endpoint="/api/v3/quotes",
    method="POST"
)

# Herdar para Reativo
reactive_case = inheritance.inherit_to_reactive(
    base_case=base_case,
    reactive_context={
        'endpoint': '/api/v3/quotes',
        'method': 'POST',
        'priority': 'Critical',
        'date': '2025-01-15',
        'error_rate': 15.5
    },
    project_id=1,
    reactive_folder_id=123
)
```

#### Submeter Resultados via CLI

```python
from src.application.reativo.testmo_cli_integration import TestmoCLIIntegration

cli = TestmoCLIIntegration()

result = cli.submit_reactive_analysis_results(
    project_id=1,
    analysis_results={
        'snapshot_id': 'abc123',
        'prioritized_recommendations': [...],
        'regression_risks': [...]
    },
    run_name="Reactive Analysis - 2025-01-15"
)
```

### Tratamento de Erros

```python
try:
    test_case = testmo.create_test_case(...)
except TestmoAPIError as e:
    print(f"Erro na API: {e}")
except Exception as e:
    print(f"Erro inesperado: {e}")
```

### Documentação Adicional

- **Nomenclatura e Estrutura**: `docs/TESTMO-NOMENCLATURA-ESTRUTURA.md`
- **CLI Integration**: `docs/TESTMO-CLI-INTEGRACAO.md`
- **Estratégia Completa**: `docs/TESTMO-ESTRATEGIA-COMPLETA.md`

## Próximos Passos

1. Configure todas as integrações
2. Teste cada integração isoladamente
3. Teste integrações combinadas
4. Configure monitoramento e alertas

