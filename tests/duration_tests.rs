use hum::SAMPLE_RATE;
use hum::hum_parse::hum_grammar;
use hum::hum_process;

#[test]
fn test_dotted_note_duration() {
    // 1/4 note at 60 BPM (1 beat per second) -> 1 second duration
    // 44100 samples
    let score_base = "[ 60_bpm ] (An_4 1/4)";
    let commands_base = hum_grammar::score(score_base).unwrap();
    let audio_base = hum_process::run_commands(commands_base).unwrap();
    let base_len = audio_base.len();

    // 1/4+ note -> 1.5 seconds
    let score_dot1 = "[ 60_bpm ] (An_4 1/4)+";
    let commands_dot1 = hum_grammar::score(score_dot1).unwrap();
    let audio_dot1 = hum_process::run_commands(commands_dot1).unwrap();
    let dot1_len = audio_dot1.len();

    // 1/4++ note -> 1.75 seconds
    let score_dot2 = "[ 60_bpm ] (An_4 1/4)++";
    let commands_dot2 = hum_grammar::score(score_dot2).unwrap();
    let audio_dot2 = hum_process::run_commands(commands_dot2).unwrap();
    let dot2_len = audio_dot2.len();

    assert_eq!(
        base_len, SAMPLE_RATE as usize,
        "Base note duration incorrect"
    );
    assert_eq!(
        dot1_len,
        (SAMPLE_RATE as f32 * 1.5) as usize,
        "Single dot duration incorrect"
    );

    // This assertion is expected to fail with the current implementation
    assert_eq!(
        dot2_len,
        (SAMPLE_RATE as f32 * 1.75) as usize,
        "Double dot duration incorrect"
    );
}
