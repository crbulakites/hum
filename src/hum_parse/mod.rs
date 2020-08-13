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

peg::parser!{pub grammar hum_grammar() for str {
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
        = ws()* "~" text:$((!['\n'][_])*) eol() {
            ("comment".to_string(), text.trim().to_string())
        }

    rule tempo() -> (String, String)
        = ws()* "[" ws()* text:$(['0'..='9']+) "_bpm" ws()* "]" ws()* {
            ("tempo".to_string(), text.to_string())
        }

    rule time() -> (String, String)
        = ws()* "[" ws()* text:$(['0'..='9' | '/']*) ws()* "]" ws()* {
            ("time".to_string(), text.to_string())
        }

    rule checkpoint() -> (String, String)
        = ws()* "*"+ ws()* {
            // The second value doesn't matter for this one :)
            ("checkpoint".to_string(), "(｡^‿^｡)".to_string())
        }

    rule voice() -> (String, String)
        = ws()* "%" ws()* text:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) ws()* {
            ("voice".to_string(), text.to_string())
        }

    rule measure() -> (String, String)
        = ws()* "|" ws()* {
            // The second value doesn't matter for this one :)
            ("measure".to_string(), "(｡￣▽￣｡)θ～♪♪".to_string())
        }

    rule reset() -> (String, String)
        = ws()* ";" ws_not_newline()* (!['\n'][_])* eol() {
            // The second value doesn't matter for this one :)
            ("reset".to_string(), "ヽ(｡＾▽＾｡)ノ".to_string())
        }

    rule note() -> (String, String)
        = ws()* "(" ws()* name:$(['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+) ws()+ length:$(['0'..='9' | '/']*) ws()* ")" dots:$("+"*) ws()* {
            (name.to_string(), format!("{}{}", length, dots).to_string())
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
}}

