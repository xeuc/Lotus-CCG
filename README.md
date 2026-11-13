Card Collectible Game (with bevy)
![alt text](<Screenshot 2026-03-29 211312.png>)
compile for windows `cargo run`
compile for android `cargo ndk -t arm64-v8a -o android_example/app/src/main/jniLibs build`

## Gameplay foundation

`src/gameplay.rs` contains the shared player progression rules: validated money
transactions, bounded Karma, the 15-interaction daily clock, card instances,
collection/deck operations, Prestige exchange, and time/condition-aware
encounter selection. Gameplay progress and application settings are stored
separately as `progress.json` and `settings.json` in the platform's application
data directory. Invalid save data is logged and is not silently rewritten.

The current boot flow and 3D/UI scenes are still prototype implementations;
this gameplay layer is intended to be consumed by those interfaces as they are
replaced. Run `cargo test --all-targets` to exercise the gameplay rules.
