Hum
===
A music notation language and synthesizer written in Rust.

Hum converts \*.hum files in playable \*.wav files.

Building the project
--------------------
To build the project, use `cargo build` in the root directory.

Testing the project
-------------------
To test the project, use `cargo run` in the root directory.

Hum requires two command-line arguments:
...1. the path of the \*.hum file
...2. the desired path of the \*.wav file

To convert the included sample \*.hum file, "daisy.hum," use in the root directory, for example:
`cargo run daisy.hum daisy.wav`
