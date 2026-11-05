# Original placeholder audio

These are small, locally synthesized, original WAV placeholders for WHITEOUT,
not third-party recordings. Mono, signed 16-bit PCM, 22050 Hz:

- `wind.wav`: two seconds of deterministic filtered noise with a slow envelope.
- `motor.wav`: two seconds of 55/110 Hz tones with low noise.
- `alarm.wav`: 0.6-second tapered 660 Hz beep.

Generation used Python's standard `wave`, `math` and seeded `random` modules
(seed 70). No generation pipeline or runtime synthesis dependency is required.
They are embedded in the binary. Wind/motor loop; alarm plays once. Geometry,
characters and motion are authored directly as Bevy sprites/transforms in
`src/stage.rs`; Bevy supplies the default font. These assets are demonstration
placeholders, not production sound or visual art.
