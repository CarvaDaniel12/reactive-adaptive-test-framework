# QA Intelligent PMS - Concept and Overview

> Collaborative document for strategy discussion and alignment

---

## Executive Summary

This project aims to **increase QA efficiency** by automating repetitive tasks, freeing time for QAs to focus on **strategic thinking and process improvement**, thereby improving overall **product quality**.

The system connects and streamlines three complementary testing approaches:

1. **Reusable Test Repository** - Knowledge base that grows and evolves
2. **Preventive Strategy** - Prepares tests before code reaches production
3. **Reactive Strategy** - Leverages observability metrics to identify weaknesses

---

## Work Cycle

### How It Works

The cycle starts with the **preventive strategy**, feeds the reusable test repository, and feeds back from it. On **Friday**, we collect metrics from the observability tool, verify improvements/degradations, and adapt the strategy.

```
┌─────────────────────────────────────────────────────────────┐
│                    WEEKLY CYCLE                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  📅 WEEK START (Monday)                                     │
│  ┌────────────────────────────────────────────┐             │
│  │  PREVENTIVE STRATEGY                      │             │
│  │  • Analyzes Jira tickets                 │             │
│  │  • Prepares tests and documentation      │             │
│  │  • Organizes in test management tool     │             │
│  │  • Queries/updates base repository       │             │
│  └────────────────────────────────────────────┘             │
│                         ↓                                    │
│  ┌────────────────────────────────────────────┐             │
│  │  REUSABLE TEST REPOSITORY                │             │
│  │  • Receives successful new tests         │             │
│  │  • Provides existing cases for reuse     │             │
│  │  • Evolves as knowledge base             │             │
│  └────────────────────────────────────────────┘             │
│                         ↓                                    │
│  📅 MID-WEEK                                                │
│  ┌────────────────────────────────────────────┐             │
│  │  REACTIVE STRATEGY                       │             │
│  │  • Leverages observability metrics       │             │
│  │  • Identifies major weaknesses           │             │
│  │  • Creates tests focused on real issues  │             │
│  │  • Reuses cases from base repository     │             │
│  └────────────────────────────────────────────┘             │
│                         ↓                                    │
│  📅 WEEK END (Friday)                                       │
│  ┌────────────────────────────────────────────┐             │
│  │  ANALYSIS AND ADAPTATION                 │             │
│  │  • Collects observability metrics        │             │
│  │  • Verifies improvements/degradations    │             │
│  │  • Adapts strategy for next week         │             │
│  │  • Merges useful cases to base repo      │             │
│  └────────────────────────────────────────────┘             │
│                         ↑                                    │
│                    (Feedback loop)                           │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Three Testing Approaches

### 1. Reusable Test Repository (Base Repository)

**Definition**: A centralized repository storing successful, well-structured tests that can be reused across different contexts.

**Rationale**:
- Prevents work duplication
- Accumulates knowledge over time
- Ensures consistency across sprints
- Serves as knowledge base for both strategies

**Mechanics**:
- Receives successful tests from preventive strategy
- Receives useful cases from reactive strategy (at sprint end)
- Provides existing cases for both strategies to query
- Grows organically, becoming increasingly valuable

---

### 2. Preventive Strategy (Before Sprint)

**Definition**: Creates tests BEFORE code reaches production, preparing tests and organized documentation in the test management tool.

**When to use**: At week/sprint start, before development begins.

**Rationale**:
- Prevents bugs BEFORE they reach production
- Prepares tests based on planned requirements (Jira tickets)
- Identifies high-risk tickets before development starts
- Generates Acceptance Criteria (AC) for tickets missing them

**Mechanics**:
- Analyzes Jira tickets for upcoming sprint
- Calculates risk scores based on component history and ticket complexity
- Generates ACs using templates
- Creates test cases organized by sprint: `Sprint-{ID}/{Component}/{Endpoint}/`
- **Queries base repository** to reuse similar existing cases
- **Updates base repository** with new cases that become reusable knowledge

**Flow**:
```
Jira Tickets → Risk Analysis → AC Generation → Test Case Generation → 
Query Base Repository (reuse if exists) → Create in Sprint Structure → 
Update Base Repository with new cases
```

---

### 3. Reactive Strategy (Friday - Observability)

**Definition**: Leverages observability metrics to understand where the major weaknesses are and create tests focused on real problems already existing in production.

**When to use**: During the week, especially on Friday, analyzing production metrics.

**Rationale**:
- Identifies REAL problems that are happening
- Focuses testing efforts on endpoints with current issues
- Creates tests based on production metrics (error rates, traffic patterns)
- Detects trends and degradation over time

**Mechanics**:
- Analyzes Splunk logs (CSV/JSON exports)
- Identifies critical endpoints (high error rate, high traffic, degrading trends)
- **Reuses existing cases from base repository** via inheritance
- Links with Postman to get real request details (bodies, headers, CURL)
- Creates test cases in reactive repository: `Reativo/{Date}_{Priority}_{Trend}/`
- At sprint end: merges useful cases back to base repository, deletes temporary reactive structure

**Flow**:
```
Splunk Logs → Metrics Analysis → Critical Endpoint Identification → 
Search Postman for Requests → Query Base Repository (inherit if exists) → 
Create in Reactive Structure → Link with base cases → 
Sprint End: Merge useful cases to Base
```

---

## QA Control and Automation

### Integrated TODO/Checklist System

The system includes an **integrated TODO/checklist** in the web application that guides QAs through all workflow steps.

**Features**:
- Integrated task management in web application
- Step-by-step guidance for all workflows (preventive and reactive)
- Clear visibility of current status and next actions
- Common TODO app patterns (check/uncheck, progress tracking, filtering)
- Tasks automatically populated based on workflow context

### Optional Automation with Manual Override

QAs can choose whether to use automation scripts or not, maintaining freedom to take control at any moment.

**Characteristics**:
- QAs can choose to run automation scripts OR perform tasks manually
- Toggle automation on/off at any workflow step
- Manual execution options always available alongside automated ones
- Full control to review, modify, or bypass automated decisions
- Freedom to take manual control at any moment

**Value**:
- **Efficiency**: Automation handles routine work, freeing QA time
- **Strategic Focus**: More time for strategic thinking and process improvement
- **Quality Improvement**: Focus on overall product quality, not repetitive tasks
- **Control**: Never locked into automation - manual override always available
- **Guidance**: Clear checklist ensures nothing is missed while maintaining flexibility

**Workflow Example**:
1. System suggests: "Analyze Splunk logs for critical endpoints"
2. QA chooses: Run automated analysis OR upload and analyze manually
3. System suggests: "Create test cases in Testmo"
4. QA chooses: Use automation script OR create manually via UI
5. TODO list updates automatically showing completed and pending tasks
6. QA can always switch between automated and manual modes

---

## Integrations

### Postman ↔ Testmo

**Problem**: Developers create and test API requests in Postman, but QA needs to create formal test cases in Testmo. Manually duplicating work is error-prone and time-consuming.

**Solution**: Automatic linking and synchronization.

**Benefits**:
- Reuses real requests without manually copying bodies/headers
- Synchronization: changes in Postman reflected in Testmo
- Accuracy: test cases use exactly what developers validated
- Automation: creates test cases automatically from Postman data

---

## Metrics and Observability

### Weekly Analysis (Friday)

On Friday, the system:

1. **Collects metrics** from observability tool (Splunk)
2. **Analyzes** improvements and degradations compared to previous week
3. **Identifies** patterns and trends
4. **Adapts** strategy for next week
5. **Merges** useful cases from reactive strategy into base repository

**Observed indicators**:
- Error rates per endpoint
- Traffic patterns
- Degradation trends
- Most critical components
- Effectiveness of preventive tests

---

## Benefits

### For QA
- **More time** for strategic thinking and process improvement
- **Accumulated knowledge** through reusable repository
- **Full control** over automation vs manual
- **Clear guidance** through integrated TODO system
- **Streamlined workflow** connecting preventive, reactive, and base repository

### For Organization
- **Increased efficiency** in test creation and execution
- **Better quality** through more strategic testing
- **Cost reduction** with less rework and production bugs
- **Visibility** through metrics and continuous analysis
- **Continuous improvement** through weekly feedback cycle

---

## Current Status

### MVP (In Progress ~50%)

**Completed**:
- ✅ Reactive web interface with file processing
- ✅ Postman integration (search, matching)
- ✅ Testmo integration (CRUD, inheritance)
- ✅ Base repository structure
- ✅ Naming conventions system
- ✅ Reactive merge service (structure implemented)

**In Progress/Planned**:
- 🚧 Complete preventive service integration
- 🚧 Preventive reuse from Base repository
- 🚧 Complete lifecycle automation
- 🚧 UI enhancements
- 🚧 TODO/checklist system with automation controls

### Roadmap 2026

See [Roadmap 2026](ROADMAP-2026.md) for complete quarterly planning details.

---

## Collaboration

This document is open for comments, suggestions, and ideas.

**How to contribute**:
- Add comments directly in the document
- Suggest workflow improvements
- Share ideas for metrics and indicators
- Propose new features

---

## Related Documentation

- [Project Concept](PROJECT-CONCEPT.md) - Detailed concepts and complete lifecycle
- [Roadmap 2026](ROADMAP-2026.md) - Detailed quarterly planning
- [Current Status](STATUS-ATUAL.md) - Current implementation status

---

**Last updated**: 2025-01-XX  
**Maintained by**: QA Team  
**Contact**: [Add contact information]
