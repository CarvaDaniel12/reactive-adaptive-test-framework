# Modelos de Dados

Este documento descreve todos os modelos de dados do sistema, incluindo entidades, value objects e estruturas de dados.

## Entidades

### Ticket

Representa um ticket do Jira que será analisado pelo Preventive Service.

```python
@dataclass
class Ticket:
    key: str                    # Ex: "PMS-123"
    summary: str                # Título do ticket
    description: str            # Descrição completa
    issue_type: str            # "Story", "Bug", "Task", etc
    status: str                # "To Do", "In Progress", "Done"
    components: List[str]       # Componentes afetados
    acceptance_criteria: List[str]  # ACs existentes
    risk_level: RiskLevel       # Nível de risco calculado
    complexity_score: float     # Score de complexidade (0.0-1.0)
    created_date: datetime
    updated_date: datetime
    assignee: Optional[str]    # Responsável
    reporter: str              # Quem criou
```

**Campos importantes**:
- `key`: Identificador único do ticket
- `risk_level`: Calculado pelo Risk Analyzer
- `complexity_score`: Baseado em histórico e características

### TestCase

Representa um caso de teste, seja gerado automaticamente ou criado manualmente.

```python
@dataclass
class TestCase:
    id: str                     # ID único do teste (Testmo ID)
    title: str                  # Título do teste (seguindo convenção)
    description: str            # Descrição detalhada
    priority: TestPriority      # Prioridade do teste
    type: str                   # "API", "Integration", "UI", "Stress"
    steps: List[str]            # Passos do teste
    expected_result: str        # Resultado esperado
    automatizable: bool         # Se pode ser automatizado
    component: str              # Componente relacionado (normalizado)
    endpoint: Optional[str]     # Endpoint relacionado (normalizado)
    method: Optional[str]       # Método HTTP (GET, POST, etc)
    ticket_key: Optional[str]   # Ticket relacionado (para fluxo preventivo)
    repository: str             # "Base", "Reativo", ou "Sprint-{ID}"
    folder_path: List[str]      # Caminho da pasta no Testmo
    base_case_id: Optional[int] # ID do caso base (se herdado)
    tags: List[str]             # Tags (incluindo links de herança)
    created_date: datetime
    last_executed: Optional[datetime]
    execution_count: int        # Quantas vezes foi executado
    success_rate: float         # Taxa de sucesso (0.0-1.0)
```

**Campos importantes**:
- `automatizable`: Indica se o teste pode ser automatizado
- `success_rate`: Histórico de execuções bem-sucedidas
- `type`: Tipo de teste determina como será executado
- `repository`: Indica em qual repositório está ("Base" ou "Reativo")
- `base_case_id`: Link para caso base se este foi herdado
- `folder_path`: Caminho completo da pasta no Testmo (ex: ["Base", "Booking", "POST_api-v3-quotes"])

**Convenção de Nomenclatura**:
- Base: `{METHOD}_{TestType}_{Description}`
- Preventivo: `{TICKET_KEY}_{METHOD}_{TestType}_{Description}`
- Reativo: `{METHOD}_{Endpoint}_{Priority}_{Date}`

### LogPattern

Representa um padrão identificado nos logs do Splunk.

```python
@dataclass
class LogPattern:
    pattern_id: str            # ID único do padrão
    description: str            # Descrição do padrão
    frequency: int              # Frequência de ocorrência
    severity: str               # "LOW", "MEDIUM", "HIGH", "CRITICAL"
    affected_endpoints: List[str]  # Endpoints afetados
    suggested_action: str       # Ação sugerida
    confidence: float           # Confiança na identificação (0.0-1.0)
    first_seen: datetime        # Primeira ocorrência
    last_seen: datetime         # Última ocorrência
    sample_logs: List[str]      # Exemplos de logs
```

**Campos importantes**:
- `confidence`: Quão confiável é a identificação do padrão
- `severity`: Determina prioridade de ação
- `affected_endpoints`: Endpoints que precisam de atenção

### Anomaly

Representa uma anomalia detectada em execuções de workflow.

```rust
pub struct Anomaly {
    pub id: Uuid,
    pub detected_at: DateTime<Utc>,
    pub anomaly_type: AnomalyType,  // SpikeInFailures, PerformanceDegradation, etc.
    pub severity: AnomalySeverity,  // Info, Warning, Critical
    pub description: String,
    pub metrics: AnomalyMetrics,
    pub affected_entities: Vec<String>,  // Workflow IDs, ticket IDs
    pub investigation_steps: Vec<String>,
}

pub struct AnomalyMetrics {
    pub current_value: f64,
    pub baseline_value: f64,
    pub deviation: f64,
    pub z_score: f64,  // Standard deviations from mean
    pub confidence: f64,  // 0.0 to 1.0
}

pub enum AnomalyType {
    SpikeInFailures,           // Sudden spike in failures
    PerformanceDegradation,     // Execution time > baseline + 2σ
    UnusualExecutionTime,       // Z-score > 2.0 (too fast or too slow)
    PatternDeviation,           // Deviation from normal behavior
    ResourceExhaustion,         // Resource exhaustion detected
    ConsecutiveFailures,        // 3+ consecutive failures
}

pub enum AnomalySeverity {
    Info,      // Informational anomaly
    Warning,   // Attention required
    Critical,  // Immediate action needed
}
```

**Campos importantes**:
- `z_score`: Medida estatística de desvio (valores > 2.0 indicam anomalia)
- `baseline_value`: Valor médio histórico (média móvel das últimas 30 execuções)
- `confidence`: Nível de confiança na detecção (baseado no z-score)
- `investigation_steps`: Passos sugeridos para investigação

**Tabela no Banco de Dados**:
```sql
CREATE TABLE anomalies (
    id UUID PRIMARY KEY,
    workflow_instance_id UUID,
    anomaly_type VARCHAR(50),
    severity VARCHAR(20),
    description TEXT,
    metrics JSONB,
    affected_entities TEXT[],
    investigation_steps TEXT[],
    detected_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ
);
```

### BaselineMetrics

Métricas de baseline para detecção estatística de anomalias.

```rust
pub struct BaselineMetrics {
    pub failure_rate: MovingAverage,  // Moving average (30-day window)
    pub execution_time: MovingAverage,
    pub success_rate: MovingAverage,
}

pub struct MovingAverage {
    pub value: f64,        // Current mean
    pub std_dev: f64,      // Standard deviation
    pub window_size: usize, // 30 days
    pub history: Vec<f64>,  // Last N values
}
```

**Campos importantes**:
- `window_size`: 30 execuções (representa ~30 dias)
- `std_dev`: Calculado para permitir cálculo de z-score
- Baseline é carregado do histórico do banco de dados a cada detecção

## Value Objects

### RiskLevel

Representa o nível de risco de um ticket ou componente.

```python
class RiskLevel(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"
```

**Valores**:
- `LOW`: Risco baixo, testes básicos suficientes
- `MEDIUM`: Risco médio, testes padrão necessários
- `HIGH`: Risco alto, testes extensivos necessários
- `CRITICAL`: Risco crítico, testes completos obrigatórios

**Cálculo**: Baseado em histórico de bugs, complexidade e componentes afetados.

### TestPriority

Representa a prioridade de execução de um teste.

```python
class TestPriority(Enum):
    P0 = "p0"      # Crítico - executar sempre
    P1 = "p1"      # Alto - executar frequentemente
    P2 = "p2"      # Médio - executar regularmente
    P3 = "p3"      # Baixo - executar ocasionalmente
```

**Valores**:
- `P0`: Testes críticos que devem sempre passar
- `P1`: Testes importantes para funcionalidades principais
- `P2`: Testes para funcionalidades secundárias
- `P3`: Testes para edge cases e validações extras

## Estruturas de Resposta de APIs

### Jira API Response

Estrutura de resposta do Jira ao buscar tickets.

```python
@dataclass
class JiraTicketResponse:
    expand: str
    startAt: int
    maxResults: int
    total: int
    issues: List[JiraIssue]
```

### Splunk Query Response

Estrutura de resposta do Splunk ao executar queries.

```python
@dataclass
class SplunkQueryResponse:
    preview: bool
    offset: int
    result: List[Dict[str, Any]]  # Resultados da query
    fields: List[Dict[str, Any]]  # Campos disponíveis
```

### Postman Collection Response

Estrutura de resposta do Postman ao criar/atualizar collections.

```python
@dataclass
class PostmanCollectionResponse:
    collection: Dict[str, Any]  # Collection completa
    id: str
    name: str
    uid: str
```

## Schemas de Configuração

### Jira Config

```yaml
jira:
  base_url: "https://seu-jira.atlassian.net"
  api_version: "3"
  authentication:
    type: "basic"  # ou "oauth"
    username: "${JIRA_USERNAME}"
    api_token: "${JIRA_API_TOKEN}"
  default_fields:
    - "summary"
    - "description"
    - "components"
    - "status"
    - "issuetype"
```

### Splunk Config

```yaml
splunk:
  host: "seu-splunk.com"
  port: 8089
  scheme: "https"
  authentication:
    type: "token"
    token: "${SPLUNK_TOKEN}"
  default_index: "pms_logs"
  timeout: 300
```

### Postman Config

```yaml
postman:
  api_key: "${POSTMAN_API_KEY}"
  workspace_id: "seu-workspace-id"
  default_collection_name: "Testes Gerados"
  base_url_variable: "{{base_url}}"
```

## Estruturas de Análise

### SprintAnalysis

Resultado da análise preventiva de uma Sprint.

```python
@dataclass
class SprintAnalysis:
    sprint_id: str
    sprint_name: str
    tickets: List[Ticket]
    risk_assessment: RiskAssessment
    test_strategy: TestStrategy
    generated_date: datetime
```

### RiskAssessment

Avaliação de risco da Sprint.

```python
@dataclass
class RiskAssessment:
    overall_risk: RiskLevel
    high_risk_tickets: List[Ticket]
    medium_risk_tickets: List[Ticket]
    low_risk_tickets: List[Ticket]
    risk_factors: Dict[str, float]  # Fatores que contribuem para o risco
```

### TestStrategy

Estratégia de testes sugerida para a Sprint.

```python
@dataclass
class TestStrategy:
    total_tests_suggested: int
    api_tests: int
    integration_tests: int
    ui_tests: int
    priority_distribution: Dict[TestPriority, int]
    estimated_time: timedelta
```

### LogAnalysis

Resultado da análise reativa de logs.

```python
@dataclass
class LogAnalysis:
    time_window: TimeRange
    patterns: List[LogPattern]
    alerts: List[Alert]
    recommendations: List[Recommendation]
    analyzed_endpoints: List[str]
    total_errors: int
    error_rate: float
```

### Alert

Alerta gerado pela análise de logs.

```python
@dataclass
class Alert:
    alert_id: str
    severity: str
    title: str
    description: str
    affected_endpoints: List[str]
    suggested_tests: List[TestCase]
    created_date: datetime
```

### Recommendation

Recomendação baseada em análise.

```python
@dataclass
class Recommendation:
    type: str  # "test", "monitor", "investigate"
    priority: TestPriority
    description: str
    action_items: List[str]
    related_patterns: List[str]
```

## Estruturas de Gravação

### QASession

Sessão de teste gravada pelo QA Agent.

```python
@dataclass
class QASession:
    session_id: str
    ticket_key: str
    qa_name: str
    start_time: datetime
    end_time: Optional[datetime]
    actions: List[QAAction]
    screenshots: List[str]  # Paths para screenshots
    final_url: Optional[str]
    automation_script: Optional[str]  # Script Playwright gerado
```

### QAAction

Ação individual gravada durante sessão de teste.

```python
@dataclass
class QAAction:
    action_id: str
    action_type: str  # "click", "fill", "navigate", "wait"
    timestamp: datetime
    element: Optional[str]  # Seletor ou descrição do elemento
    value: Optional[str]  # Valor preenchido (sem dados sensíveis)
    url: str
    screenshot: Optional[str]  # Screenshot após ação
```

## Persistência

### Formato de Armazenamento

**Tickets**: JSON files em `data/tickets/`
**Test Cases**: JSON files em `data/test_cases/`
**Log Patterns**: JSON files em `data/patterns/`
**Sessions**: JSON files em `data/sessions/`

### Estrutura de Arquivo

```json
{
  "metadata": {
    "version": "1.0",
    "created_at": "2025-01-15T10:00:00Z",
    "updated_at": "2025-01-15T10:00:00Z"
  },
  "data": {
    // Dados da entidade
  }
}
```

## Validações

### Regras de Validação

- **Ticket**: `key` deve seguir padrão do Jira (ex: "PMS-123")
- **TestCase**: `priority` deve ser válido, `steps` não pode estar vazio
- **LogPattern**: `confidence` deve estar entre 0.0 e 1.0
- **RiskLevel**: Deve ser um dos valores do enum
- **TestPriority**: Deve ser um dos valores do enum

### Validações de Negócio

- Ticket sem ACs deve ter risco aumentado
- TestCase sem `expected_result` não pode ser automatizado
- LogPattern com `confidence` < 0.5 deve ser marcado como "low confidence"

## Evolução dos Modelos

Os modelos podem evoluir conforme o projeto cresce:

1. **Fase 1 (MVP)**: Modelos básicos acima
2. **Fase 2**: Adicionar relacionamentos entre entidades
3. **Fase 3**: Adicionar histórico e versionamento
4. **Fase 4**: Migrar para banco de dados se necessário

