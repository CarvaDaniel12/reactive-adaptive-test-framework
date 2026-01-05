# Decisões Técnicas

Este documento explica todas as decisões técnicas tomadas no projeto, suas justificativas e alternativas consideradas.

## Stack Tecnológica

### Python Puro

**Decisão**: Usar Python sem frameworks pesados (Django, Flask) inicialmente.

**Justificativa**:
- **Simplicidade**: Menos abstrações, código mais direto
- **Produtividade**: Desenvolvimento rápido de scripts e serviços
- **Ecossistema**: Bibliotecas maduras para todas as necessidades
- **Manutenibilidade**: Código fácil de entender e modificar
- **Baseado em casos reais**: Shopify, Spotify e Nubank usam Python para automação de QA

**Alternativas consideradas**:
- **Node.js**: Menos maduro para ML e análise de dados
- **Java**: Mais verboso, desenvolvimento mais lento
- **Go**: Boa para performance, mas ecossistema menor para QA

**Quando reconsiderar**: Se precisarmos de alta performance ou integração com stack Java/Node existente.

### APIs Diretas (não MCPs)

**Decisão**: Usar APIs REST diretas (Jira REST API, Splunk SDK, Postman API) ao invés de MCPs (Model Context Protocols).

**Justificativa**:
- **Estabilidade**: APIs REST mudam raramente, MCPs mudaram 3x em 6 meses
- **Documentação**: APIs têm documentação oficial completa
- **Suporte**: Suporte oficial das empresas, MCPs são community-driven
- **Produção**: Nenhuma empresa grande usa MCPs em produção ainda
- **ROI**: Menor complexidade, maior confiabilidade

**Alternativas consideradas**:
- **Postman MCP**: Experimental, protocolo instável
- **Playwright MCP**: Community-driven, não oficial
- **Browser-Use MCP**: Proof of concept, quebra em SPAs complexos

**Quando reconsiderar**: Quando MCPs estiverem estáveis (6+ meses sem mudanças) e empresas grandes usarem em produção.

### Playwright Nativo

**Decisão**: Usar Playwright diretamente via Python, sem MCP.

**Justificativa**:
- **Estabilidade**: Playwright Python é maduro e estável
- **Codegen**: Ferramenta oficial para gerar scripts
- **Performance**: Mais rápido que Selenium
- **Features**: Screenshots, vídeos, network interception
- **Baseado em casos reais**: Nubank usa Playwright para automação

**Alternativas consideradas**:
- **Selenium**: Mais lento, menos features
- **Cypress**: JavaScript-only, não Python
- **Playwright MCP**: Experimental, não oficial

**Quando reconsiderar**: Se precisarmos de automação mais avançada com IA (quando MCPs estiverem estáveis).

### Scikit-learn para ML

**Decisão**: Usar Scikit-learn para análise de padrões e ML simples.

**Justificativa**:
- **Maturidade**: Biblioteca Python mais madura para ML
- **Simplicidade**: Fácil de usar para problemas simples
- **Performance**: Suficiente para análise de padrões
- **Documentação**: Excelente documentação e exemplos
- **Baseado em casos reais**: Spotify usa Scikit-learn para análise de logs

**Alternativas consideradas**:
- **TensorFlow/PyTorch**: Overkill para problemas simples
- **Pandas apenas**: Não tem algoritmos de ML
- **APIs de IA**: Custo e latência desnecessários

**Quando reconsiderar**: Se precisarmos de deep learning ou modelos mais complexos.

### Tesseract OCR

**Decisão**: Usar Tesseract para análise visual básica.

**Justificativa**:
- **Open Source**: Gratuito e open source
- **Maturidade**: Biblioteca madura e estável
- **Python**: Boa integração com Python
- **Suficiente**: Para extração de texto de screenshots
- **Baseado em casos reais**: Empresas usam Tesseract para OCR básico

**Alternativas consideradas**:
- **APIs de OCR pagas**: Custo desnecessário para uso básico
- **OpenCV apenas**: Não tem OCR integrado
- **IA multimodal**: Overkill e instável ainda

**Quando reconsiderar**: Se precisarmos de análise visual mais complexa (quando IA multimodal estiver estável).

### Templates + Regras (não IA complexa)

**Decisão**: Usar templates e regras simples para geração de ACs, ao invés de IA complexa.

**Justificativa**:
- **Confiabilidade**: Resultados previsíveis e controláveis
- **Manutenibilidade**: Fácil de ajustar e melhorar
- **Performance**: Rápido, sem latência de APIs
- **Custo**: Zero custo adicional
- **Baseado em casos reais**: Shopify usa templates para geração de ACs

**Alternativas consideradas**:
- **GPT-4/Claude**: Custo, latência e resultados imprevisíveis
- **GLM-4.6V**: Ainda experimental, instável
- **IA customizada**: Complexidade alta, ROI baixo

**Quando reconsiderar**: Quando IA multimodal estiver estável e tivermos casos de uso claros.

## Arquitetura

### Arquitetura Hexagonal

**Decisão**: Usar Arquitetura Hexagonal (Ports & Adapters).

**Justificativa**:
- **Testabilidade**: Facilita testes isolados
- **Manutenibilidade**: Separação clara de responsabilidades
- **Flexibilidade**: Fácil adicionar novas integrações
- **Independência**: Core não depende de frameworks

**Alternativas consideradas**:
- **MVC**: Menos flexível para múltiplas integrações
- **Microserviços**: Over-engineering para início
- **Monolito simples**: Menos organizado, difícil de testar

**Quando reconsiderar**: Se o projeto crescer muito, podemos evoluir para microserviços.

### Domain-Driven Design (DDD)

**Decisão**: Usar conceitos de DDD (Entities, Value Objects, Repositories).

**Justificativa**:
- **Organização**: Código bem organizado
- **Clareza**: Conceitos de domínio explícitos
- **Manutenibilidade**: Fácil de entender e modificar
- **Escalabilidade**: Preparado para crescer

**Alternativas consideradas**:
- **Anemic Domain Model**: Menos expressivo
- **CRUD simples**: Não escala bem

**Quando reconsiderar**: Se o domínio ficar muito simples, podemos simplificar.

## Integrações

### Jira REST API

**Decisão**: Usar Jira REST API v3 diretamente.

**Justificativa**:
- **Estabilidade**: API estável há 10+ anos
- **Documentação**: Documentação oficial completa
- **Suporte**: Suporte oficial da Atlassian
- **Baseado em casos reais**: Shopify usa Jira REST API

**Alternativas consideradas**:
- **Jira Python library**: Abstração desnecessária
- **GraphQL**: Menos documentado, mais complexo

**Quando reconsiderar**: Se Atlassian deprecar REST API (improvável).

### Splunk SDK

**Decisão**: Usar Splunk SDK for Python.

**Justificativa**:
- **Oficial**: SDK oficial da Splunk
- **Estabilidade**: Estável há 5+ anos
- **Features**: Todas as funcionalidades necessárias
- **Baseado em casos reais**: Spotify usa Splunk SDK

**Alternativas consideradas**:
- **REST API direta**: Mais verboso, menos features
- **Splunk MCP**: Experimental, instável

**Quando reconsiderar**: Se Splunk deprecar SDK (improvável).

### Postman API

**Decisão**: Usar Postman API v1 diretamente.

**Justificativa**:
- **Estabilidade**: API estável há 8+ anos
- **Documentação**: Documentação oficial completa
- **Features**: Criação e execução de collections
- **Baseado em casos reais**: Empresas usam Postman API

**Alternativas consideradas**:
- **Postman MCP**: Experimental, instável
- **Newman apenas**: Não permite criar collections

**Quando reconsiderar**: Se Postman deprecar API (improvável).

## Persistência

### File Storage Inicial

**Decisão**: Usar armazenamento em arquivos (JSON/YAML) inicialmente.

**Justificativa**:
- **Simplicidade**: Sem necessidade de banco de dados
- **Portabilidade**: Fácil de mover e fazer backup
- **Desenvolvimento**: Mais rápido para começar
- **Suficiente**: Para MVP e pequeno volume

**Alternativas consideradas**:
- **PostgreSQL**: Over-engineering para início
- **SQLite**: Pode ser necessário depois
- **NoSQL**: Complexidade desnecessária

**Quando reconsiderar**: Quando volume de dados crescer ou precisarmos de queries complexas.

## Testes

### Testes Unitários + Integração

**Decisão**: Usar pytest para testes unitários e de integração.

**Justificativa**:
- **Padrão**: Padrão da indústria Python
- **Features**: Fixtures, parametrização, mocks
- **Simplicidade**: Fácil de usar e entender
- **Ecossistema**: Muitas extensões disponíveis

**Alternativas consideradas**:
- **unittest**: Mais verboso
- **nose**: Deprecated

**Quando reconsiderar**: Não necessário.

## Configuração

### YAML + .env

**Decisão**: Usar YAML para configurações estruturadas e .env para credenciais.

**Justificativa**:
- **YAML**: Legível, estruturado, fácil de editar
- **.env**: Padrão para credenciais, não versionado
- **Separação**: Configurações vs credenciais
- **Simplicidade**: Sem necessidade de banco de configuração

**Alternativas consideradas**:
- **JSON**: Menos legível
- **TOML**: Menos comum
- **Banco de dados**: Complexidade desnecessária

**Quando reconsiderar**: Se precisarmos de configuração dinâmica ou multi-ambiente complexo.

## Sistema de Nomenclatura e Estrutura Testmo

### Convenções Parseáveis

**Decisão**: Usar padrão estruturado com delimitadores `_` para nomes de test cases e pastas.

**Justificativa**:
- **Parseabilidade**: Scripts podem extrair componentes automaticamente (método, tipo, descrição)
- **Legibilidade**: Humanos entendem facilmente o formato
- **Validação**: Regex patterns permitem validação automática
- **Consistência**: Garante estrutura uniforme em todo o repositório

**Formato**:
- Test Case: `{METHOD}_{TestType}_{Description}` (ex: `POST_CreateQuote_ValidRequest`)
- Pasta Endpoint: `{METHOD}_{NormalizedPath}` (ex: `POST_api-v3-quotes`)
- Pasta Reativo: `{YYYY-MM-DD}_{Priority}_{Trend}` (ex: `2025-01-15_Critical_Degrading`)

**Alternativas consideradas**:
- **Nomes livres**: Impossível parsear automaticamente
- **JSON metadata**: Complexidade desnecessária
- **IDs numéricos**: Não legíveis por humanos

**Implementação**: `NameParser` com regex patterns e validação em tempo real.

### Dois Repositórios Separados

**Decisão**: Manter dois repositórios separados no Testmo: Base (reutilizável) e Reativo (sprint-based).

**Justificativa**:
- **Separação de responsabilidades**: Base cresce organicamente como conhecimento, Reativo é focado em problemas atuais
- **Limpeza**: Reativo pode ser deletado ao final da sprint sem afetar Base
- **Organização**: Estrutura diferente para cada propósito (componente vs. criticidade)
- **Manutenibilidade**: Fácil identificar origem e propósito de cada caso

**Estrutura**:
- **Base**: `Base/{Componente}/{METHOD}_{Endpoint}/{METHOD}_{TestType}_{Description}`
- **Reativo**: `Reativo/{Data}_{Prioridade}_{Tendência}/{METHOD}_{Endpoint}/`

**Alternativas consideradas**:
- **Repositório único**: Mistura conhecimento permanente com temporário
- **Tags apenas**: Não oferece estrutura visual clara
- **Milestones**: Não permite estrutura hierárquica adequada

**Implementação**: `TestmoStructureService` gerencia criação e busca de pastas.

### Herança de Casos

**Decisão**: Sistema de herança onde casos reativos herdam estrutura de casos base similares.

**Justificativa**:
- **Reutilização**: Evita duplicação de steps e detalhes
- **Rastreabilidade**: Mantém link entre caso base e reativo
- **Contexto**: Adiciona contexto específico (data, prioridade, métricas) sem perder base
- **Eficiência**: QA não precisa recriar casos do zero

**Fluxo**:
1. Busca caso similar no Base
2. Cria novo caso no Reativo herdando estrutura
3. Adiciona contexto específico (tags, descrição adicional)
4. Mantém link via tags ou campos customizados

**Alternativas consideradas**:
- **Duplicação**: Perde rastreabilidade e cria inconsistências
- **Referência apenas**: Não permite contexto específico
- **Cópia manual**: Ineficiente e propenso a erros

**Implementação**: `TestCaseInheritanceService` gerencia busca, herança e links.

### Normalização Automática

**Decisão**: Normalizadores automáticos para componentes e endpoints com mapeamentos customizáveis.

**Justificativa**:
- **Matching preciso**: Componentes e endpoints podem ter nomes inconsistentes
- **Estrutura consistente**: Garante organização uniforme
- **Manutenibilidade**: Mapeamentos em YAML são fáceis de atualizar
- **Flexibilidade**: Permite variações conhecidas (ex: "Booking Service" → "Booking")

**Normalizadores**:
- `ComponentNormalizer`: PascalCase, remove caracteres especiais, aplica mapeamentos
- `EndpointNormalizer`: Remove trailing slashes, normaliza versões de API, gera slugs
- `TestCaseNameGenerator`: Gera nomes seguindo convenções baseado em método, tipo, descrição

**Alternativas consideradas**:
- **Sem normalização**: Matching impreciso, estrutura inconsistente
- **Normalização rígida**: Pode perder informações importantes
- **Normalização manual**: Ineficiente e propenso a erros

**Implementação**: Normalizadores em `src/application/shared/normalizers.py` com mapeamentos em `configs/component_mappings.yaml`.

### Merge ao Final da Sprint

**Decisão**: Sistema automático para merge de casos úteis do Reativo para Base ao final da sprint.

**Justificativa**:
- **Conhecimento acumulado**: Casos úteis se tornam parte do conhecimento permanente
- **Limpeza**: Reativo pode ser deletado sem perder conhecimento valioso
- **Evolução**: Base cresce organicamente com descobertas de cada sprint
- **Eficiência**: Automatiza processo manual propenso a esquecimentos

**Fluxo**:
1. Analisa casos do Reativo
2. Identifica candidatos para merge (baseado em uso, qualidade, relevância)
3. Migra para Base mantendo estrutura
4. Deleta estrutura reativa após merge

**Alternativas consideradas**:
- **Manter separado**: Base não evolui, conhecimento se perde
- **Merge manual**: Ineficiente e propenso a erros
- **Não deletar Reativo**: Acúmulo de dados temporários

**Implementação**: `ReactiveMergeService` gerencia análise, identificação e merge.

## Conclusão

Todas as decisões foram tomadas com base em:
1. **Estabilidade**: Tecnologias comprovadas em produção
2. **Simplicidade**: Menor complexidade possível
3. **Casos reais**: Baseado em implementações de empresas grandes
4. **ROI**: Maior valor com menor esforço

As decisões podem ser revisitadas conforme o projeto evolui, mas a base está sólida para começar.

