/*
Hum: A Music Markup Language Synthesizer
Copyright (C) 2018 Connor R. Bulakites

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate peg;

peg::parser! {
    pub grammar hum() for str {
        pub rule score() -> Vec<(String, String)>
            = commands:command()* {
                commands
            }

        rule command() -> (String, String)
            = comment()
            / tempo()
            / time()
            / checkpoint()
            / voice()
            / measure()
            / reset()
            / note()

        rule comment() -> (String, String)
            = ws()* "~" text:$(!['\n']*) eol() {
                ("comment".to_string(), text.trim().to_string())
            }

        rule tempo() -> (String, String)
            = ws()* "[" ws()* text:$(number()) "_bpm" ws()* "]" ws()* {
                ("tempo".to_string(), text.to_string())
            }

        rule time() -> (String, String)
            = ws()* "[" ws()* text:$(fraction()) ws()* "]" ws()* {
                ("time".to_string(), text.to_string())
            }

        rule checkpoint() -> (String, String)
            = ws()* "*"+ ws()* {
                // The second value doesn't matter for this one :)
                ("checkpoint".to_string(), "(｡^‿^｡)".to_string())
            }

        rule voice() -> (String, String)
            = ws()* "%" ws()* text:$(name()) ws()* {
                ("voice".to_string(), text.to_string())
            }

        rule measure() -> (String, String)
            = ws()* "|" ws()* {
                // The second value doesn't matter for this one :)
                ("measure".to_string(), "(｡￣▽￣｡)θ～♪♪".to_string())
            }

        rule reset() -> (String, String)
            = ws()* ";" ws_not_newline()* !['\n']* eol() {
                // The second value doesn't matter for this one :)
                ("reset".to_string(), "ヽ(｡＾▽＾｡)ノ".to_string())
            }

        rule note() -> (String, String)
            = ws()* "(" ws()* note_name:$(name()) ws()+ length:$(fraction()) ws()* ")" dots:$("+"*) ws()* {
                (note_name.to_string(), format!("{}{}", length, dots).to_string())
            }

        rule name() -> String
            = text:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) {
                text.to_string()
            }

        rule fraction() -> String
            = text:$(['0'..='9' | '/']*) {
                text.to_string()
            }

        rule number() -> String
            = text:$(['0'..='9']+) {
                text.to_string()
            }

        rule ws()
            = " "
            / "-" // Special whitespace character allowed in hum scores
            / "\t"
            / "\r"
            / "\n"

        rule eol()
            = "\n"
            / ![_] // "End of file"

        rule ws_not_newline()
            = " "
            / "-" // Special whitespace character allowed in hum scores
            / "\t"
            / "\r"
    }
}
