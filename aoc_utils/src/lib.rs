pub fn measure_exec_time<T, F>(op: F, desc: &str)
where
    F: FnOnce() -> T,
    T: std::fmt::Display + std::fmt::Debug,
{
    measure_exec_time_with_scale(op, desc, None)
}

pub fn measure_exec_time_with_scale<T, F>(op: F, desc: &str, force_scale: Option<Scale>)
where
    F: FnOnce() -> T,
    T: std::fmt::Display + std::fmt::Debug,
{
    let instant = std::time::Instant::now();
    let result = op();
    let elapsed = instant.elapsed();
    let (time, scale) = match force_scale {
        Some(scale) => infer_scale_to_output(&elapsed, scale, true),
        None => infer_scale_to_output(&elapsed, Scale::Millis, false),
    };

    println!("{desc}: {result}, finished in {time} {scale}");
}

fn infer_scale_to_output(
    elapsed: &std::time::Duration,
    current: Scale,
    skip_infer: bool,
) -> (String, String) {
    let scale_to_change = match current.value(elapsed) {
        0 if !skip_infer => current.down(),
        val if !skip_infer && val > 1000 => current.up(),
        _ => None,
    };
    scale_to_change
        .and_then(|new_scale| Some(infer_scale_to_output(elapsed, new_scale, skip_infer)))
        .unwrap_or_else(|| {
            (
                current.value(elapsed).to_string(),
                current.output_format().to_string(),
            )
        })
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Nanos,
    Micros,
    Millis,
    Seconds,
}
impl Scale {
    pub fn output_format(&self) -> &'static str {
        match self {
            Self::Nanos => "ns",
            Self::Micros => "μs",
            Self::Millis => "ms",
            Self::Seconds => "s",
        }
    }

    pub fn value(&self, elapsed: &std::time::Duration) -> u128 {
        match self {
            Scale::Nanos => elapsed.as_nanos(),
            Scale::Micros => elapsed.as_micros(),
            Scale::Millis => elapsed.as_millis(),
            Scale::Seconds => elapsed.as_secs() as u128,
        }
    }

    pub fn down(&self) -> Option<Self> {
        match self {
            Self::Nanos => None,
            Self::Micros => Some(Self::Nanos),
            Self::Millis => Some(Self::Micros),
            Self::Seconds => Some(Self::Millis),
        }
    }

    pub fn up(&self) -> Option<Self> {
        match self {
            Self::Nanos => Some(Self::Micros),
            Self::Micros => Some(Self::Millis),
            Self::Millis => Some(Self::Seconds),
            Self::Seconds => None,
        }
    }
}
