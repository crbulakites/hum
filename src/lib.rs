mod hum_io;
mod hum_process;

pub fn convert_to_wav(filename: &str, outfname: &str) {
    let score_contents = hum_io::read_hum(filename);
    let waveform = hum_process::parse_score(score_contents);
    hum_io::write_wav(waveform, outfname);
}
