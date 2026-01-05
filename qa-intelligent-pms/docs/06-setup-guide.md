# Guia de Setup

Este guia fornece instruções passo a passo para configurar o Sistema de QA Inteligente para PMS.

## Pré-requisitos

### Software Necessário

- **Python 3.9+**: [Download](https://www.python.org/downloads/)
- **Git**: [Download](https://git-scm.com/downloads)
- **Tesseract OCR**: [Download](https://github.com/tesseract-ocr/tesseract)

### Acessos Necessários

- **Jira**: Acesso com API token
- **Splunk**: Acesso com token de autenticação
- **Postman**: Conta com API key

## Instalação Passo a Passo

### 1. Clonar/Configurar Repositório

```bash
# Se já tem o código
cd qa-intelligent-pms

# Ou criar novo diretório
mkdir qa-intelligent-pms
cd qa-intelligent-pms
```

### 2. Criar Ambiente Virtual

```bash
# Windows
python -m venv venv
venv\Scripts\activate

# Linux/Mac
python3 -m venv venv
source venv/bin/activate
```

### 3. Instalar Dependências

```bash
pip install -r requirements.txt
```

### 4. Instalar Playwright

```bash
playwright install chromium
```

### 5. Instalar Tesseract OCR

#### Windows

1. Baixe o instalador: https://github.com/UB-Mannheim/tesseract/wiki
2. Execute o instalador
3. Adicione ao PATH ou configure no código

#### Linux

```bash
# Ubuntu/Debian
sudo apt-get install tesseract-ocr
sudo apt-get install tesseract-ocr-por  # Para português

# RedHat/CentOS
sudo yum install tesseract
```

#### Mac

```bash
brew install tesseract
```

### 6. Configurar Variáveis de Ambiente

Copie o arquivo de exemplo:

```bash
cp .env.example .env
```

Edite o arquivo `.env` com suas credenciais:

```bash
# Jira
JIRA_API_TOKEN=seu-token-jira-aqui
JIRA_USERNAME=seu-email@exemplo.com

# Splunk
SPLUNK_TOKEN=seu-token-splunk-aqui
SPLUNK_HOST=seu-splunk.com

# Postman
POSTMAN_API_KEY=seu-api-key-postman-aqui
POSTMAN_WORKSPACE_ID=seu-workspace-id
```

**IMPORTANTE**: Nunca commite o arquivo `.env` no Git!

### 7. Configurar Arquivos YAML

Copie os arquivos de exemplo:

```bash
cp configs/jira_config.yaml.example configs/jira_config.yaml
cp configs/splunk_config.yaml.example configs/splunk_config.yaml
cp configs/postman_config.yaml.example configs/postman_config.yaml
```

Edite cada arquivo com suas configurações (veja [Guia de Integrações](05-integrations.md)).

### 8. Criar Diretórios de Dados

```bash
mkdir -p data/tickets
mkdir -p data/test_cases
mkdir -p data/patterns
mkdir -p data/sessions
mkdir -p screenshots
mkdir -p videos
```

## Verificação da Instalação

### Teste Rápido: Script de Testes de Integração

Execute o script de testes que verifica todas as integrações:

```bash
python scripts/test_integrations.py
```

Este script testa:
- Inicialização dos adapters (Jira, Splunk, Postman)
- Playwright (navegação básica)

**Nota**: Os testes verificam apenas a inicialização. Para testes completos, configure as credenciais primeiro.

### Teste 1: Importações Python

```bash
python -c "from src.domain.entities.ticket import Ticket; print('OK')"
```

### Teste 2: Conexão com Jira

```python
# test_jira.py
from src.infrastructure.adapters.jira_adapter import JiraAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = JiraAdapter(config.jira)

# Teste simples
try:
    # Substitua pelo ID de um ticket real
    ticket = adapter.get_ticket("PMS-1")
    print(f"✅ Jira conectado! Ticket: {ticket.summary}")
except Exception as e:
    print(f"❌ Erro: {e}")
```

Execute:

```bash
python test_jira.py
```

### Teste 3: Conexão com Splunk

```python
# test_splunk.py
from src.infrastructure.adapters.splunk_adapter import SplunkAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = SplunkAdapter(config.splunk)

# Teste simples
try:
    query = "search index=pms_logs | head 1"
    results = adapter.execute_query(query)
    print(f"✅ Splunk conectado! Resultados: {len(results)}")
except Exception as e:
    print(f"❌ Erro: {e}")
```

Execute:

```bash
python test_splunk.py
```

### Teste 4: Conexão com Postman

```python
# test_postman.py
from src.infrastructure.adapters.postman_adapter import PostmanAdapter
from src.infrastructure.config.load_config import load_config

config = load_config()
adapter = PostmanAdapter(config.postman)

# Teste simples
try:
    # Listar collections (teste básico)
    collections = adapter.list_collections()
    print(f"✅ Postman conectado! Collections: {len(collections)}")
except Exception as e:
    print(f"❌ Erro: {e}")
```

Execute:

```bash
python test_postman.py
```

### Teste 5: Playwright

```python
# test_playwright.py
from playwright.sync_api import sync_playwright

try:
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        page.goto("https://example.com")
        print(f"✅ Playwright funcionando! Título: {page.title()}")
        browser.close()
except Exception as e:
    print(f"❌ Erro: {e}")
```

Execute:

```bash
python test_playwright.py
```

### Teste 6: Tesseract OCR

```python
# test_tesseract.py
import pytesseract
from PIL import Image
import io

try:
    # Teste básico
    text = pytesseract.image_to_string(Image.new('RGB', (100, 100), color='white'))
    print("✅ Tesseract funcionando!")
except Exception as e:
    print(f"❌ Erro: {e}")
    print("Verifique se Tesseract está instalado e no PATH")
```

Execute:

```bash
python test_tesseract.py
```

## Primeiro Uso

### 1. Executar Exemplos Mínimos (Sem Credenciais)

Teste funcionalidades básicas sem precisar configurar credenciais:

```bash
python scripts/example_minimal.py
```

Este script demonstra:
- Análise de risco de tickets
- Geração automática de ACs
- Uso de Value Objects (RiskLevel, TestPriority)

### 2. Análise Preventiva

```bash
python scripts/run_preventive.py preventivo --sprint-id 123
```

Isso irá:
- Buscar tickets da Sprint 123
- Analisar risco de cada ticket
- Gerar ACs para tickets sem ACs
- Criar collection Postman com testes sugeridos

### 2. Análise Reativa

```python
# run_reactive.py
from src.application.reativo import ReactiveService
from datetime import timedelta

service = ReactiveService()
analysis = service.analyze_production_logs(timedelta(days=7))

print(f"Padrões encontrados: {len(analysis.patterns)}")
print(f"Alertas gerados: {len(analysis.alerts)}")
```

### 3. QA Agent

```python
# run_agent.py
from src.application.agent import QAAgent

agent = QAAgent()
session = agent.assist_qa_testing("PMS-456")

print(f"Sessão gravada: {session.session_id}")
print(f"Script gerado: {session.automation_script is not None}")
```

## Configuração Avançada

### Configurar Logs

Crie `configs/logging.yaml`:

```yaml
version: 1
formatters:
  default:
    format: '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
handlers:
  console:
    class: logging.StreamHandler
    level: INFO
  file:
    class: logging.FileHandler
    filename: logs/qa-intelligent.log
    level: DEBUG
root:
  level: INFO
  handlers: [console, file]
```

### Configurar Cache

Para melhorar performance, configure cache:

```python
# src/infrastructure/config/settings.py
CACHE_ENABLED = True
CACHE_TTL = 3600  # 1 hora
CACHE_DIR = "cache"
```

### Configurar Notificações

Configure notificações (Slack, Email) em `configs/notifications.yaml`:

```yaml
notifications:
  slack:
    enabled: true
    webhook_url: "${SLACK_WEBHOOK_URL}"
  email:
    enabled: false
    smtp_host: "smtp.exemplo.com"
    smtp_port: 587
    username: "${EMAIL_USERNAME}"
    password: "${EMAIL_PASSWORD}"
```

## Troubleshooting

### Erro: "Module not found"

```bash
# Certifique-se de que o ambiente virtual está ativado
source venv/bin/activate  # Linux/Mac
venv\Scripts\activate    # Windows

# Reinstale as dependências
pip install -r requirements.txt
```

### Erro: "Tesseract not found"

**Windows**: Adicione Tesseract ao PATH ou configure:

```python
import pytesseract
pytesseract.pytesseract.tesseract_cmd = r'C:\Program Files\Tesseract-OCR\tesseract.exe'
```

**Linux/Mac**: Verifique instalação:

```bash
which tesseract
tesseract --version
```

### Erro: "Playwright browser not found"

```bash
playwright install chromium
```

### Erro de Autenticação (Jira/Splunk/Postman)

1. Verifique se as credenciais no `.env` estão corretas
2. Verifique se os tokens não expiraram
3. Verifique permissões no sistema externo
4. Teste a conexão manualmente (veja testes acima)

### Erro: "Config file not found"

Certifique-se de que copiou os arquivos de exemplo:

```bash
cp configs/*.yaml.example configs/*.yaml
```

E edite com suas configurações.

### Erro: "Permission denied" (Linux/Mac)

```bash
chmod +x scripts/*.sh
```

## Próximos Passos

Após setup completo:

1. ✅ Execute todos os testes de verificação
2. ✅ Leia a [Documentação de Arquitetura](01-architecture.md)
3. ✅ Leia o [Guia de Integrações](05-integrations.md)
4. ✅ Execute uma análise preventiva de teste
5. ✅ Execute uma análise reativa de teste
6. ✅ Teste o QA Agent com um ticket real

## Suporte

Se encontrar problemas:

1. Verifique os logs em `logs/qa-intelligent.log`
2. Execute os testes de verificação
3. Consulte a documentação específica
4. Verifique se todas as dependências estão instaladas

## Manutenção

### Atualizar Dependências

```bash
pip install --upgrade -r requirements.txt
```

### Limpar Cache

```bash
rm -rf cache/*
```

### Limpar Dados Antigos

```bash
# Manter apenas últimos 30 dias
find data/ -type f -mtime +30 -delete
```

### Backup

Faça backup regular dos dados:

```bash
tar -czf backup-$(date +%Y%m%d).tar.gz data/ configs/
```

