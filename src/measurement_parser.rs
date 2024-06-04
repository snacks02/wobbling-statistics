use anyhow::Error;

#[derive(Debug)]
pub struct Point {
    pub frequency_hz: f64,
    pub phase_degrees: Option<f64>,
    pub spl_db: f64,
}

enum Kind {
    AchoReviews,
    Comment,
    FftAudioTools,
    FrequencyMagnitude,
    FrequencySplPhase,
    RewV5,
    Unknown,
    XCurve1,
    XLeft,
    XRight,
}

pub fn parse(text: &str) -> Result<Vec<Point>, Error> {
    let mut points = vec![];
    let mut kind = Kind::Unknown;

    for line in text.lines() {
        if line == "* +info = www.achoreviews.com/solospec" || line == "+info = www.achoreviews.com"
        {
            kind = Kind::AchoReviews;
            continue;
        }
        if line == "Comment: TJ Comment\t" {
            kind = Kind::Comment;
            continue;
        }
        if line.starts_with("FFT\tAudioTools") {
            kind = Kind::FftAudioTools;
            continue;
        }
        if line == "Frequency(Hz)\t   Magnitude(dB)" {
            kind = Kind::FrequencyMagnitude;
            continue;
        }
        if line == "Freq[Hz]     dBSPL  Phase[Deg]" || line == "Freq[Hz]\tdBSPL\tPhase[Deg]" {
            kind = Kind::FrequencySplPhase;
            continue;
        }
        if line.starts_with("* Measurement data measured by REW V5") {
            kind = Kind::RewV5;
            continue;
        }
        if line == "x\tCurve1" {
            kind = Kind::XCurve1;
            continue;
        }
        if line == "x\tLEFT" {
            kind = Kind::XLeft;
            continue;
        }
        if line == "x\tRIGHT" {
            kind = Kind::XRight;
            continue;
        }

        if line == "Comment: TJ Comment"
            || line == "Frequency\tdB\tUnweighted"
            || line == "Senny IE600 L.txt"
            || line == "Senny IE600 R.txt"
            || line.is_empty()
            || line.starts_with('*')
            || line.starts_with("averaging")
            || line.starts_with("decay")
            || line.starts_with("latitude")
            || line.starts_with("longitude")
            || line.starts_with("overall")
            || line.starts_with("peak")
            || line.starts_with("saved")
            || line.starts_with("source")
        {
            continue;
        }

        match kind {
            Kind::AchoReviews => {
                let values: Vec<String> = line.split('\t').map(|x| x.trim().to_string()).collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: values[2].parse::<f64>().ok(),
                    spl_db: values[1].parse::<f64>()?,
                });
            }
            Kind::Comment
            | Kind::FftAudioTools
            | Kind::FrequencyMagnitude
            | Kind::XCurve1
            | Kind::XLeft
            | Kind::XRight => {
                let values: Vec<String> = line.split('\t').map(|x| x.trim().to_string()).collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: None,
                    spl_db: values[1].parse::<f64>()?,
                });
            }
            Kind::FrequencySplPhase => {
                let separator = if line.contains('\t') { "\t" } else { " " };
                let values: Vec<String> = line
                    .split(separator)
                    .filter(|x| !x.is_empty())
                    .map(|x| x.trim().to_string())
                    .collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: values[2].parse::<f64>().ok(),
                    spl_db: values[1].parse::<f64>()?,
                });
            }
            Kind::RewV5 => {
                let separator = if line.contains(", ") {
                    ", "
                } else if line.contains("; ") {
                    "; "
                } else if line.contains('\t') {
                    "\t"
                } else {
                    " "
                };
                let values: Vec<String> = line
                    .split(separator)
                    .map(|x| x.trim().to_string())
                    .collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: values.get(2).and_then(|value| value.parse::<f64>().ok()),
                    spl_db: values[1].parse::<f64>()?,
                });
            }
            Kind::Unknown => {
                let separator = if line.contains(',') {
                    ","
                } else if line.contains('\t') {
                    "\t"
                } else {
                    " "
                };
                let values: Vec<String> = line
                    .split(separator)
                    .map(|x| x.trim().to_string())
                    .collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: values.get(2).and_then(|value| value.parse::<f64>().ok()),
                    spl_db: values[1].parse::<f64>()?,
                });
            }
        }
    }

    Ok(points)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
            self.frequency_hz == other.frequency_hz
                && self.phase_degrees == other.phase_degrees
                && self.spl_db == other.spl_db
        }
    }

    #[test]
    fn it_parses_acho_reviews() {
        let result = parse(indoc!(
            "
            * +info = www.achoreviews.com/solospec
            *
            * Freq(Hz)\tSPL(dB)\tPhase(degrees)
            20.000000\t63.772\t109.1282
            20.299999\t63.378\t108.6914
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.000000,
                    phase_degrees: Some(109.1282),
                    spl_db: 63.772
                },
                Point {
                    frequency_hz: 20.299999,
                    phase_degrees: Some(108.6914),
                    spl_db: 63.378
                }
            ]
        );
    }

    #[test]
    fn it_parses_malformed_acho_reviews() {
        let result = parse(indoc!(
            "
            +info = www.achoreviews.com
            *
            * Freq(Hz)\tSPL(dB)\tPhase(degrees)
            20.299999\t99.796\t-151.2608
            20.600000\t99.811\t-151.7908
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.299999,
                    phase_degrees: Some(-151.2608),
                    spl_db: 99.796
                },
                Point {
                    frequency_hz: 20.600000,
                    phase_degrees: Some(-151.7908),
                    spl_db: 99.811
                }
            ]
        );
    }

    #[test]
    fn it_parses_fft_audio_tools() {
        let result = parse(indoc!(
            "
            FFT\tAudioTools \tv14.8.6 iPad battery  77%\t2021/05/12 11:16\tHeadset Mic 3 Low Range\tEB05A3E2-10AC-49FE-A287-06AC06EBE5AD
            Frequency\tdB\tUnweighted
            19.5\t70.9
            19.8\t71
            overall dB\t96.8 dB
            decay\tAverage
            averaging\t1/12 Octave
            source\tHeadset Mic 3 Low Range
            latitude\t0
            longitude\t0
            saved\t2021/05/12 11:16
            peak\t16.1Hz
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 19.5,
                    phase_degrees: None,
                    spl_db: 70.9
                },
                Point {
                    frequency_hz: 19.8,
                    phase_degrees: None,
                    spl_db: 71.0
                }
            ]
        );
    }

    #[test]
    fn it_parses_frequency_magnitude() {
        let result = parse(indoc!(
            "
            Frequency(Hz)\t   Magnitude(dB)
            1.475\t113.006454
            1.498\t113.051888
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 1.475,
                    phase_degrees: None,
                    spl_db: 113.006454
                },
                Point {
                    frequency_hz: 1.498,
                    phase_degrees: None,
                    spl_db: 113.051888
                }
            ]
        );
    }

    #[test]
    fn it_parses_frequency_spl_phase_with_spaces() {
        let result = parse(indoc!(
            "
            Freq[Hz]     dBSPL  Phase[Deg]
            5.00000    123.785    -5.731
            5.07273    123.783    -5.741
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 5.00000,
                    phase_degrees: Some(-5.731),
                    spl_db: 123.785
                },
                Point {
                    frequency_hz: 5.07273,
                    phase_degrees: Some(-5.741),
                    spl_db: 123.783
                }
            ]
        );
    }

    #[test]
    fn it_parses_frequency_spl_phase_with_tabs() {
        let result = parse(indoc!(
            "
            Freq[Hz]\tdBSPL\tPhase[Deg]
            5.00000\t106.340\t-147.475
            5.07273\t106.324\t-147.610
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 5.00000,
                    phase_degrees: Some(-147.475),
                    spl_db: 106.340
                },
                Point {
                    frequency_hz: 5.07273,
                    phase_degrees: Some(-147.610),
                    spl_db: 106.324
                }
            ]
        );
    }

    #[test]
    fn it_parses_rew_v5_with_degrees() {
        let result = parse(indoc!(
            "
            * Measurement data measured by REW V5.20.3
            * Source: USB-C to 3.5mm Headphone Jack Adapter, USB-C to 3.5mm Headphone Jack Adapter, 0, volume: 0.138. Timing signal peak level -19.8 dBFS, measurement signal peak level -19.3 dBFS
            * Format: 256k Log Swept Sine, 1 sweep at -12.0 dBFS using an acoustic timing reference
            * Dated: Jul 10, 2023 9:36:37 PM
            * REW Settings:
            *  C-weighting compensation: Off
            *  Target level: 75.0 dB
            * Note: Delay -0.1027 ms (-35 mm, -1.39 in) using estimated IR delay relative to Acoustic reference on USB-C to 3.5mm Headphone Jack Adapter L with no timing offset
            * Measurement: Duo L Jul 10
            * Smoothing: 1/12 octave
            * Frequency Step: 1/48 octave
            * Start Frequency: 20.000 Hz
            *
            * Freq(Hz)\tSPL(dB)\tPhase(degrees)
            20.000000\t96.774\t36.7401
            20.299999\t96.813\t36.0714
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.000000,
                    phase_degrees: Some(36.7401),
                    spl_db: 96.774
                },
                Point {
                    frequency_hz: 20.299999,
                    phase_degrees: Some(36.0714),
                    spl_db: 96.813
                }
            ]
        );
    }

    #[test]
    fn it_parses_rew_v5_without_degrees() {
        let result = parse(indoc!(
            "
            * Measurement data measured by REW V5.20.13
            * Source: USB-C to 3.5mm Headphone Jack Adapter, USB-C to 3.5mm Headphone Jack Adapter, 0, volume: 0.510
            * Format: 256k Log Swept Sine, 1 sweep at -12.0 dBFS with no timing reference
            * Dated: 21 Jan, 2024 4:18:19 PM
            * REW Settings:
            *  C-weighting compensation: Off
            *  Target level: 75.0 dB
            * Note: ;
            * Measurement: PMG Audio APX L
            * Smoothing: None
            * Frequency Step: 1/48 octave
            * Start Frequency: 20.000 Hz
            *
            * Freq(Hz)\tSPL(dB)
            20.000000\t68.017
            20.299999\t68.027
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.000000,
                    phase_degrees: None,
                    spl_db: 68.017
                },
                Point {
                    frequency_hz: 20.299999,
                    phase_degrees: None,
                    spl_db: 68.027
                }
            ]
        );
    }

    #[test]
    fn it_parses_x_curve1() {
        let result = parse(indoc!(
            "
            x\tCurve1
            20.0421\t38.3025
            20.331\t38.3396
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.0421,
                    phase_degrees: None,
                    spl_db: 38.3025
                },
                Point {
                    frequency_hz: 20.331,
                    phase_degrees: None,
                    spl_db: 38.3396
                }
            ]
        );
    }

    #[test]
    fn it_parses_x_left() {
        let result = parse(indoc!(
            "
            x\tLEFT
            20.0527\t37.4904
            20.3383\t37.5329
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.0527,
                    phase_degrees: None,
                    spl_db: 37.4904
                },
                Point {
                    frequency_hz: 20.3383,
                    phase_degrees: None,
                    spl_db: 37.5329
                }
            ]
        );
    }

    #[test]
    fn it_parses_x_right() {
        let result = parse(indoc!(
            "
            x\tRIGHT
            20.0522\t36.256
            20.3412\t36.3084
            "
        ))
        .unwrap();

        assert_eq!(
            result,
            vec![
                Point {
                    frequency_hz: 20.0522,
                    phase_degrees: None,
                    spl_db: 36.256
                },
                Point {
                    frequency_hz: 20.3412,
                    phase_degrees: None,
                    spl_db: 36.3084
                }
            ]
        );
    }
}
