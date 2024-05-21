use anyhow::Error;

#[derive(Debug)]
pub struct Point {
    pub frequency_hz: f64,
    pub phase_degrees: Option<f64>,
    pub spl_db: f64,
}

enum Kind {
    AchoReviews,
    Curve1,
    FftAudioTools,
    FrequencyMagnitude,
    FrequencySplPhase,
    RewV5,
    Unknown,
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
        if line == "x\tCurve1" {
            kind = Kind::Curve1;
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
        if line == "Freq[Hz]\tdBSPL\tPhase[Deg]" {
            kind = Kind::FrequencySplPhase;
            continue;
        }
        if line.starts_with("* Measurement data measured by REW V5") {
            kind = Kind::RewV5;
            continue;
        }

        if line == "Frequency\tdB\tUnweighted"
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
            Kind::AchoReviews | Kind::FrequencySplPhase => {
                let values: Vec<String> = line.split('\t').map(|x| x.trim().to_string()).collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: values[2].parse::<f64>().ok(),
                    spl_db: values[1].parse::<f64>()?,
                })
            }
            Kind::Curve1 | Kind::FftAudioTools | Kind::FrequencyMagnitude => {
                let values: Vec<String> = line.split('\t').map(|x| x.trim().to_string()).collect();
                points.push(Point {
                    frequency_hz: values[0].parse::<f64>()?,
                    phase_degrees: None,
                    spl_db: values[1].parse::<f64>()?,
                })
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
                })
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
                })
            }
        }
    }

    Ok(points)
}
