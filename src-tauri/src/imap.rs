use serde::{Deserialize, Serialize};

/// Configuration for connecting to an IMAP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    /// IMAP server hostname (e.g., "imap.gmail.com")
    pub host: String,
    /// IMAP server port (typically 993 for SSL/TLS, 143 for STARTTLS)
    pub port: u16,
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
    /// Whether to use SSL/TLS for connection
    pub use_tls: bool,
}

/// Represents an IMAP client connection
#[derive(Debug)]
pub struct ImapClient {
    /// Configuration for the IMAP connection
    pub config: ImapConfig,
    /// Connection state
    pub connected: bool,
}

impl ImapClient {
    /// Creates a new IMAP client with the given configuration
    pub fn new(config: ImapConfig) -> Self {
        Self {
            config,
            connected: false,
        }
    }

    /// Placeholder for connect method (to be implemented)
    pub fn connect(&mut self) -> Result<(), String> {
        // TODO: Implement actual IMAP connection
        Err("Not implemented yet".to_string())
    }

    /// Placeholder for disconnect method (to be implemented)
    pub fn disconnect(&mut self) -> Result<(), String> {
        // TODO: Implement disconnect
        self.connected = false;
        Ok(())
    }
}

/// Represents an email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Unique identifier for the email
    pub uid: u32,
    /// Email subject
    pub subject: String,
    /// Sender email address
    pub from: String,
    /// Recipient email addresses
    pub to: Vec<String>,
    /// CC recipients
    pub cc: Vec<String>,
    /// Date the email was sent
    pub date: String,
    /// Email body (plain text)
    pub body_text: Option<String>,
    /// Email body (HTML)
    pub body_html: Option<String>,
    /// Whether the email has been read
    pub is_read: bool,
    /// Whether the email has attachments
    pub has_attachments: bool,
}

/// Represents a mailbox/folder in the IMAP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    /// Name of the mailbox (e.g., "INBOX", "Sent", "Drafts")
    pub name: String,
    /// Number of messages in the mailbox
    pub message_count: u32,
    /// Number of unread messages
    pub unread_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imap_config_creation() {
        let config = ImapConfig {
            host: "imap.example.com".to_string(),
            port: 993,
            username: "user@example.com".to_string(),
            password: "password".to_string(),
            use_tls: true,
        };
        assert_eq!(config.host, "imap.example.com");
        assert_eq!(config.port, 993);
        assert!(config.use_tls);
    }

    #[test]
    fn test_imap_client_creation() {
        let config = ImapConfig {
            host: "imap.example.com".to_string(),
            port: 993,
            username: "user@example.com".to_string(),
            password: "password".to_string(),
            use_tls: true,
        };
        let client = ImapClient::new(config);
        assert!(!client.connected);
    }

    #[test]
    fn test_email_struct() {
        let email = Email {
            uid: 1,
            subject: "Test Email".to_string(),
            from: "sender@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            cc: vec![],
            date: "2026-01-15".to_string(),
            body_text: Some("This is a test".to_string()),
            body_html: None,
            is_read: false,
            has_attachments: false,
        };
        assert_eq!(email.subject, "Test Email");
        assert!(!email.is_read);
    }

    #[test]
    fn test_mailbox_struct() {
        let mailbox = Mailbox {
            name: "INBOX".to_string(),
            message_count: 100,
            unread_count: 5,
        };
        assert_eq!(mailbox.name, "INBOX");
        assert_eq!(mailbox.message_count, 100);
        assert_eq!(mailbox.unread_count, 5);
    }
}
