# ADR-0003: Adapter before renderer
Status: Accepted (initial design)

## Decision
Output adapters translate semantic information into renderer-specific presentation commands. Input adapters translate raw interaction into semantic action proposals. Adapters may be stateful, and a single adapter may orchestrate multiple renderers.

## Consequences
An authored-text and a generative-text adapter can share one text renderer. A future Bevy adapter can maintain a presentation scene graph. The engine does not know either implementation.
