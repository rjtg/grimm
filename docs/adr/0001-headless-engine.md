# ADR-0001: Headless engine
Status: Accepted (initial design)

## Decision
The engine owns game rules and state but depends on no renderer, UI framework, or model provider. A composition root assembles the runtime.

## Consequences
CLI is a first-class frontend; graphical presentation is optional; game logic is testable without rendering.
