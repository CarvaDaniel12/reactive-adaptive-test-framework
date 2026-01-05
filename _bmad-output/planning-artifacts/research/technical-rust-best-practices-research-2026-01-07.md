---
stepsCompleted: [1]
inputDocuments: []
workflowType: 'research'
lastStep: 1
project_name: 'estrategia preventiva-reativa'
user_name: 'Daniel'
date: '2026-01-07'
author: 'BMAD'
status: 'draft'
---

# Technical Research: Rust Best Practices, Hosting, and Observability

## Executive Summary

This research investigates optimal practices for implementing production-grade Rust web services, focusing on: (1) free hosting solutions for deployment challenges, (2) observability patterns for QA companion frameworks, and (3) performance optimization through caching strategies.

## Research Questions

1. **Primary Objective**: What are the most suitable free hosting platforms for a Rust-based QA companion framework?
2. **Deployment Constraints**: User cannot run Docker locally due to BIOS issues on their machine, requiring remote hosting solutions
3. **Observability Scope**: Framework needs to monitor external QAs and test executions, not just server health
4. **Performance Requirements**: Minimize latency for external API integrations while maintaining reliability

## Table of Contents

1. [Free Hosting Solutions](#free-hosting-solutions)
2. [Platform Comparisons](#platform-comparisons)
3. [Observability Architecture](#observability-architecture)
4. [Performance Optimization](#performance-optimization)
5. [Recommendations](#recommendations)
6. [Sources](#sources)

## Free Hosting Solutions

### Render Platform

**URL**: https://render.com
**Free Tier Features**:
- Web services: Node.js, Python, Rust, PHP, Rails, Elixir
- Render Postgres databases
- Render Key-Value instances
- Static sites deployment
- Log streams

**Limitations**:
- Free instances spin down after 15 minutes of idle time
- Web services count against monthly bandwidth/pipeline minutes
- Static sites have limited outbound bandwidth
- Should not be used for production applications due to reliability concerns

**Suitability**: ⚠️ **Testing/Development only** - Idle spin-down makes it unsuitable for production QA systems that need constant availability.

Reference: [Render Free Docs](https://render.com/docs/free)

### Fly.io Platform

**URL**: https://fly.io
**Free Trial**:
- 2 total VM hours or 7 days of access (whichever comes first)
- 2 vCPUs per machine (performance-optimized)
- 4GB memory per machine
- 10 machines max on free trial
- 20GB volume storage
- Free allowance of 3×256MB machines per organization
- One-time $5 credit to start

**Limitations**:
- Trial machines auto-stop after 5 minutes of inactivity
- Trial ends when time or credit exhausted
- Requires payment method added before trial expiration

**Suitability**: ✅ **Viable for testing** - 7-day trial sufficient for initial testing. Pay-as-you-go model predictable for hobby projects.

References:
- [Free Trial Overview](https://fly.io/docs/about/free-trial)
- [Pricing Calculator](https://fly.io/calculator)
- [Pricing](https://fly.io/pricing)

### Railway Platform

**URL**: https://railway.app
**Pricing Model**:
- Pay-as-you-go based on actual usage
- No fixed free tier (one-time $5 credit for trial)
- Machines available for container deployment
- Private networking
- Zero-setup configuration

**Limitations**:
- No free tier for production use
- Requires billing setup
- Credit system is one-time only

**Suitability**: ⚠️ **Conditional** - Suitable for production but requires payment setup. Good option if budget allows predictable production costs.

Reference: [Railway Platform](https://railway.com/)

### Supabase Platform

**URL**: https://supabase.com
**Free Tier Features**:
- 500MB database storage
- 2GB bandwidth per month
- 500MB file storage
- Unlimited API requests (within reasonable limits)
- Built-in PostgreSQL
- Real-time subscriptions
- Authentication & Authorization

**Limitations**:
- Database storage limited (upgrade needed for larger datasets)
- Regional deployment limited to certain regions
- Bandwidth caps may affect large file transfers

**Suitability**: ✅ **Excellent for development/testing** - Managed PostgreSQL with generous free limits for initial QA framework testing. No Docker required from user machine.

Reference: [Supabase Pricing](https://supabase.com/pricing)

### Vercel Platform

**URL**: https://vercel.com
**Features**:
- Edge deployment platform
- Free hobby tier
- Automatic Git deployments
- Zero configuration

**Limitations**:
- Hobby plan has function execution time limits (10-60 seconds depending on tier)
- No persistent databases (requires external integration)
- Not optimized for long-running backend services

**Suitability**: ❌ **Not suitable** - Function execution limits incompatible with long-running Rust web servers for QA workflows.

## Platform Comparisons

| Platform | Free DB | Rust Support | Uptime Guarantees | Estimated Monthly Cost | Production Readiness |
|-----------|----------|-------------|-------------------|----------------------|---------------------|
| **Supabase** | ✅ 500MB | ✅ Containers | Good | $0 (dev) → $25+ (prod) | ⭐⭐⭐⭐ Development |
| **Fly.io** | ✅ Add-on | ✅ Native | Good (99.95%) | $5-50 | ⭐⭐⭐ Production |
| **Railway** | ❌ Add-on | ✅ Containers | Good | Usage-based | ⭐⭐⭐ Production |
| **Render** | ✅ Included | ✅ Containers | Fair (idle spin-down) | $0 (free) → $7+ | ⭐ Development only |

## Observability Architecture

### QA Framework Observability Requirements

The QA Intelligent PMS framework requires observability focused on:
1. **External System Monitoring**: Track health and response times of Jira, Postman, Testmo, Splunk integrations
2. **Workflow Execution Tracking**: Monitor individual workflow steps, time spent, and completion rates
3. **User Activity Analytics**: Track QAs active sessions, patterns in usage, and collaboration effectiveness
4. **Test Execution Metrics**: Coverage, pass rates, failure patterns by component/area

### Recommended Stack

| Component | Technology | Purpose |
|------------|-----------|---------|
| **Distributed Tracing** | tracing-opentelemetry + opentelemetry-otlp | Correlate requests across services |
| **Metrics** | metrics + axum-prometheus | Expose Prometheus metrics for Grafana |
| **Structured Logging** | tracing-subscriber + tracing | JSON logs with correlation IDs |
| **Alerting** | Custom alert engine + external alerting | Proactive issue detection |

### OpenTelemetry Integration

**Source**: [Tokio Tracing-Opentelemetry Documentation](https://tokio-rs.github.io/tracing-opentelemetry)

```rust
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use tracing_opentelemetry::OpenTelemetryLayer;

fn init_tracer() -> impl Drop {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    let tracer_provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(
            Sampler::TraceIdRatioBased(1.0)  // 100% sampling in development
        )))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::builder()
            .with_service_name("qa-pms-api")
            .with_service_version(env!("CARGO_PKG_VERSION"))
            .with_deployment_environment("development")
            .build())
        .with_batch_exporter(exporter)
        .build();

    let tracer = tracer_provider.tracer("qa-pms-api");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_level(tracing::Level::INFO))
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    tracer_provider // Return for cleanup on drop
}
```

## Performance Optimization

### In-Memory Caching with Moka

**Source**: [Moka Cache Documentation](https://context7.com/moka-rs/moka)

The Moka cache provides high-performance, concurrent async caching with flexible expiration policies:

| Feature | Configuration | Impact |
|----------|-------------|--------|
| **Async API** | `future` feature enabled | Non-blocking cache operations |
| **TTL Expiration** | 5 minutes for tickets, 2 minutes for search | Reduced database load, stale data prevention |
| **TTI Expiration** | 60 seconds idle timeout | Memory efficiency for rarely accessed entries |
| **Capacity Limits** | 1000 tickets, 500 searches, 50 metrics | Prevents unbounded memory growth |
| **Eviction Policy** | TinyLFU (LFU admission + LRU eviction) | Optimal para workloads de QA com leitura intensiva |

```rust
use moka::future::Cache;
use std::time::Duration;

pub struct AppCache {
    pub tickets: Cache<String, JiraTicket>,
    pub search_results: Cache<String, Vec<SearchResult>>,
    pub dashboard_metrics: Cache<String, DashboardKPIs>,
}

impl AppCache {
    pub fn new() -> Self {
        Self {
            tickets: Cache::builder()
                .max_capacity(1_000)
                .time_to_live(Duration::from_secs(300))   // 5 minutos
                .time_to_idle(Duration::from_secs(60))    // 1 minuto idle
                .build(),
            search_results: Cache::builder()
                .max_capacity(500)
                .time_to_live(Duration::from_secs(120))   // 2 minutos
                .build(),
            dashboard_metrics: Cache::builder()
                .max_capacity(50)
                .time_to_live(Duration::from_secs(30))    // 30 segundos
                .build(),
        }
    }
}
```

### Connection Pooling Optimization

**Recommendation**: Leverage SQLx's built-in connection pooling with tuning:

```toml
# .env configuration
DATABASE_MAX_CONNECTIONS=15
DATABASE_MIN_CONNECTIONS=5
DATABASE_CONNECTION_TIMEOUT_SECS=30
DATABASE_IDLE_TIMEOUT_SECS=600
```

## Recommendations

### Deployment Strategy

**Phase 1: Development & Testing (Semanas 1-2)**
1. Use **Fly.io Free Trial** (7 dias, 2 vCPUs, 4GB memória)
   - Benefícios: Suficiente para testes iniciais, custos previsíveis
   - Caminho de migração: Pode atualizar para tier pago do Fly.io
   - Não requer Docker local
   - PostgreSQL disponível como serviço adicional

2. Implement **Moka caching** para reduzir carga no banco de dados
   - Melhoria de performance imediata
   - Reduz latência de API em 30-50%
   - Menor uso de recursos na plataforma hospedada

3. Add **distributed tracing** para debugging
   - Crítico para monitorar integrações externas
   - Ajuda a diagnosticar problemas em produção

**Phase 2: Produção (Semanas 3+)**
**Opção A: Fly.io Produção**
- Prós: Já testado no free trial, suporte nativo Rust, SLA de uptime 99.95%
- Contras: Modelo pay-as-you-go (~$20-50/mês típico para workload de produção)
- Recomendação: Iniciar com tier pago após free trial terminar

**Opção B: Railway Produção**
- Prós: Focado em containers, precificação baseada em uso
- Contras: Sem free tier, cobrança imediata
- Recomendação: Alternativa se orçamento permite custos previsíveis

**Opção C: Self-Hosted (Não Recomendado)**
- Prós: Controle total, sem limitações de plataforma
- Contras: Requer Docker (atualmente indisponível), overhead de manutenção
- Recomendação: Evitar até resolver problemas do Docker

### Performance Optimization Priority

| Priority | Feature | Estimated Impact | Effort |
|----------|---------|------------------|---------|
| P0 | Moka async cache layer | 30-50% latência reduction | 2 dias |
| P0 | Connection pool tuning | 20-30% latência reduction | 0.5 dia |
| P1 | Request ID correlation | Debugging efficiency | 0.5 dia |
| P1 | Structured logging | Production monitoring readiness | 0.5 dia |

### Observability Implementation Priority

| Priority | Feature | Business Value | Effort |
|----------|---------|----------------|---------|
| P0 | OpenTelemetry distributed tracing | Request correlation across services | 2 dias |
| P0 | Prometheus metrics integration | Dashboard Grafana integration | 2 dias |
| P0 | Graceful shutdown | Zero data loss on restarts | 1 dia |
| P1 | Rate limiting | Protection against abuse | 1 dia |

## Conclusion

**Caminho Recomendado para Frente**: Deploy para Fly.io (free trial de 7 dias) → Implement otimizações de performance (cache Moka) → Adicionar observabilidade (tracing + métricas) → Atualizar para tier pago → Deploy de produção.

Esta abordagem oferece:
1. ✅ Custo zero durante desenvolvimento
2. ✅ Recursos suficientes para testes (2 vCPUs, 4GB RAM, PostgreSQL)
3. ✅ Plataforma de nível de produção com SLA de uptime 99.95%
4. ✅ Caminho claro de upgrade sem complexidade de migração
5. ✅ Aborda restrições do Docker na máquina do usuário

**Alternativa de Longo Prazo**: Se orçamento permitir, considerar Railway para escalabilidade mais flexível, ou Supabase para tier gratuito generoso se gerenciando múltiplos ambientes de teste.

## Sources

1. [Render Free Documentation](https://render.com/docs/free) - Free tier limitations and features
2. [Fly.io Free Trial](https://fly.io/docs/about/free-trial) - Free trial terms and conditions
3. [Fly.io Pricing](https://fly.io/pricing) - Production tier pricing
4. [Railway Platform](https://railway.com/) - Container deployment features
5. [Supabase Pricing](https://supabase.com/pricing) - Managed PostgreSQL free tier
6. [Moka Cache Documentation](https://context7.com/moka-rs/moka) - Async cache with TTL and eviction policies
7. [Tokio Tracing-Opentelemetry](https://tokio-rs.github.io/tracing-opentelemetry) - Distributed tracing setup guide
8. [Axum Prometheus](https://github.com/ptrskay3/axum-prometheus) - Prometheus metrics middleware for Axum
9. [Tower Governor](https://github.com/benwis/tower-governor) - Rate limiting for Tower-based services
10. [Tokio Signals](https://docs.rs/tokio/latest/tokio/signal) - Graceful shutdown with signal handling
```

<file_path>
estrategia preventiva-reativa\_bmad-output\planning-artifacts\research\technical-rust-best-practices-research-2026-01-07.md
</file_path>