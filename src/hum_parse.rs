/*
Hum: A Music Markup Language Synthesizer
Copyright (C) 2018-2026 Connor R. Bulakites

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

peg::parser! {
    pub grammar hum_grammar() for str {
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

        pub rule comment() -> (String, String)
            = ws()* "~" text:$((!['\n'][_])*) eol() {
                ("comment".to_string(), text.trim().to_string())
            }

        pub rule tempo() -> (String, String)
            = ws()* "[" ws()* text:$(number()) "_bpm" ws()* "]" ws()* {
                ("tempo".to_string(), text.to_string())
            }

        pub rule time() -> (String, String)
            = ws()* "[" ws()* text:$(fraction()) ws()* "]" ws()* {
                ("time".to_string(), text.to_string())
            }

        pub rule checkpoint() -> (String, String)
            = ws()* "*"+ ws()* {
                // The second value doesn't matter for this one :)
                ("checkpoint".to_string(), "(｡^‿^｡)".to_string())
            }

        pub rule voice() -> (String, String)
            = ws()* "%" ws()* text:$(name()) ws()* {
                ("voice".to_string(), text.to_string())
            }

        pub rule measure() -> (String, String)
            = ws()* "|" ws()* {
                // The second value doesn't matter for this one :)
                ("measure".to_string(), "(｡￣▽￣｡)θ～♪♪".to_string())
            }

        pub rule reset() -> (String, String)
            = ws()* ";" ws_not_newline()* text:$((!['\n'][_])*) eol() {
                ("reset".to_string(), text.trim().to_string())
            }

        pub rule note() -> (String, String)
            = ws()*
            "(" ws()* note_name:$(name()) ws()+ length:$(fraction()) dots_inside:$("+"*) ws()* ")"
            dots_outside:$("+"*) ws()* {
                let all_dots = format!("{}{}", dots_inside, dots_outside);
                (note_name.to_string(), format!("{}{}", length, all_dots).to_string())
            }

        rule name()
            = ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']+

        rule fraction()
            = ['0'..='9' | '/']+

        rule number()
            = ['0'..='9']+

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

#[cfg(test)]
mod tests {
    use super::hum_grammar;

    #[test]
    fn test_parse_note() {
        assert_eq!(
            hum_grammar::note("(A_4 1/4)"),
            Ok(("A_4".to_string(), "1/4".to_string()))
        );
        assert_eq!(
            hum_grammar::note("(Cs_4 1/8)+"),
            Ok(("Cs_4".to_string(), "1/8+".to_string()))
        );
        assert_eq!(
            hum_grammar::note("(Bf_3 1/2)++"),
            Ok(("Bf_3".to_string(), "1/2++".to_string()))
        );
    }

    #[test]
    fn test_parse_tempo() {
        assert_eq!(
            hum_grammar::tempo("[ 120_bpm ]"),
            Ok(("tempo".to_string(), "120".to_string()))
        );
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(
            hum_grammar::time("[ 4/4 ]"),
            Ok(("time".to_string(), "4/4".to_string()))
        );
    }

    #[test]
    fn test_parse_voice() {
        assert_eq!(
            hum_grammar::voice("% piano"),
            Ok(("voice".to_string(), "piano".to_string()))
        );
    }

    #[test]
    fn test_parse_comment() {
        assert_eq!(
            hum_grammar::comment("~ This is a comment\n"),
            Ok(("comment".to_string(), "This is a comment".to_string()))
        );
    }

    #[test]
    fn test_parse_score() {
        let input = r#"
            [ 120_bpm ]
            [ 4/4 ]
            % sine
            |
            (C_4 1/4) (D_4 1/4) (E_4 1/4) (F_4 1/4)
            |
            *
        "#;
        let result = hum_grammar::score(input);
        assert!(result.is_ok());
        let commands = result.unwrap();
        assert_eq!(commands.len(), 10); // tempo, time, voice, measure, 4 notes, measure, checkpoint
    }
}
