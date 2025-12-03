// Calculator with unit conversion support
use std::collections::HashMap;

pub struct Calculator {
    conversion_rates: HashMap<String, HashMap<String, f64>>,
}

impl Calculator {
    pub fn new() -> Self {
        let mut calc = Self {
            conversion_rates: HashMap::new(),
        };
        calc.init_conversions();
        calc
    }

    /// Initialize conversion rates for different unit types
    fn init_conversions(&mut self) {
        // Length conversions (base unit: meter)
        let mut length = HashMap::new();
        length.insert("km".to_string(), 1000.0);
        length.insert("m".to_string(), 1.0);
        length.insert("cm".to_string(), 0.01);
        length.insert("mm".to_string(), 0.001);
        length.insert("mi".to_string(), 1609.344);
        length.insert("ft".to_string(), 0.3048);
        length.insert("in".to_string(), 0.0254);
        self.conversion_rates.insert("length".to_string(), length);

        // Weight conversions (base unit: gram)
        let mut weight = HashMap::new();
        weight.insert("kg".to_string(), 1000.0);
        weight.insert("g".to_string(), 1.0);
        weight.insert("mg".to_string(), 0.001);
        weight.insert("lb".to_string(), 453.592);
        weight.insert("oz".to_string(), 28.3495);
        self.conversion_rates.insert("weight".to_string(), weight);

        // Data conversions (base unit: byte)
        let mut data = HashMap::new();
        data.insert("TB".to_string(), 1024.0 * 1024.0 * 1024.0 * 1024.0);
        data.insert("GB".to_string(), 1024.0 * 1024.0 * 1024.0);
        data.insert("MB".to_string(), 1024.0 * 1024.0);
        data.insert("KB".to_string(), 1024.0);
        data.insert("B".to_string(), 1.0);
        self.conversion_rates.insert("data".to_string(), data);

        // Time conversions (base unit: second)
        let mut time = HashMap::new();
        time.insert("d".to_string(), 86400.0);
        time.insert("h".to_string(), 3600.0);
        time.insert("min".to_string(), 60.0);
        time.insert("s".to_string(), 1.0);
        time.insert("ms".to_string(), 0.001);
        self.conversion_rates.insert("time".to_string(), time);
    }

    /// Evaluate a mathematical expression
    pub fn evaluate(&self, expression: &str) -> Result<f64, String> {
        // Check if it's a unit conversion
        if let Some(result) = self.try_unit_conversion(expression) {
            return result;
        }

        // Check if it's a temperature conversion
        if let Some(result) = self.try_temperature_conversion(expression) {
            return result;
        }

        // Otherwise, evaluate as math expression
        self.evaluate_math(expression)
    }

    /// Try to parse and convert units
    fn try_unit_conversion(&self, expression: &str) -> Option<Result<f64, String>> {
        // Pattern: "100 km to mi" or "100km to mi"
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() < 3 || parts[parts.len() - 2].to_lowercase() != "to" {
            return None;
        }

        // Parse value and from_unit
        let (value, from_unit) = self.parse_value_and_unit(parts[0])?;
        let to_unit = parts[parts.len() - 1];

        // Find which category these units belong to
        for (category, rates) in &self.conversion_rates {
            if rates.contains_key(from_unit) && rates.contains_key(to_unit) {
                let from_rate = rates[from_unit];
                let to_rate = rates[to_unit];
                let result = value * from_rate / to_rate;
                return Some(Ok(result));
            }
        }

        Some(Err(format!(
            "Cannot convert from {} to {}",
            from_unit, to_unit
        )))
    }

    /// Try to parse temperature conversion
    fn try_temperature_conversion(&self, expression: &str) -> Option<Result<f64, String>> {
        let parts: Vec<&str> = expression.split_whitespace().collect();

        if parts.len() < 3 || parts[parts.len() - 2].to_lowercase() != "to" {
            return None;
        }

        let value: f64 = parts[0].parse().ok()?;
        let from_unit = parts[1];
        let to_unit = parts[parts.len() - 1];

        // Temperature conversions
        let result = match (from_unit, to_unit) {
            ("°C" | "C", "°F" | "F") => value * 9.0 / 5.0 + 32.0,
            ("°F" | "F", "°C" | "C") => (value - 32.0) * 5.0 / 9.0,
            ("°C" | "C", "K") => value + 273.15,
            ("K", "°C" | "C") => value - 273.15,
            ("°F" | "F", "K") => (value - 32.0) * 5.0 / 9.0 + 273.15,
            ("K", "°F" | "F") => (value - 273.15) * 9.0 / 5.0 + 32.0,
            _ => return None,
        };

        Some(Ok(result))
    }

    /// Parse a value and unit string like "100km" or "100 km"
    fn parse_value_and_unit(&self, s: &str) -> Option<(f64, &str)> {
        let s = s.trim();

        // Try to find where the number ends
        let mut num_end = 0;
        for (i, c) in s.char_indices() {
            if c.is_ascii_digit() || c == '.' || c == '-' {
                num_end = i + 1;
            } else {
                break;
            }
        }

        if num_end == 0 {
            return None;
        }

        let value: f64 = s[..num_end].parse().ok()?;
        let unit = s[num_end..].trim();

        if unit.is_empty() {
            return None;
        }

        Some((value, unit))
    }

    /// Evaluate a mathematical expression using meval
    fn evaluate_math(&self, expression: &str) -> Result<f64, String> {
        meval::eval_str(expression).map_err(|e| format!("Math error: {}", e))
    }

    /// Format result with appropriate precision
    pub fn format_result(&self, result: f64) -> String {
        // If result is close to an integer, show as integer
        if (result - result.round()).abs() < 0.0001 {
            format!("{}", result.round() as i64)
        } else if result.abs() > 1000.0 || result.abs() < 0.001 {
            // Use scientific notation for very large or small numbers
            format!("{:.4e}", result)
        } else {
            // Otherwise show up to 6 decimal places, trimming trailing zeros
            format!("{:.6}", result).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        let calc = Calculator::new();
        assert_eq!(calc.evaluate("2 + 2").unwrap(), 4.0);
        assert_eq!(calc.evaluate("10 * 5").unwrap(), 50.0);
        assert_eq!(calc.evaluate("100 / 4").unwrap(), 25.0);
    }

    #[test]
    fn test_length_conversion() {
        let calc = Calculator::new();
        let result = calc.evaluate("1 km to m").unwrap();
        assert!((result - 1000.0).abs() < 0.01);

        let result = calc.evaluate("100 cm to m").unwrap();
        assert!((result - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_temperature_conversion() {
        let calc = Calculator::new();
        let result = calc.evaluate("0 °C to °F").unwrap();
        assert!((result - 32.0).abs() < 0.01);

        let result = calc.evaluate("32 °F to °C").unwrap();
        assert!(result.abs() < 0.01);
    }

    #[test]
    fn test_data_conversion() {
        let calc = Calculator::new();
        let result = calc.evaluate("1 GB to MB").unwrap();
        assert!((result - 1024.0).abs() < 0.01);
    }

    #[test]
    fn test_weight_conversion() {
        let calc = Calculator::new();
        let result = calc.evaluate("1 kg to g").unwrap();
        assert!((result - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_format_result() {
        let calc = Calculator::new();
        assert_eq!(calc.format_result(42.0), "42");
        assert_eq!(calc.format_result(3.14159), "3.14159");
        assert_eq!(calc.format_result(1000000.0), "1.0000e6");
    }
}
