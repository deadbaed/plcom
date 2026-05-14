use crate::location::Location;
use backon::BlockingRetryable;
use backon::ExponentialBuilder;
use core::time::Duration;

#[derive(Debug)]
pub enum HttpRequestError {
    Network(reqwest::Error),
    HttpCode(reqwest::StatusCode),
    Json(reqwest::Error),
    NoLocation,
}

impl crate::Gps {
    fn http_request(&self) -> Result<Location, HttpRequestError> {
        let api_request = reqwest::blocking::Client::new()
            .get(format!(
                // Documentation: https://nominatim.org/release-docs/develop/api/Reverse/
                "https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat={}&lon={}&zoom=8",
                self.latitude, self.longitude,
            ))
            .header("User-Agent", "https://philippeloctaux.com")
            .send();

        let api_request = match api_request {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to make API call to get location: {e}");
                return Err(HttpRequestError::Network(e));
            }
        };

        let http_code = api_request.status();
        if !http_code.is_success() {
            eprintln!("got HTTP {http_code}, {:?}", api_request.headers());
            return Err(HttpRequestError::HttpCode(http_code));
        }

        let json = api_request.json::<serde_json::Value>();
        let data = match json {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to deserialize JSON: {e}");
                return Err(HttpRequestError::Json(e));
            }
        };

        let location = &data["display_name"];

        if location.is_string() {
            let location = location.to_string();
            // Remove first and last characters (the string is wrapped in double quotes '"')
            let location = {
                let mut chars = location.chars();
                chars.next();
                chars.next_back();
                chars.as_str()
            };
            eprintln!("Raw location is `{}`", location);

            let mut location = location.split(',');
            let precise = location.next().unwrap_or("?").to_string();

            let mut broad: String = location.collect::<Vec<&str>>().join(",").trim().to_string();
            if broad.is_empty() {
                broad.push('?');
            }

            let location = Location { precise, broad };
            eprintln!("Location is `{:?}`", location);
            Ok(location)
        } else {
            eprintln!("Failed to find location.");
            Err(HttpRequestError::NoLocation)
        }
    }

    pub fn get_location(&self) -> Result<Location, HttpRequestError> {
        let operation = || self.http_request();

        operation
            .retry(ExponentialBuilder::default().with_max_times(10))
            // Sleep implementation, default to std::thread::sleep if `std-blocking-sleep` has been enabled.
            .sleep(std::thread::sleep)
            // When to retry
            .when(|e| matches!(e, HttpRequestError::HttpCode(_)))
            // Notify when retrying
            .notify(|err: &HttpRequestError, dur: Duration| {
                println!("retrying {:?} after {:?}", err, dur);
            })
            .call()
    }
}
