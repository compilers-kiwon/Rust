use hound::WavWriter;
use std::f32::consts::PI;

// 상수 선언
const SAMPLE_RATE: u32 = 44100; // 음질, 44.1kHz(CD음질)
const TONE: f32 = 440.0;        // 주파수, 440Hz('라'음)

fn main() {
    // WAV 파일 포맷 지정
    let spec = hound::WavSpec {
        channels: 1,    // 1:모노, 2:스테레오
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    // WavWriter 객체 생성
    let mut fw = WavWriter::create("a.wav", spec).unwrap();
    // 연속으로 sine 파형 만들기
    let samples = SAMPLE_RATE*3;    // 3초
    for t in 0..samples {
        let v = ((t as f32/SAMPLE_RATE as f32)*TONE*2.0*PI).sin();
        fw.write_sample((v*i16::MAX as f32) as i16).unwrap();
    }
}