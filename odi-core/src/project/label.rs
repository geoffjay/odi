//! Label entity and operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Label identifier type
pub type LabelId = String;

/// Label entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub id: LabelId,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

impl Label {
    /// Create a new label
    pub fn new(id: LabelId, name: String, color: String) -> Self {
        Self {
            id,
            name,
            description: None,
            color,
            created_at: Utc::now(),
        }
    }

    /// Create a new label with description
    pub fn with_description(id: LabelId, name: String, color: String, description: String) -> Self {
        Self {
            id,
            name,
            description: Some(description),
            color,
            created_at: Utc::now(),
        }
    }

    /// Validate label ID (1-50 characters, alphanumeric + ._-)
    pub fn validate_id(id: &str) -> bool {
        !id.is_empty() && id.len() <= 50 && 
        id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-')
    }

    /// Validate label name (1-50 characters)
    pub fn validate_name(name: &str) -> bool {
        !name.is_empty() && name.len() <= 50
    }

    /// Validate hex color format (#RRGGBB)
    pub fn validate_color(color: &str) -> bool {
        color.len() == 7 && 
        color.starts_with('#') && 
        color[1..].chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validate description length (max 200 characters)
    pub fn validate_description(description: &str) -> bool {
        description.len() <= 200
    }

    /// Set label description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    /// Update label name
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if !Self::validate_name(&name) {
            return Err("Invalid label name".to_string());
        }
        self.name = name;
        Ok(())
    }

    /// Update label color
    pub fn set_color(&mut self, color: String) -> Result<(), String> {
        if !Self::validate_color(&color) {
            return Err("Invalid color format. Expected #RRGGBB".to_string());
        }
        self.color = color;
        Ok(())
    }

    /// Get RGB values from hex color
    pub fn rgb_values(&self) -> Result<(u8, u8, u8), String> {
        if !Self::validate_color(&self.color) {
            return Err("Invalid color format".to_string());
        }

        let hex = &self.color[1..]; // Remove #
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red value")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green value")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue value")?;

        Ok((r, g, b))
    }

    /// Check if color is dark (useful for text color selection)
    pub fn is_dark_color(&self) -> bool {
        if let Ok((r, g, b)) = self.rgb_values() {
            // Calculate luminance using relative luminance formula
            let luminance = 0.299 * (r as f32) + 0.587 * (g as f32) + 0.114 * (b as f32);
            luminance < 128.0
        } else {
            false // Default to light if color is invalid
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_creation() {
        let label = Label::new(
            "bug".to_string(),
            "Bug".to_string(),
            "#FF0000".to_string(),
        );
        
        assert_eq!(label.id, "bug");
        assert_eq!(label.name, "Bug");
        assert_eq!(label.color, "#FF0000");
        assert!(label.description.is_none());
    }

    #[test]
    fn test_label_with_description() {
        let label = Label::with_description(
            "feature".to_string(),
            "Feature".to_string(),
            "#00FF00".to_string(),
            "New feature request".to_string(),
        );
        
        assert_eq!(label.description, Some("New feature request".to_string()));
    }

    #[test]
    fn test_label_validation() {
        // ID validation
        assert!(Label::validate_id("bug"));
        assert!(Label::validate_id("feature-request"));
        assert!(Label::validate_id("v1.0"));
        assert!(!Label::validate_id(""));
        assert!(!Label::validate_id(&"a".repeat(51)));

        // Name validation
        assert!(Label::validate_name("Bug"));
        assert!(Label::validate_name("Feature Request"));
        assert!(!Label::validate_name(""));
        assert!(!Label::validate_name(&"a".repeat(51)));

        // Color validation
        assert!(Label::validate_color("#FF0000"));
        assert!(Label::validate_color("#00ff00"));
        assert!(Label::validate_color("#123ABC"));
        assert!(!Label::validate_color("FF0000")); // Missing #
        assert!(!Label::validate_color("#FF00")); // Too short
        assert!(!Label::validate_color("#GGGGGG")); // Invalid hex

        // Description validation
        assert!(Label::validate_description("Short description"));
        assert!(Label::validate_description(""));
        assert!(!Label::validate_description(&"a".repeat(201)));
    }

    #[test]
    fn test_label_updates() {
        let mut label = Label::new(
            "test".to_string(),
            "Test".to_string(),
            "#FF0000".to_string(),
        );
        
        // Update name
        assert!(label.set_name("New Test".to_string()).is_ok());
        assert_eq!(label.name, "New Test");
        
        assert!(label.set_name("".to_string()).is_err()); // Invalid name
        
        // Update color
        assert!(label.set_color("#00FF00".to_string()).is_ok());
        assert_eq!(label.color, "#00FF00");
        
        assert!(label.set_color("invalid".to_string()).is_err()); // Invalid color
        
        // Update description
        label.set_description(Some("Test description".to_string()));
        assert_eq!(label.description, Some("Test description".to_string()));
        
        label.set_description(None);
        assert!(label.description.is_none());
    }

    #[test]
    fn test_rgb_values() {
        let label = Label::new(
            "test".to_string(),
            "Test".to_string(),
            "#FF8000".to_string(),
        );
        
        let (r, g, b) = label.rgb_values().unwrap();
        assert_eq!(r, 255);
        assert_eq!(g, 128);
        assert_eq!(b, 0);
        
        // Test invalid color
        let mut invalid_label = label.clone();
        invalid_label.color = "invalid".to_string();
        assert!(invalid_label.rgb_values().is_err());
    }

    #[test]
    fn test_color_brightness() {
        let dark_label = Label::new(
            "dark".to_string(),
            "Dark".to_string(),
            "#000000".to_string(),
        );
        assert!(dark_label.is_dark_color());
        
        let light_label = Label::new(
            "light".to_string(),
            "Light".to_string(),
            "#FFFFFF".to_string(),
        );
        assert!(!light_label.is_dark_color());
        
        let medium_label = Label::new(
            "medium".to_string(),
            "Medium".to_string(),
            "#808080".to_string(),
        );
        assert!(!medium_label.is_dark_color()); // 128 is the threshold
    }
}