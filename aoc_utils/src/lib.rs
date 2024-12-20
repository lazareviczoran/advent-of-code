pub mod structs;

pub fn measure_exec_time_with_scale<T, F>(
    mod_path: &str,
    op: F,
    desc: &str,
    force_scale: Option<Scale>,
) -> T
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

    println!("{mod_path} {desc}: {result}, finished in {time} {scale}",);
    result
}

#[macro_export]
macro_rules! run_solution {
    ($op: expr, $desc: expr) => {{
        $crate::measure_exec_time_with_scale(module_path!(), $op, $desc, None)
    }};
    ($op: expr, $desc: expr, $force_scale: expr) => {{
        $crate::measure_exec_time_with_scale(module_path!(), $op, $desc, $force_scale)
    }};
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
        .map(|new_scale| infer_scale_to_output(elapsed, new_scale, skip_infer))
        .unwrap_or_else(|| (current.output_value(elapsed), current.output_format()))
}

#[derive(Debug, Clone, Copy)]
pub enum Scale {
    Nanos,
    Micros,
    Millis,
    Seconds,
}
impl Scale {
    pub fn output_format(&self) -> String {
        match self {
            Self::Nanos => "ns",
            Self::Micros => "μs",
            Self::Millis => "ms",
            Self::Seconds => "s",
        }
        .into()
    }

    pub fn output_value(&self, elapsed: &std::time::Duration) -> String {
        let (value, precision) = match self {
            Self::Nanos => (elapsed.as_nanos() as f64, 0),
            Self::Micros => (elapsed.as_nanos() as f64 / 1000.0, 3),
            Self::Millis => (elapsed.as_micros() as f64 / 1000.0, 3),
            Self::Seconds => (elapsed.as_millis() as f64 / 1000.0, 3),
        };
        format!("{value:.PRECISION$}", PRECISION = precision)
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

#[macro_export]
macro_rules! read_to_string_in_module {
    ($filename: expr) => {{
        std::fs::read_to_string($crate::get_file_path_within_module!($filename))
            .expect("Failed to read file")
    }};
}

#[macro_export]
macro_rules! get_file_path_within_module {
    ($filename: expr) => {{
        let manifest_dir = std::env!("CARGO_MANIFEST_DIR");
        let module_dir = module_path!()
            .split_terminator("::")
            .filter(|n| n != &"tests")
            .last()
            .unwrap();
        format!("{manifest_dir}/src/{module_dir}/{}", $filename)
    }};
}
