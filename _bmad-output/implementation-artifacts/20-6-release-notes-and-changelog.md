# Story 20.6: Release Notes and Changelog

Status: ready-for-dev

Epic: 20 - Documentation & Process
Priority: P1
Estimated Effort: 1 day
Dependencies: None

## Story

**As a** User,  
**I want** to see what's new in each release,  
**So that** I can understand changes and learn new features.

---

## Story Metadata

| Field | Value |
|-------|-------|
| Story ID | 20.6 |
| Epic | Epic 20: Documentation & Process |
| Sprint | Sprint 1: Documentation Foundation |
| Priority | P1 |
| Estimated Days | 1 |
| Dependencies | None |
| Status | `ready-for-dev` |

---

## Technical Requirements

1. Create release notes structure
   - Version number and release date
   - New features (with descriptions)
   - Enhancements and improvements
   - Bug fixes (with issue references)
   - Breaking changes (with migration guide)
   - Known issues and workarounds
   - Upgrade instructions

2. Create changelog system
   - Filterable by version
   - Available in-app and online
   - Includes links to documentation for new features
   - Subscribeable (email digest of releases)

3. Automate release notes generation
   - Extract from git commits/tags
   - Generate from PR descriptions
   - Format consistently

---

## Acceptance Criteria

- [ ] **Given** a new version is released  
  **When** user views release notes  
  **Then** they see:
    - Version number and release date
    - New features (with descriptions)
    - Enhancements and improvements
    - Bug fixes (with issue references)
    - Breaking changes (with migration guide)
    - Known issues and workarounds
    - Upgrade instructions
  **And** changelog is:
    - Filterable by version
    - Available in-app and online
    - Includes links to documentation for new features
    - Subscribeable (email digest of releases)

---

## Tasks / Subtasks

- [ ] Task 1: Create release notes structure (AC: #1)
  - [ ] 1.1: Define release notes template
  - [ ] 1.2: Create release notes for current version
  - [ ] 1.3: Create migration guides for breaking changes
  - [ ] 1.4: Document upgrade instructions

- [ ] Task 2: Create changelog system (AC: #2)
  - [ ] 2.1: Create changelog database schema (optional)
  - [ ] 2.2: Create changelog API endpoint
  - [ ] 2.3: Create changelog UI component
  - [ ] 2.4: Implement version filtering
  - [ ] 2.5: Add documentation links
  - [ ] 2.6: Implement email subscription (optional)

- [ ] Task 3: Automate release notes generation (AC: #3)
  - [ ] 3.1: Setup git-based release notes extraction
  - [ ] 3.2: Create PR description parsing
  - [ ] 3.3: Create release notes generator script
  - [ ] 3.4: Integrate with CI/CD pipeline

---

## Dev Notes

### Project Structure Notes

- Release notes should be stored in `docs/releases/` directory
- Changelog can be stored in database or Markdown files
- Use semantic versioning (MAJOR.MINOR.PATCH)
- Release notes in Markdown format
- Changelog API endpoint: `GET /api/v1/changelog`

### References

- [Source: _bmad-output/planning-artifacts/prd.md#Epic-20-Story-20.6] - PRD requirements
- [Source: qa-intelligent-pms/docs/] - Documentation structure
- [Source: _bmad-output/planning-artifacts/project-context.md] - Project context

### Implementation Notes

- Use conventional commits format for automated extraction
- Generate release notes from git tags and commits
- Use GitHub releases or similar for hosting
- Email subscription can use existing notification infrastructure
- Link to documentation files for new features
- Breaking changes require migration guides
- Known issues should link to issue tracker

---

## Dev Agent Record

### Agent Model Used

Auto (Agent Router)

### Debug Log References

### Completion Notes List

### File List
