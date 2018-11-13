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
  1. the path of the \*.hum file
  2. the desired path of the \*.wav file

To convert the included sample \*.hum file, "daisy.hum," use in the root directory, for example:

`cargo run daisy.hum daisy.wav`

DISCLAIMER:
-----------
This program produces sound output in the form of \*.wav files, and it is not yet considered stable. I have tried to limit the volume in the code, but you should turn your volume down before experimenting with sound output. I don't want anybody to hurt their ears or speakers while using the program 🙃.

An Explanation of the Hum Music Notation Language:
--------------------------------------------------
Hum files are regular and procedural in nature. They consist of a series of sentences separated by the "." character. Each sentence consists of two clauses separated by the ":" character. The first clause is the command, and the second clause is the value. I haven't implemented hardly any error handling yet, so if you don't have exactly one ":" for every "."––or you use an unknown command or value pattern––it's very possible that the program will crash or produce unexpected output.

### List of currently available commands:

- `#`:

  This is the comment command. These sentences are ignored by the synthesizer and are intended for including annotations in the score.

  Example: `#: This is a comment.`

  Note that at this time, the ":" and "." character are not supported in comments.

- `Tempo`

  This is the tempo command. It sets the tempo of the project. It requires an _integer value_.

  Example: `Tempo: 90.` (this corresponds to 90 beats per minute)

- `Key`

  This is the key command. It accepts two possible text values: `sharps` or `flats`. This just allows you to specify whether you want your score to use sharp notes or flats.

  Example: `Key: sharps.`

  Note that there is no support for mixing sharps and flats at this time.

- `Measure`

  This is the measure command. It specifies the beginning of a new measure in the score. It requires an _integer value_ corresponding to the number of beats in the measure.

  Example: `Measure: 3.` (this starts a new measure which contains three beats)

  Note that the measure command should never come before the tempo or key commands.
