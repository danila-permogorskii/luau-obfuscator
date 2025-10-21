//! Simulated Roblox environment for testing
//!
//! This module provides mock implementations of Roblox services
//! to test protected scripts without requiring actual Roblox Studio.

use std::collections::HashMap;

/// Simulated Roblox environment
pub struct RobloxEnvironment {
    pub players: MockPlayersService,
    pub http_service: MockHttpService,
    pub game_metadata: GameMetadata,
}

impl RobloxEnvironment {
    /// Create a new simulated environment
    pub fn new() -> Self {
        Self {
            players: MockPlayersService::new(),
            http_service: MockHttpService::new(),
            game_metadata: GameMetadata::default(),
        }
    }

    /// Create environment with specific player
    pub fn with_player(user_id: u64, username: impl Into<String>) -> Self {
        let mut env = Self::new();
        env.players.add_player(user_id, username);
        env
    }

    /// Set Place ID
    pub fn with_place_id(mut self, place_id: u64) -> Self {
        self.game_metadata.place_id = place_id;
        self
    }

    /// Enable HTTP service
    pub fn with_http_enabled(mut self) -> Self {
        self.http_service.http_enabled = true;
        self
    }
}

impl Default for RobloxEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock Players service
pub struct MockPlayersService {
    pub local_player: Option<MockPlayer>,
    pub players: Vec<MockPlayer>,
}

impl MockPlayersService {
    pub fn new() -> Self {
        Self {
            local_player: None,
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, user_id: u64, username: impl Into<String>) {
        let player = MockPlayer {
            user_id,
            username: username.into(),
            display_name: username.into(),
        };
        
        if self.local_player.is_none() {
            self.local_player = Some(player.clone());
        }
        
        self.players.push(player);
    }

    pub fn get_player_by_id(&self, user_id: u64) -> Option<&MockPlayer> {
        self.players.iter().find(|p| p.user_id == user_id)
    }
}

impl Default for MockPlayersService {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock player object
#[derive(Debug, Clone)]
pub struct MockPlayer {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
}

/// Mock HTTP service
pub struct MockHttpService {
    pub http_enabled: bool,
    pub mock_responses: HashMap<String, MockHttpResponse>,
}

impl MockHttpService {
    pub fn new() -> Self {
        Self {
            http_enabled: false,
            mock_responses: HashMap::new(),
        }
    }

    pub fn add_mock_response(&mut self, url: impl Into<String>, response: MockHttpResponse) {
        self.mock_responses.insert(url.into(), response);
    }

    pub fn get_async(&self, url: &str) -> Result<String, String> {
        if !self.http_enabled {
            return Err("HttpService is not enabled".to_string());
        }

        if let Some(response) = self.mock_responses.get(url) {
            if response.success {
                Ok(response.body.clone())
            } else {
                Err(response.error_message.clone().unwrap_or_else(|| "HTTP request failed".to_string()))
            }
        } else {
            Err(format!("No mock response configured for URL: {}", url))
        }
    }
}

impl Default for MockHttpService {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock HTTP response
#[derive(Debug, Clone)]
pub struct MockHttpResponse {
    pub success: bool,
    pub body: String,
    pub error_message: Option<String>,
}

impl MockHttpResponse {
    pub fn success(body: impl Into<String>) -> Self {
        Self {
            success: true,
            body: body.into(),
            error_message: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            body: String::new(),
            error_message: Some(message.into()),
        }
    }
}

/// Game metadata
#[derive(Debug, Clone)]
pub struct GameMetadata {
    pub place_id: u64,
    pub game_id: u64,
    pub creator_id: u64,
}

impl Default for GameMetadata {
    fn default() -> Self {
        Self {
            place_id: 123456789,
            game_id: 987654321,
            creator_id: 111111111,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roblox_environment_creation() {
        let env = RobloxEnvironment::new();
        assert!(env.players.local_player.is_none());
        assert_eq!(env.players.players.len(), 0);
    }

    #[test]
    fn test_add_player() {
        let mut env = RobloxEnvironment::new();
        env.players.add_player(123456, "TestPlayer");
        
        assert!(env.players.local_player.is_some());
        assert_eq!(env.players.local_player.as_ref().unwrap().user_id, 123456);
        assert_eq!(env.players.players.len(), 1);
    }

    #[test]
    fn test_with_player() {
        let env = RobloxEnvironment::with_player(999, "Player999");
        assert!(env.players.local_player.is_some());
        assert_eq!(env.players.local_player.unwrap().user_id, 999);
    }

    #[test]
    fn test_http_service_disabled_by_default() {
        let env = RobloxEnvironment::new();
        assert!(!env.http_service.http_enabled);
        
        let result = env.http_service.get_async("https://example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_http_service_mock_response() {
        let mut env = RobloxEnvironment::new().with_http_enabled();
        
        env.http_service.add_mock_response(
            "https://api.example.com/validate",
            MockHttpResponse::success(r#"{"valid":true}"#)
        );
        
        let result = env.http_service.get_async("https://api.example.com/validate");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("\"valid\":true"));
    }

    #[test]
    fn test_game_metadata() {
        let env = RobloxEnvironment::new().with_place_id(555555);
        assert_eq!(env.game_metadata.place_id, 555555);
    }
}
