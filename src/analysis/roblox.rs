//! Roblox API detection and preservation

use crate::parser::ParseResult;
use crate::utils::errors::ObfuscatorError;
use anyhow::Result;
use std::collections::HashSet;

/// Types of Roblox APIs detected
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RobloxApiType {
    /// Global object (game, workspace, script, plugin, shared)
    Global(String),
    /// Service (Players, ReplicatedStorage, etc.)
    Service(String),
    /// Datatype (Vector3, CFrame, Instance, etc.)
    Datatype(String),
    /// Remote (RemoteEvent, RemoteFunction)
    Remote(String),
}

/// Detects and catalogs Roblox API usage
pub struct RobloxApiDetector {
    detected_apis: Vec<RobloxApiType>,
    preserved_names: HashSet<String>,
}

impl RobloxApiDetector {
    pub fn new() -> Self {
        let mut preserved_names = HashSet::new();

        // Add Roblox global objects
        preserved_names.extend([
            "game".to_string(),
            "workspace".to_string(),
            "script".to_string(),
            "plugin".to_string(),
            "shared".to_string(),
            "_G".to_string(),
        ]);

        // Add common Roblox services
        preserved_names.extend([
            "Players".to_string(),
            "ReplicatedStorage".to_string(),
            "ServerStorage".to_string(),
            "ServerScriptService".to_string(),
            "StarterPlayer".to_string(),
            "StarterPack".to_string(),
            "StarterGui".to_string(),
            "Lighting".to_string(),
            "MaterialService".to_string(),
            "SoundService".to_string(),
            "Chat".to_string(),
            "Teams".to_string(),
            "BadgeService".to_string(),
            "HttpService".to_string(),
            "RunService".to_string(),
            "TweenService".to_string(),
            "UserInputService".to_string(),
            "ContextActionService".to_string(),
            "CollectionService".to_string(),
            "DataStoreService".to_string(),
            "MarketplaceService".to_string(),
            "PathfindingService".to_string(),
            "TextService".to_string(),
            "LocalizationService".to_string(),
        ]);

        // Add Roblox datatypes
        preserved_names.extend([
            "Vector3".to_string(),
            "Vector2".to_string(),
            "CFrame".to_string(),
            "UDim".to_string(),
            "UDim2".to_string(),
            "Color3".to_string(),
            "BrickColor".to_string(),
            "Instance".to_string(),
            "Enum".to_string(),
            "Ray".to_string(),
            "Region3".to_string(),
            "NumberSequence".to_string(),
            "ColorSequence".to_string(),
            "NumberRange".to_string(),
            "Rect".to_string(),
            "Faces".to_string(),
            "Axes".to_string(),
            "TweenInfo".to_string(),
            "Random".to_string(),
        ]);

        // Add Remote types
        preserved_names.extend([
            "RemoteEvent".to_string(),
            "RemoteFunction".to_string(),
            "BindableEvent".to_string(),
            "BindableFunction".to_string(),
        ]);

        Self {
            detected_apis: Vec::new(),
            preserved_names,
        }
    }

    /// Detect Roblox API usage in parsed code
    pub fn detect(&mut self, parse_result: &ParseResult) -> Result<Vec<RobloxApiType>> {
        // Scan string literals for service names
        for string_lit in &parse_result.strings {
            if self.is_service_name(&string_lit.value) {
                self.detected_apis.push(RobloxApiType::Service(string_lit.value.clone()));
            } else if self.is_remote_type(&string_lit.value) {
                self.detected_apis.push(RobloxApiType::Remote(string_lit.value.clone()));
            }
        }

        // TODO: Analyze AST for API calls like game:GetService("Players")
        // This would require walking the AST again or enhancing the visitor

        Ok(self.detected_apis.clone())
    }

    /// Get list of identifier names that must be preserved
    pub fn get_preserved_names(&self) -> Vec<String> {
        self.preserved_names.iter().cloned().collect()
    }

    /// Check if a name is a Roblox service
    fn is_service_name(&self, name: &str) -> bool {
        matches!(
            name,
            "Players"
                | "ReplicatedStorage"
                | "ServerStorage"
                | "ServerScriptService"
                | "Lighting"
                | "HttpService"
                | "RunService"
                | "TweenService"
                | "UserInputService"
                | "ContextActionService"
                | "CollectionService"
                | "DataStoreService"
                | "MarketplaceService"
                | "PathfindingService"
                | "TextService"
        )
    }

    /// Check if a name is a remote type
    fn is_remote_type(&self, name: &str) -> bool {
        matches!(
            name,
            "RemoteEvent" | "RemoteFunction" | "BindableEvent" | "BindableFunction"
        )
    }

    /// Check if an identifier should be preserved
    pub fn should_preserve(&self, identifier: &str) -> bool {
        self.preserved_names.contains(identifier)
    }
}

impl Default for RobloxApiDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roblox_globals_preserved() {
        let detector = RobloxApiDetector::new();
        assert!(detector.should_preserve("game"));
        assert!(detector.should_preserve("workspace"));
        assert!(detector.should_preserve("script"));
    }

    #[test]
    fn test_services_preserved() {
        let detector = RobloxApiDetector::new();
        assert!(detector.should_preserve("Players"));
        assert!(detector.should_preserve("ReplicatedStorage"));
        assert!(detector.should_preserve("HttpService"));
    }

    #[test]
    fn test_datatypes_preserved() {
        let detector = RobloxApiDetector::new();
        assert!(detector.should_preserve("Vector3"));
        assert!(detector.should_preserve("CFrame"));
        assert!(detector.should_preserve("Color3"));
    }

    #[test]
    fn test_user_identifiers_not_preserved() {
        let detector = RobloxApiDetector::new();
        assert!(!detector.should_preserve("myVariable"));
        assert!(!detector.should_preserve("calculateDamage"));
    }
}
