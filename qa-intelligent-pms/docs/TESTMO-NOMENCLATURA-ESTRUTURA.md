# Sistema de Nomenclatura e Estrutura Testmo

## Visão Geral

O sistema implementa uma estrutura organizada e padronizada para gerenciar test cases no Testmo, com dois repositórios principais:

1. **Repositório Base**: Testes reutilizáveis organizados por componente
2. **Repositório Reativo**: Testes focados em problemas atuais, organizados por criticidade/tendência

## Estrutura de Repositórios

### Repositório Base (Reutilizável)

**Organização**: Por componente (estrutura estável)

**Estrutura de Pastas**:
```
Base/
  ├── {Component}/
  │   ├── {METHOD}_{NormalizedEndpoint}/
  │   │   ├── {METHOD}_{TestType}_{Description}
  │   │   └── ...
```

**Exemplo Real**:
```
Base/
  ├── Booking/
  │   ├── POST_api-v3-quotes/
  │   │   ├── POST_CreateQuote_ValidRequest
  │   │   ├── POST_CreateQuote_InvalidData
  │   │   └── POST_CreateQuote_Unauthorized
  │   └── GET_api-v3-quotes/
  │       ├── GET_GetQuote_ById
  │       └── GET_GetQuote_NotFound
  ├── Payment/
  │   └── POST_api-v3-payments/
  │       └── POST_ProcessPayment_ValidCard
```

**Quando usar**: Testes que são reutilizáveis e servem como base de conhecimento

### Repositório Reativo (Sprint-based)

**Organização**: Por prioridade/criticidade (estrutura dinâmica)

**Estrutura de Pastas**:
```
Reativo/
  ├── {YYYY-MM-DD}_{Priority}_{Trend}/
  │   ├── {METHOD}_{NormalizedEndpoint}/
  │   │   ├── {METHOD}_{Endpoint}_{Priority}_{Date}
  │   │   └── ...
```

**Exemplo Real**:
```
Reativo/
  ├── 2025-01-15_Critical_Degrading/
  │   ├── POST_api-v3-quotes/
  │   │   ├── POST_api-v3-quotes_Critical_2025-01-15
  │   │   └── ...
  │   └── GET_api-v3-bookings/
  │       └── GET_api-v3-bookings_High_2025-01-15
  ├── 2025-01-20_High_Stable/
  │   └── PUT_api-v3-reservations/
  │       └── PUT_api-v3-reservations_High_2025-01-20
```

**Quando usar**: Testes focados em problemas identificados na análise reativa

**Ciclo de vida**: 
- Criado a cada análise reativa
- Migrado para Base ao final da sprint (casos úteis)
- Deletado após merge bem-sucedido

### Repositório Preventivo (Sprint-based)

**Organização**: Por sprint e componente

**Estrutura de Pastas**:
```
Sprint-{ID}/
  ├── {Component}/
  │   ├── {METHOD}_{NormalizedEndpoint}/
  │   │   ├── {TICKET_KEY}_{METHOD}_{TestType}_{Description}
  │   │   └── ...
```

**Exemplo Real**:
```
Sprint-123/
  ├── Booking/
  │   ├── POST_api-v3-quotes/
  │   │   ├── PMS-456_POST_CreateQuote_ValidRequest
  │   │   └── PMS-457_POST_CreateQuote_WithDiscount
  │   └── GET_api-v3-bookings/
  │       └── PMS-458_GET_GetBooking_ById
```

**Quando usar**: Testes gerados a partir de tickets do Jira antes da sprint

## Convenções de Nomenclatura

### Test Cases

**Formato**: `{METHOD}_{TestType}_{Description}`

- `METHOD`: HTTP method (GET, POST, PUT, DELETE, PATCH)
- `TestType`: Tipo de teste (Create, Update, Delete, Get, Validate, Error)
- `Description`: Descrição curta (PascalCase, sem espaços)

**Exemplos**:
- `POST_CreateQuote_ValidRequest`
- `GET_GetQuote_NotFound`
- `PUT_UpdateBooking_InvalidStatus`
- `DELETE_DeleteReservation_Unauthorized`

**Para fluxo preventivo**: `{TICKET_KEY}_{METHOD}_{TestType}_{Description}`
- Ex: `PMS-123_POST_CreateQuote_ValidRequest`

**Para fluxo reativo**: `{METHOD}_{Endpoint}_{Priority}_{Date}`
- Ex: `POST_api-v3-quotes_Critical_2025-01-15`

### Pastas de Endpoint

**Formato**: `{METHOD}_{NormalizedPath}`

- `METHOD`: HTTP method (GET, POST, PUT, DELETE, PATCH)
- `NormalizedPath`: Path normalizado convertido para slug (ex: `/api/v3/quotes` → `api-v3-quotes`)

**Exemplos**:
- `POST_api-v3-quotes`
- `GET_api-v3-bookings`
- `PUT_api-v3-reservations`

### Pastas Reativas

**Formato**: `{YYYY-MM-DD}_{Priority}_{Trend}`

- `YYYY-MM-DD`: Data da análise
- `Priority`: Critical, High, Medium, Low
- `Trend`: Degrading, Improving, Stable, New

**Exemplos**:
- `2025-01-15_Critical_Degrading`
- `2025-01-15_High_Stable`
- `2025-01-20_Medium_Improving`

### Pastas de Sprint

**Formato**: `Sprint-{ID}`

- `ID`: ID da sprint (normalizado, sem caracteres especiais)

**Exemplos**:
- `Sprint-123`
- `Sprint-2025-Q1-Sprint1`

## Normalização

### Componentes

Componentes do Jira são normalizados para PascalCase:

- `"Booking Service"` → `"Booking"`
- `"booking-service"` → `"Booking"`
- `"payment_service"` → `"Payment"`

**Mapeamentos customizáveis**: `configs/component_mappings.yaml`

### Endpoints

Endpoints são normalizados para matching e criação de slugs:

- Remove query parameters: `/api/v3/quotes?param=value` → `/api/v3/quotes`
- Remove trailing slashes: `/api/v3/quotes/` → `/api/v3/quotes`
- Converte para slug: `/api/v3/quotes` → `api-v3-quotes`

## Validação e Parse

### Regex de Validação

**Test Case**: `^[A-Z]+_[A-Z][a-zA-Z0-9]+_[A-Z][a-zA-Z0-9]+$`

**Pasta Endpoint**: `^[A-Z]+_[a-z0-9-]+$`

**Pasta Reativa**: `^\d{4}-\d{2}-\d{2}_[A-Z][a-z]+_[A-Z][a-z]+$`

**Pasta Sprint**: `^Sprint-[\w\-]+$`

### Parser de Nomes

O sistema inclui `NameParser` que extrai componentes de nomes:

```python
from src.application.shared.name_parser import NameParser

# Parse test case name
parsed = NameParser.parse_test_case_name("POST_CreateQuote_ValidRequest")
# Returns: {'method': 'POST', 'test_type': 'CreateQuote', 'description': 'ValidRequest'}

# Validate format
is_valid = NameParser.validate_test_case_name("POST_CreateQuote_ValidRequest")
# Returns: True
```

## Herança de Test Cases

### Conceito

Casos do repositório Reativo podem herdar estrutura de casos do Base:

1. **Busca**: Sistema busca caso similar no Base
2. **Herança**: Cria novo caso no Reativo herdando:
   - Steps
   - Expected results
   - Estrutura básica
3. **Contexto**: Adiciona contexto específico:
   - Data da análise
   - Prioridade
   - Métricas (error rate, total errors)
4. **Link**: Mantém referência ao caso base via tags

### Exemplo de Herança

**Caso Base**:
- Título: `POST_CreateQuote_ValidRequest`
- Steps: ["1. Enviar requisição POST", "2. Validar status 200"]
- Expected: "Quote criado com sucesso"

**Caso Reativo Herdado**:
- Título: `POST_api-v3-quotes_Critical_2025-01-15`
- Steps: 
  - "Contexto: Análise reativa de 2025-01-15 identificou este endpoint como Critical priority"
  - "1. Enviar requisição POST"
  - "2. Validar status 200"
- Expected: "Quote criado com sucesso"
- Tags: `['inherited', 'reactive', 'base-case:123', 'date:2025-01-15', 'priority:critical']`

## Processo de Merge

Ao final de cada sprint, casos reativos são analisados e migrados:

### Critérios para Migração

1. **Casos únicos**: Não existem no Base
2. **Casos melhorados**: Têm melhor taxa de sucesso que versão no Base
3. **Casos executados**: Foram executados com sucesso (success_rate > 80%)

### Processo

1. **Análise**: `ReactiveMergeService.analyze_reactive_cases()`
2. **Identificação**: `ReactiveMergeService.identify_candidates_for_merge()`
3. **Migração**: `ReactiveMergeService.merge_to_base()`
4. **Limpeza**: `ReactiveMergeService.cleanup_reactive_folder()`

### Resultado

- Casos úteis migrados para Base
- Estrutura reativa deletada
- Base atualizado com novos conhecimentos

## Serviços Implementados

### TestmoStructureService

Gerencia criação e busca de estrutura de pastas:

```python
from src.application.shared.testmo_structure_service import TestmoStructureService

# Criar estrutura Base
structure = structure_service.ensure_base_structure(
    project_id=1,
    component="Booking",
    endpoint="/api/v3/quotes",
    method="POST"
)

# Criar estrutura Reativo
structure = structure_service.ensure_reactive_structure(
    project_id=1,
    date="2025-01-15",
    priority="Critical",
    trend="Degrading",
    endpoint="/api/v3/quotes",
    method="POST"
)
```

### TestCaseInheritanceService

Gerencia herança de casos:

```python
from src.application.reativo.test_case_inheritance_service import TestCaseInheritanceService

# Buscar caso no Base
base_case = inheritance_service.find_base_case(
    project_id=1,
    component="Booking",
    endpoint="/api/v3/quotes",
    method="POST"
)

# Herdar para Reativo
reactive_case = inheritance_service.inherit_to_reactive(
    base_case=base_case,
    reactive_context={
        'endpoint': '/api/v3/quotes',
        'method': 'POST',
        'priority': 'Critical',
        'date': '2025-01-15',
        'error_rate': 15.5,
        'total_errors': 150
    },
    project_id=1,
    reactive_folder_id=123
)
```

### ReactiveMergeService

Gerencia merge ao final da sprint:

```python
from src.application.reativo.reactive_merge_service import ReactiveMergeService

# Analisar casos reativos
analysis = merge_service.analyze_reactive_cases(
    project_id=1,
    reactive_folder_id=456
)

# Identificar candidatos
candidates = merge_service.identify_candidates_for_merge(
    project_id=1,
    reactive_folder_id=456,
    min_success_rate=0.8
)

# Migrar para Base
stats = merge_service.merge_to_base(
    project_id=1,
    candidates=candidates,
    replace_existing=False
)

# Limpar pasta reativa
merge_service.cleanup_reactive_folder(
    project_id=1,
    reactive_folder_id=456,
    confirm=True
)
```

## Interface Web

### Edição de Nomes

A interface web permite editar nomes sugeridos antes de sincronizar:

1. **Sugestão automática**: Sistema gera nome seguindo convenção
2. **Edição**: Usuário pode editar o nome
3. **Validação em tempo real**: Formato validado enquanto digita
4. **Preview**: Mostra estrutura antes de criar

### Validação

- Valida formato parseável
- Mostra feedback visual (verde = válido, vermelho = inválido)
- Impede sincronização com nomes inválidos

## Configuração

### Mapeamentos de Componentes

Arquivo: `configs/component_mappings.yaml`

```yaml
component_mappings:
  "booking service": "Booking"
  "booking-service": "Booking"
  "payment service": "Payment"
  # ... mais mapeamentos
```

### Convenções Customizáveis

```yaml
naming_conventions:
  test_case:
    format: "{METHOD}_{TestType}_{Description}"
    ticket_format: "{TICKET_KEY}_{METHOD}_{TestType}_{Description}"
  
  folder_endpoint:
    format: "{METHOD}_{NormalizedPath}"
  
  folder_reactive:
    format: "{YYYY-MM-DD}_{Priority}_{Trend}"
```

## Fluxos Completos

### Fluxo Preventivo

1. Buscar tickets da Sprint (Jira)
2. Extrair componentes (normalizar)
3. Para cada ticket:
   - Criar estrutura: `Sprint-{ID}/{Component}/{Endpoint}`
   - Gerar test case: `{TICKET_KEY}_{METHOD}_{Description}`
   - Salvar no repositório Base

### Fluxo Reativo

1. Processar análise reativa (Splunk)
2. Identificar endpoints prioritários
3. Para cada endpoint:
   - Buscar caso similar no Base (herdar se existir)
   - Criar estrutura: `{Date}_{Priority}_{Trend}/{Endpoint}`
   - Criar test case herdado ou novo
   - Salvar no repositório Reativo
4. Ao final da sprint:
   - Analisar casos reativos
   - Migrar casos úteis para Base
   - Deletar pasta reativa

## Benefícios

1. **Organização clara**: Estrutura previsível e navegável
2. **Reutilização**: Casos do Base podem ser herdados
3. **Rastreabilidade**: Links entre casos base e reativos
4. **Parseabilidade**: Nomes podem ser processados por scripts
5. **Legibilidade**: Humanos entendem facilmente a estrutura
6. **Manutenibilidade**: Fácil encontrar e atualizar casos

## Próximos Passos

1. Implementar UI de merge (seleção de casos para migrar)
2. Adicionar métricas de reutilização
3. Implementar busca avançada por estrutura
4. Adicionar validação de integridade da estrutura

