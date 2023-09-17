Hum Synthesizer 0.6.0 👄
========================
A music notation language and synthesizer written in Rust.

Hum converts markup text files to playable music saved as WAV files.

_This project is in early development, and its public API is possibly subject to breaking changes at any time. If I knowingly make a breaking change, I will update the MINOR version in the semantic versioning scheme, where the version numbers are MAJOR.MINOR.PATCH._

_DISCLAIMER: This program produces sound output in the form of \*.wav files, and it is not yet considered stable. You should turn down your volume before experimenting with sound output to protect your ears and speakers._

<a name="requirements"></a>
Requirements
------------
- Rust: https://www.rust-lang.org/en-US/install.html

Building the Project
--------------------
To build the project, use `cargo build` in the root directory.

Testing the Project
-------------------
To test the project, use `cargo run` in the root directory.

Hum has two required, positional command-line arguments:
  1. the path of the \*.hum input file
  2. the path of the \*.wav output file

To convert the included \*.hum file, "daisy.hum," to a file called "daisy.wav," use the following command in the root directory:

`cargo run daisy.hum daisy.wav`

Installing the Latest Release
-----------------------------
To install the latest release as a CLI tool, first make sure that you have fulfilled the requirements by [_installing Rust_](#requirements).

Then you can run the following command in the terminal:

`cargo install hum`

Now you can use hum like any other CLI tool. For example, presuming the file `daisy.hum` exists in the current directory, you could use:

`hum daisy.hum daisy.wav`

Using Hum as a Library
----------------------
You can also use Hum as a library in your own Rust programs. Right now, there is one method which implements the functionality of the CLI tool:

```
extern crate hum;
...
hum::convert_to_wav(input, output);
```

An Explanation of the Hum Music Notation Language:
--------------------------------------------------
The Hum music notation language is intended to be easily interpreted by human musicians and computers. It is still in early development and subject to change, but here is a brief explanation of the features available so far. I encourage you to look at the included example files and modify them to help you understand how the language works. First off, here is what the language looks like:

```
~ DAISY BELL by Harry Dacre
~ Based on an 1892 print in The Johns Hopkins University Lester S Levy Sheet Music Collection
~ Arranged by Connor Bulakites to demonstrate the Hum Synthesizer

[ 180_bpm ][ 3/4 ]

***********************************************************************

% square
| (Dn_5 1/2)+ -------------------- | (Bn_4 1/2)+ -------------------- ;
~ Dai-                             ~ sy!

% sine
| (Rest 1/4) (Bn_4 1/4) (Bn_4 1/4) | (Rest 1/4) (Gn_4 1/4) (Gn_4 1/4) ;
| (Rest 1/4) (Gn_4 1/4) (Gn_4 1/4) | (Rest 1/4) (Dn_4 1/4) (Dn_4 1/4) ;
| (Rest 1/4) (Dn_4 1/4) (Dn_4 1/4) | (Rest 1/4) (Bn_3 1/4) (Bn_3 1/4) ;
| (Dn_4 1/2)+ -------------------- | (Bn_3 1/2)+ -------------------- ;

% sawtooth
| (Gn_2 1/2)+ -------------------- | (Dn_2 1/2)+ -------------------- ;


***********************************************************************

% square
| (Gn_4 1/2)+ -------------------- | (Dn_4 1/2)+ -------------------- ;
~ Dai-                             ~ sy!

% sine
| (Rest 1/4) (Dn_4 1/4) (Dn_4 1/4) | (Rest 1/4) (Bn_3 1/4) (Bn_3 1/4) ;
| (Rest 1/4) (Gn_3 1/4) (Gn_3 1/4) | (Rest 1/4) (Gn_3 1/4) (Gn_3 1/4) ;
| (Gn_3 1/2)+ -------------------- | (Dn_3 1/2)+ -------------------- ;

% sawtooth
| (Bn_1 1/2)+ -------------------- | (Gn_1 1/2)+ -------------------- ;
```

Now for some explanation of what you're seeing:

- The tilde character `~` indicates a single-line comment. Everything that appears after this symbol on a line is ignored by the computer. I use this for annotations and for lyrics.

- The tempo tag `[ 100_bpm ]` sets the tempo of the song to 100 _beats per minute_. You can change the numeric portion of the tag to change the tempo, but you must keep the `_bpm` suffix. You can change the tempo partway through a song by putting another tempo tag between any two measures.

- The time signature tag `[ 3/4 ]` sets the time signature of the music. The numerator corresponds to the number of beats per measure, and the denominator corresponds to the reciprocal of the length value of one beat. So in 3/4 time, there are 3 beats with length "1/4" per measure. For a more in-depth explanation of time signatures, see: https://en.wikipedia.org/wiki/Time_signature. You can change the time signature partway through a song by putting another time signature tag between any two measures.

- The line of asterisks `*` indicates a write checkpoint. You should have at least one of these before your first measure. _All lines of music written before the next checkpoint are presumed to occur concurrently_. Lines of music written after the next checkpoint are presumed to start immediately after the last measure in the previous checkpoint. The number of measures or horizontal columns of music you allow per checkpoint and the total number of checkpoints you use are a matter of style and up to you. In the included examples, I put two measures per checkpoint because it fits nicely on an 80-column terminal screen, but you are under no obligation to follow this convention. Additionally, the number of asterisks in the checkpoint line is also a matter of style (you just have to have at least one).

- The division sign `%` is used to switch the voice or "instrument" of lines of music. When you switch to a particular voice, all lines of music underneath the command will be played with that voice until you switch to a new voice. As of now, there are three supported voices: `sine`, `square`, and `sawtooth`.

- The pipe operator `|` indicates the start of a new measure. To ensure that your music is played back correctly, _you must start every measure with the pipe operator_. Additionally, you should make sure that the total length of notes and rests in your measure adds up to the value of the current time signature. Otherwise, music from one measure may bleed over incorrectly into another measure.

- The semicolon `;` serves as the reset character. When a semicolon is encountered, Hum knows that you are done writing one line of music and want to start writing another line of music starting at the last checkpoint. Typically, _all lines of music after a checkpoint which are meant to be played concurrently should end in a semicolon_.

- Hum ignores minus signs `-`. Essentially, they're treated the same as whitespace. This is done to make it easier for you to vertically align concurrent lines of music within a checkpoint so that it is more readable to humans. Exactly how you choose to utilize this feature is up to your stylistic discretion.

- Finally, we must provide an explanation for notes:

  - A note consists of two values enclosed within parentheses and separated by a space. The first value is the note name, and the second value is the note length. The note length divided by the current time signature determines the fraction of the measure that the note takes up. Within a single line of music, notes are added to a measure one after another in succession, reading from left to right.

  - The `+` operator can be appended to the end of a note outside the parentheses to increase the length of the note by one half of its original length value. This corresponds to a "dot" in traditional music notation. So, for example, the note (An_4 1/2)+ has a total length of `1/2 + 1/4 = 3/4`. You can append as many plus signs to the end of a note as you want to keep increasing the length value by one half its original value.

  - There are currently _96_ possible note names corresponding roughly to the keys on a grand piano. The note names are formatted like so: `{pitch}_{octave}`. If you are writing in a key that uses sharps, these are the pitches that you should use:

    `["Cn", "Cs", "Dn", "Ds", "En", "Fn", "Fs", "Gn", "Gs", "An", "As", "Bn"]`

  - If you are writing in a key that uses flats, these are the pitches that you should use:

    `["Cn", "Df", "Dn", "Ef", "En", "Fn", "Gf", "Gn", "Af", "An", "Bf", "Bn"]`

  - In this style, "n" refers to "natural," "s" refers to "sharp," and "f" refers to "flat." Although it's unusual, you can mix sharps and flats in the same song if you wish.

  - Additionally, the octave part of a note can range from 0 to 7, with the lowest possible note being `Cn_0` and the highest possible note being `Bn_7`. Note that octave numbers roll over on C natural, so this is how part of the sequence of notes in order of pitch goes: `An_4, As_4, Bn_4, Cn_5, Cs_5, Dn_5, etc...`.

  - There is also a special note called `Rest` which corresponds to silence within a single voice.

  - If you use a note value that is not recognized, the current behavior is to not insert the note, which will throw off the timing of your measure. I will work on fixing this in a later version.

Why Did I Make This?
--------------------
I thought it was cool, and I've never programmed a large project in Rust before, so obviously I'm the person for the job 😎. Also, I like the idea of an open music markup language being easily readable by both humans and computers. I think if it's implemented correctly, it might make it easier to preserve musical scores in digital format. This has been a fun project to get started with so far, and I hope that people make awesome music with it.

Why the Choice of License?
--------------------------
For more insight on what you currently _are_ and _aren't_ allowed to do with this code, you can read more about the terms of the GPL at [the GNU website](https://www.gnu.org/licenses/licenses.html). If anybody actually starts contributing to or using this code and wants to convince me to release it under an alternative license, then just contact me, and I am open to having a conversation regarding the matter 🙂.
