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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments.
    let matches = clap::Command::new("hum")
        .version(hum::VERSION)
        .author(hum::AUTHOR)
        .about(hum::ABOUT)
        .subcommand(
            clap::Command::new("edit")
                .about("Opens the Hum editor")
                .arg(clap::Arg::new("FILE").help("The file to edit").index(1)),
        )
        .arg(
            clap::Arg::new("INPUT")
                .help("Sets the path of the hum notation file.")
                .index(1),
        )
        .arg(
            clap::Arg::new("OUTPUT")
                .help("Sets the path of the output WAV file.")
                .index(2),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("edit") {
        let filename = matches.get_one::<String>("FILE").cloned();
        hum::hum_editor::run_editor(filename)?;
        return Ok(());
    }

    if let (Some(input), Some(output)) = (
        matches.get_one::<String>("INPUT"),
        matches.get_one::<String>("OUTPUT"),
    ) {
        // Read the contents of the input file.
        let score_contents = hum::hum_io::read(input)?;

        // Run the program.
        hum::convert_to_wav(&score_contents, output)?;
    } else {
        eprintln!("Error: Missing INPUT and OUTPUT arguments for conversion.");
        eprintln!("Usage: hum <INPUT> <OUTPUT>");
        eprintln!("       hum edit");
        std::process::exit(1);
    }

    Ok(())
}
