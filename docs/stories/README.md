# Stories

Stories are work packets. They turn product intent into bounded implementation
and validation work.

Active initiative: [SPEC-001 secure OSS refactor](initiatives/SPEC-001/overview.md)
with [plan](initiatives/SPEC-001/execplan.md), design and validation. Historical
US-011/021/031/041/042/043 packets remain under epics/ for provenance; current
contracts and acceptance status supersede their older runtime behavior.

## Normal Story

Use `docs/templates/story.md` for normal feature work.

Suggested path:

```text
docs/stories/epics/E01-domain-name/US-001-short-story-title.md
```

## High-Risk Story

Use `docs/templates/high-risk-story/` when the feature intake classifies work as
high-risk.

Suggested path:

```text
docs/stories/epics/E02-risky-domain/US-012-risky-story-title/
  execplan.md
  overview.md
  design.md
  validation.md
```

## Status Flow

```text
planned -> in_progress -> implemented
                  |
                  v
               changed
                  |
                  v
               retired
```
