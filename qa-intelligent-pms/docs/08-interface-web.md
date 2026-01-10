# Interface Web - Guia Completo

## Visão Geral

A interface web fornece uma experiência completa e visual para análise reativa de métricas, com integração direta ao Postman e Testmo.

**Status**: ✅ Funcional e em uso

## Funcionalidades Implementadas

### ✅ Processamento de Arquivos

- **Upload de arquivos CSV/JSON**: Suporta arquivos exportados do Splunk
- **Validação**: Verifica extensão e tamanho (máx 100MB)
- **Progresso Real**: Barra de progresso que reflete o processamento real do Python (não estimativa)
- **Processamento Assíncrono**: Processamento em background com threading

**Tecnologias**:
- Flask (backend)
- JavaScript (frontend com polling)
- Threading para processamento assíncrono

### ✅ Visualização de Resultados

- **Métricas Gerais**: Total de requisições, erros, taxa de erro, endpoints únicos
- **Endpoints Críticos**: Lista de endpoints com maior taxa de erro
- **Endpoints Mais Usados**: Lista priorizada por volume de requisições
- **Design Art Deco**: Interface visual consistente com paleta de cores e tipografia personalizada

### ✅ Gerenciamento de Arquivos

- **Lista de Arquivos Disponíveis**: Visualiza todos os arquivos CSV/JSON disponíveis
- **Processar Arquivo Existente**: Processa arquivos já carregados
- **Deletar Arquivos**: Remove arquivos do sistema
- **Lista de Snapshots Processados**: Visualiza todas as análises processadas
- **Ver Relatórios HTML**: Acessa relatórios HTML gerados automaticamente
- **Deletar Snapshots**: Remove snapshots e relatórios associados

### ✅ Integração com Postman

- **Busca Automática**: Busca matches no Postman para endpoints identificados
- **Visualização de Matches**: Exibe informações detalhadas dos requests encontrados
- **Informações do Postman**: Mostra nome, descrição, collection, método HTTP
- **Score de Match**: Indica qualidade da correspondência

**Endpoint**: `/reactive/find-postman-matches/<snapshot_id>`

### ✅ Integração com Testmo

- **Verificação de Existência**: Verifica se test cases já existem no Testmo
- **Comparação**: Compara casos existentes com novos
- **Sincronização**: Cria ou atualiza test cases no Testmo
- **Herança**: Sistema de herança de casos base para casos reativos
- **Estrutura Organizada**: Cria estrutura de pastas conforme nomenclatura definida

**Endpoint**: `/reactive/sync-test-cases`

**Fluxo**:
1. Buscar matches no Postman
2. Verificar existência no Testmo
3. Comparar e sugerir atualizações
4. Sincronizar selecionados
5. Criar estrutura reativa com herança do Base

### ✅ Design System

- **Art Deco**: Paleta de cores consistente (obsidian, gold, champagne, pewter)
- **Componentes Reutilizáveis**: Cards, botões, tabelas padronizados
- **Tokens de Design**: Centralizados em `design-tokens.js`
- **Responsivo**: Layout adaptável
- **Acessibilidade**: Contrastes adequados e navegação clara

### ✅ Anomaly Detection Dashboard (Story 31.9)

**Funcionalidades:**
- **Lista de Anomalias**: Visualização completa de anomalias detectadas
  - Filtros por tipo (6 tipos), severidade (3 níveis), período (24h, 7d, 30d)
  - Badges de severidade (Critical, Warning, Info)
  - Timestamps relativos
  - Cards interativos com hover effects

- **Gráficos de Tendências**:
  - **Anomaly Frequency Chart**: Line chart mostrando frequência de anomalias ao longo do tempo
  - **Severity Distribution Chart**: Bar chart mostrando distribuição por severidade
  - Gráficos responsivos usando Recharts

- **Modal de Detalhes**:
  - Métricas estatísticas completas (z-score, confidence, deviation)
  - Entidades afetadas (workflow IDs, ticket IDs)
  - Passos de investigação sugeridos
  - Visualização de baseline vs current value

- **Integração**:
  - Endpoint: `/anomalies` (rota React)
  - Item de navegação no sidebar "Anomalies"
  - Atualização automática via React Query
  - Loading skeletons e empty states

**Tecnologias:**
- React 19.2 com TypeScript
- TanStack Query para data fetching
- Recharts para visualizações
- Tailwind CSS para estilização

## Como Executar

### Iniciar o Servidor

```bash
# Navegar até a raiz do projeto
cd qa-intelligent-pms

# Ativar ambiente virtual (se necessário)
venv\Scripts\activate  # Windows
source venv/bin/activate  # Linux/Mac

# Executar servidor
python src/presentation/web_app.py
```

O servidor inicia em:
- **Local**: `http://localhost:5000`
- **Rede**: `http://<seu-ip>:5000` (mostrado no terminal)

### Acessar Interface

1. Abra o navegador
2. Acesse `http://localhost:5000` (ou IP de rede se acessando de outro dispositivo)
3. Navegue para "Análise Reativa"

## Fluxo de Uso

### 1. Processar Arquivo

1. Clique em "Escolher Arquivo" ou arraste um arquivo CSV/JSON
2. Clique em "Processar Métricas"
3. Aguarde processamento (barra de progresso mostra status real)
4. Resultados aparecem automaticamente

### 2. Buscar no Postman

1. Após processar, clique em "Buscar Matches no Postman"
2. Aguarde busca (pode levar alguns segundos)
3. Visualize matches encontrados com informações detalhadas

### 3. Sincronizar com Testmo

1. Revise os matches encontrados
2. Selecione os test cases desejados
3. Clique em "Sincronizar com Testmo"
4. Confirme Project ID do Testmo
5. Aguarde sincronização
6. Veja estatísticas (criados, atualizados, herdados, erros)

## Arquitetura Técnica

### Backend (Flask)

```
src/presentation/web_app.py
├── Rotas principais:
│   ├── /                    → Home
│   ├── /reactive            → Página reativa
│   ├── /process             → Processar arquivo (POST)
│   ├── /process/status/<id> → Status do processamento (GET)
│   ├── /files               → Listar arquivos (GET)
│   ├── /files/<name>        → Deletar arquivo (DELETE)
│   ├── /snapshots           → Listar snapshots (GET)
│   ├── /snapshots/<id>      → Deletar snapshot (DELETE)
│   ├── /analysis/<id>       → Obter análise completa (GET)
│   ├── /report/<id>         → Relatório HTML (GET)
│   ├── /reactive/find-postman-matches/<id> → Buscar Postman (POST)
│   └── /reactive/sync-test-cases            → Sincronizar Testmo (POST)
└── Processamento assíncrono:
    └── Threading para processamento não-bloqueante
```

### Frontend (HTML/JS/CSS)

```
src/presentation/templates/
├── base.html              → Template base com design system
├── home.html              → Página inicial
├── reactive.html          → Página de análise reativa
└── components/            → Componentes reutilizáveis
    ├── status.html
    ├── progress_bar.html
    └── table.html

src/presentation/static/
├── css/
│   └── art-deco.css      → Estilos Art Deco
└── js/
    └── design-tokens.js   → Tokens de design (cores, símbolos)
```

### Processamento Assíncrono

O sistema usa threading para processamento não-bloqueante:

1. **Upload**: Arquivo é salvo imediatamente
2. **Task ID**: Retorna task_id para tracking
3. **Thread**: Processamento roda em thread separada
4. **Progresso Real**: Callbacks atualizam progresso em etapas:
   - 20%: Lendo arquivo
   - 40%: Processando métricas
   - 60%: Criando snapshot
   - 75%: Gerando relatório HTML
   - 85%: Salvando snapshot
   - 90%: Finalizando
   - 100%: Concluído
5. **Polling**: Frontend faz polling a cada 500ms para atualizar UI

## Configuração

### Variáveis de Ambiente

Certifique-se de ter configurado no `.env`:

```bash
# Postman
POSTMAN_API_KEY=sua-api-key
POSTMAN_WORKSPACE_ID=seu-workspace-id

# Testmo
TESTMO_URL=https://seu-instance.testmo.net
TESTMO_TOKEN=seu-token
TESTMO_PROJECT_ID=1
```

### Arquivos de Configuração

- `configs/postman_config.yaml`: Configuração do Postman
- `configs/testmo_config.yaml`: Configuração do Testmo

## Troubleshooting

### Servidor não inicia

- Verifique se a porta 5000 está livre
- Certifique-se de estar no ambiente virtual
- Verifique dependências: `pip install -r requirements.txt`

### Erro de conexão do navegador

- Use o IP de rede mostrado no terminal (não localhost)
- Verifique firewall do Windows
- Certifique-se de que o servidor está rodando em `0.0.0.0`

### Processamento falha

- Verifique logs no terminal do servidor
- Confirme que o arquivo é CSV ou JSON válido
- Verifique tamanho do arquivo (máx 100MB)

### Integração Postman/Testmo falha

- Verifique credenciais no `.env`
- Confirme que APIs estão acessíveis
- Verifique logs de erro no console do navegador

## Melhorias Futuras

- [ ] Adicionar autenticação de usuários
- [ ] Histórico de processamentos com filtros
- [ ] Exportação de dados (CSV, Excel)
- [ ] Dashboard com gráficos e métricas
- [ ] Notificações em tempo real
- [ ] Suporte a múltiplos projetos
- [ ] Cache de resultados do Postman
- [ ] Preview de test cases antes de sincronizar

## Status de Integrações

- ✅ **Splunk File Adapter**: Funcionando (processa CSV/JSON)
- ✅ **Postman Adapter**: Funcionando (busca e matching)
- ✅ **Testmo Adapter**: Funcionando (criação, atualização, herança)
- ⚠️ **Splunk API**: Requer configuração adicional de rede

## Notas Técnicas

### Progresso Real vs Estimado

Anteriormente o sistema usava progresso estimado. Agora implementa:
- **Threading**: Processamento em background
- **Callbacks**: Atualização de progresso em etapas reais
- **Polling**: Frontend consulta status a cada 500ms
- **Timeout**: Máximo de 5 minutos de processamento

### Design System

Todos os componentes seguem o design Art Deco:
- Cores: `--color-obsidian`, `--color-gold`, `--color-champagne`, `--color-pewter`
- Tipografia: Marcellus (títulos), Josefin Sans (corpo)
- Símbolos: Substituição automática de emojis por símbolos Art Deco
- Componentes: Cards, botões, tabelas padronizados

## Documentação Relacionada

- [Fluxos de Trabalho](04-workflows.md) - Fluxo reativo completo
- [Integrações](05-integrations.md) - Configuração de APIs
- [Setup Guide](06-setup-guide.md) - Guia de instalação
- [Testmo Integração](TESTMO-CLI-INTEGRACAO.md) - Detalhes do Testmo
