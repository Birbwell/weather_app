use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    location: Loc,
    current: Current,
}

impl LocationResponse {
    pub fn get_name(&self) -> &str {
        &self.location.name
    }

    pub fn get_temp(&self) -> f32 {
        self.current.temp_f
    }

    pub fn get_condition(&self) -> &str {
        &self.current.condition.text
    }

    pub fn get_region(&self) -> &str {
        &self.location.region
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Loc {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    condition: Condition,
    humidity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    text: String,
}
