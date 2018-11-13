Hum 👄
===
A music notation language and synthesizer written in Rust.

Hum converts \*.hum files to playable \*.wav files.

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

- `Tempo`:

  This is the tempo command. It sets the tempo of the project. It requires an _integer value_.

  Example: `Tempo: 90.` (this corresponds to 90 beats per minute)

- `Key`:

  This is the key command. It accepts two possible text values: `sharps` or `flats`. This just allows you to specify whether you want your score to use sharp notes or flats.

  Example: `Key: sharps.`

  Note that there is no support for mixing sharps and flats at this time.

- `Measure`:

  This is the measure command. It specifies the beginning of a new measure in the score. It requires an _integer value_ corresponding to the number of beats in the measure.

  Example: `Measure: 3.` (this starts a new measure which contains three beats)

  Note that the measure command should never come before the tempo or key commands.

- `Voice`:

  This is the voice command. It specifies the beginning of notation for a single instrument at the beginning of the last declared measure. In other words, a voice is monophonic. To achieve polyphonic sound, you need multiple voice commands under one measure command. See the included `daisy.hum` file for several examples of this. Every time you create a new voice, notation begins at the beginning of the last declared measure. You can include as many voices as you want per measure, and each measure is _not_ required to have the same number of voices. Be careful about including more than five or so at this point, though, because I have not implemented volume controls yet, and the audio might clip (this essentially means to max out in volume and become distorted).

  The voice command requires a text argument corresponding to the wave type or instrument sound which you want to play that part. Right now, the only supported value is `sine`.

  Example: `Voice: sine.`

- Note Commands:

  There are currently _88_ possible note commands corresponding to the keys on a standard grand piano. The note commands are formatted like so: `{note name}_{octave}`. If you set your key value to sharps, then these are the possible note names:

  `["An", "As", "Bn", "Cn", "Cs", "Dn", "Ds", "En", "Fn", "Fs", "Gn", "Gs"]`

  If you set your key value to flats, these are the possible note names:

  `["An", "Bf", "Bn", "Cn", "Df", "Dn", "Ef", "En", "Fn", "Gf", "Gn", "Af"]`

  In this style, "n" refers to "natural," "s" refers to "sharp," and "f" refers to "flat." Additionally, the octave part of a note can range from 0 to 8, with the lowest possible note being `An_0` and the highest possible note being `Cn_7`.

  NOTE: Octave numbers roll over on A natural, so this is how part of the sequence of notes in order of pitch goes: `Gn_4, Gs_4, An_5, As_5, etc...`.

  ALSO––I HAVE JUST NOTICED ON WIKIPEDIA: the traditional convention seems to be to roll over the octave number on C natural instead of A... so that may have to change in version 0.2.0 🧐...

  There is also a special note called `Rest` which corresponds to silence within a single voice, but it _has not yet been implemented_.

  If you use a note value that is not recognized, the current behavior is to not insert the note, which will throw off the timing of your measure. I will work on fixing this in a later version.

  ...So that covers the possible note _commands_. Now for the possible note _values_:

  A note value is simply a fraction in the form `{numerator}/{denominator}`. It evaluates to a floating point number, and it determines the _fraction of the measure_ that the note should fill. Remember that the first note under a voice command is positioned at the beginning of the measure. Therefore, to fill all of the space in the measure, all of the note _values_ should add up to _1.0_, else there will be silence at the end of the measure. If the note values exceed 1.0, then the note will bleed over into the next measure, but the notes that actually belong to the next measure will still start at the beginning of that measure. If this is confusing, I encourage you to play around with a couple of simple \*.hum files until you get the hang of it.

  Putting it all together, here's a simple \*.hum file with two measures, one voice per measure playing the melody, and three voices per measure playing a C major chord:

  ```
  Tempo: 60.
  Key: sharps.

  #: Declare a measure with 3 beats.
  Measure: 3.
  #: Here's a melody with three notes evenly dividing the measure.
  Voice: sine.
  Cn_4: 1/3. Cn_4: 1/3. Cn_4: 1/3.
  #: Here's a chord with three voices playing one note per measure.
  Voice: sine.
  Cn_3: 3/3.
  Voice: sine.
  En_3: 3/3.
  Voice: sine.
  Gn_3: 3/3.

  #: Let's repeat that without comments.

  Measure: 3.
  Voice: sine.
  Cn_4: 1/3. Cn_4: 1/3. Cn_4: 1/3.
  Voice: sine.
  Cn_3: 3/3.
  Voice: sine.
  En_3: 3/3.
  Voice: sine.
  Gn_3: 3/3.
  ```

Why Did I Make This?
====================
I thought it was cool, and I've never programmed a large project in Rust before, so obviously I'm the person for the job 😎. Also, I like the idea of an open music markup language being easily readable by both humans and computers. I think if it's implemented correctly, it might make it easier to preserve musical scores in digital format. This has been a fun project to get started with so far, and I hope that people make awesome music with it.

Why the Choice of License?
==========================
I originally imagined this project acting as a more of a standalone application as opposed to a library, and I would like to remain open, so I felt that the GPLv3 was a good fit. For more insight on what you are and _aren't_ allowed to do with this code, you can read more about its license at [the GNU website](https://www.gnu.org/licenses/licenses.html). If anybody actually starts contributing to or using this code and wants to convince me to release it under an alternative license, then just contact me, and I am open to having a conversation regarding the matter 🙂.
