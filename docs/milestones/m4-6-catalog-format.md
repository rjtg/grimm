# M4.6 renderer catalog: first independent implementation increment

This offline CLI contract implements the portable comparison part of the
[M4.6 proposal](m4-6-scene-state-renderer-contract.md). It does **not** implement
scene lifecycle, world providers, boundary saves, preloading or graphics.
See the [integration ledger](m4-6-integration.md) for the remaining gates.

```sh
cargo run --locked -p grimm-cli -- check-catalog \
  crates/cli/tests/fixtures/catalog/story.yaml \
  crates/cli/tests/fixtures/catalog/catalog.yaml
```

The CLI emits JSON with `errors` and `warnings`; errors produce a nonzero exit.
Invalid story/schema input also exits nonzero with a diagnostic. The existing
M3 index supplies every cue/narrative reference from every authored branch,
including timeouts and interrupted action outcomes, independently of live state.
Source pointers identify duration/reference errors. No runtime protocol changes
or extra dependencies are needed.

## Concrete manifest version 1

- `renderer`: stable lowercase ASCII dotted ID.
- `contexts`: explicit story-context ID → semantic presentation-scene ID mapping.
  This is authoring metadata, not new renderer access to the live story graph.
- `scenes`: scene ID → nonempty opaque renderer `resource` URI.
- `presentations`: reusable narrative/presentation ID → `scene_ids`, `completion`,
  optional `duration_ms` and optional `duration_kind`.

IDs are explicit and independent of physical paths. Duplicate map keys are
rejected. Invalid normalized IDs are rejected rather than silently rewritten.
One reusable presentation may explicitly support multiple scenes. Runtime
occurrence cue IDs remain distinct from presentation/content IDs.

Completion modes:

| Mode | Duration metadata | Meaning |
| --- | --- | --- |
| `timed` | Required `exact` duration, including zero | Fixed presentation completion target |
| `callback` | No duration or an `estimate` | Renderer completion signal; estimate does not schedule a callback |
| `instant` | No duration or exact zero | Immediate operation |
| `persistent` | No duration | Lasting renderer state, not an implicitly timed action |

Metadata milliseconds must fit signed 64-bit arithmetic. When the story authors
an exact narrative duration, the catalog must agree on timed/exact completion
(or instant for zero). An estimate never overrides the story's exact schedule.
Unknown scenes, missing mappings/presentations, scope mismatches and conflicting
completion metadata are errors. Unused scenes/presentations are warnings.

The supplied manifest is an availability declaration. Its resource strings stay
opaque to GRIMM: this tool does **not** prove files exist or that an animation
can load. Actual renderer exporters, format/import validation and production
assets remain required by M4.6/M5d integration. The fixture uses `fixture://`
resources and is not a production renderer or a playable M4.6 demo.
