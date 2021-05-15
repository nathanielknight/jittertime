use chrono::naive::NaiveTime;
use chrono::{Duration, Timelike};
use rand::Rng;
use wasm_bindgen::prelude::*;

fn inner_generate(base: NaiveTime, min_jitter_secs: u32, max_jitter_secs: u32) -> NaiveTime {
    let mut rng = rand::thread_rng();
    let jitter_seconds = rng.gen_range(min_jitter_secs..max_jitter_secs) as i64;
    base + Duration::seconds(jitter_seconds)
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn generate(base_src: &str, jitter_amount: u32, jitter_unit_src: &str) -> String {
    let base: chrono::NaiveTime = match NaiveTime::parse_from_str(base_src, "%H:%M") {
        Ok(t) => t,
        Err(_) => {
            log(&format!("couldn't parse {} as time", base_src));
            return "time parse error".into();
        }
    };
    let jitter_base = match jitter_unit_src {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        "d" => 3600 * 24,
        _ => {
            log(&format!("couldn't match {} to unit", jitter_unit_src));
            return "unit parse error".into();
        }
    };
    let max_jitter = jitter_base * jitter_amount;
    let jittered = inner_generate(base, 0, max_jitter);
    format!(
        "{:02}:{:02}:{:02}",
        jittered.hour(),
        jittered.minute(),
        jittered.second()
    )
}

#[test]
fn basic_generate_test() {
    generate("00:00", "1m");
}
