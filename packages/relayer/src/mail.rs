use std::path::PathBuf;

use anyhow::Result;
use handlebars::Handlebars;
use relayer_utils::ParsedEmail;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use tokio::fs::read_to_string;
use uuid::Uuid;

use crate::{
    config::PathConfig,
    model::{insert_expected_reply, is_valid_reply, update_request, RequestStatus},
    RelayerState,
};

/// Represents an email message to be sent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub reference: Option<String>,
    pub reply_to: Option<String>,
    pub body_plain: String,
    pub body_html: String,
    pub body_attachments: Option<Vec<EmailAttachment>>,
}

/// Represents an attachment in an email message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAttachment {
    pub inline_id: String,
    pub content_type: String,
    pub contents: Vec<u8>,
}

/// Represents different types of email events.
#[derive(Debug, Clone)]
pub enum EmailEvent {
    Command {
        request_id: Uuid,
        email_address: String,
        command: String,
        account_code: Option<String>,
        subject: String,
        body: String,
    },
    Ack {
        email_addr: String,
        command: String,
        original_message_id: Option<String>,
        original_subject: String,
    },
    Completion {},
    Error {
        email_addr: String,
        error: String,
        original_subject: String,
        original_message_id: Option<String>,
    },
}

/// Handles all possible email events and requests.
///
/// # Arguments
///
/// * `event` - The `EmailAuthEvent` to be handled.
///
/// # Returns
///
/// A `Result` indicating success or an `EmailError`.
pub async fn handle_email_event(event: EmailEvent, relayer_state: RelayerState) -> Result<()> {
    match event {
        EmailEvent::Command {
            request_id,
            email_address,
            command,
            account_code,
            subject,
            body,
        } => {
            // Prepare the command with the account code if it exists
            let command = if let Some(code) = account_code {
                format!("{} Code {}", command, code)
            } else {
                command
            };

            // Create the plain text body
            let body_plain = format!(
                "ZK Email request. \
                Your request ID is {}",
                request_id
            );

            // Prepare data for HTML rendering
            let render_data = serde_json::json!({
                "userEmailAddr": email_address,
                "body": body,
                "requestId": request_id,
                "command": command,
            });
            let body_html =
                render_html("command_template.html", render_data, relayer_state.clone()).await?;

            // Create and send the email
            let email = EmailMessage {
                to: email_address,
                subject,
                reference: None,
                reply_to: None,
                body_plain,
                body_html,
                body_attachments: None,
            };

            send_email(
                email,
                Some(ExpectsReply::new(request_id)),
                relayer_state.clone(),
            )
            .await?;

            update_request(&relayer_state.db, request_id, RequestStatus::EmailSent).await?;
        }
        EmailEvent::Ack {
            email_addr,
            command,
            original_message_id,
            original_subject,
        } => todo!(),
        EmailEvent::Completion {} => todo!(),
        EmailEvent::Error {
            email_addr,
            error,
            original_subject,
            original_message_id,
        } => todo!(), // EmailEvent::Ack {
                      //     email_addr,
                      //     command,
                      //     original_message_id,
                      //     original_subject,
                      // } => {
                      //     let body_plain = format!(
                      //         "Hi {}!\nYour email with the command {} is received.",
                      //         email_addr, command
                      //     );
                      //     // Prepare data for HTML rendering
                      //     let render_data = serde_json::json!({"userEmailAddr": email_addr, "request": command});
                      //     let body_html = render_html("acknowledgement.html", render_data).await?;
                      //     let subject = format!("Re: {}", original_subject);
                      //     // Create and send the email
                      //     let email = EmailMessage {
                      //         to: email_addr,
                      //         subject,
                      //         body_plain,
                      //         body_html,
                      //         reference: original_message_id.clone(),
                      //         reply_to: original_message_id,
                      //         body_attachments: None,
                      //     };
                      //     send_email(email, None).await?;
                      // }
                      // EmailEvent::Completion {} => {}
                      // EmailEvent::Error {
                      //     email_addr,
                      //     error,
                      //     original_subject,
                      //     original_message_id,
                      // } => {
                      //     let subject = format!("Re: {}", original_subject);

                      //     let body_plain = format!(
                      //         "An error occurred while processing your request. \
                      //         Error: {}",
                      //         error
                      //     );

                      //     // Prepare data for HTML rendering
                      //     let render_data = serde_json::json!({
                      //         "error": error,
                      //         "userEmailAddr": email_addr,
                      //     });
                      //     let body_html = render_html("error.html", render_data).await?;

                      //     // Create and send the email
                      //     let email = EmailMessage {
                      //         to: email_addr,
                      //         subject,
                      //         reference: original_message_id.clone(),
                      //         reply_to: original_message_id,
                      //         body_plain,
                      //         body_html,
                      //         body_attachments: None,
                      //     };

                      //     send_email(email, None).await?;
                      // }
    }

    Ok(())
}

/// Renders an HTML template with the given data.
///
/// # Arguments
///
/// * `template_name` - The name of the template file.
/// * `render_data` - The data to be used in rendering the template.
///
/// # Returns
///
/// A `Result` containing the rendered HTML string or an `Error`.
async fn render_html(
    template_name: &str,
    render_data: Value,
    relayer_state: RelayerState,
) -> Result<String> {
    // Construct the full path to the email template
    let email_template_filename = PathBuf::new()
        .join(relayer_state.config.path.email_templates)
        .join(template_name);

    // Read the email template file
    let email_template = read_to_string(&email_template_filename).await?;

    // Create a new Handlebars instance
    let reg = Handlebars::new();

    // Render the template with the provided data
    let template = reg.render_template(&email_template, &render_data)?;
    Ok(template)
}

/// Sends an email using the configured SMTP server.
///
/// # Arguments
///
/// * `email` - The `EmailMessage` to be sent.
/// * `expects_reply` - An optional `ExpectsReply` struct indicating if a reply is expected.
///
/// # Returns
///
/// A `Result` indicating success or an `EmailError`.
async fn send_email(
    email: EmailMessage,
    expects_reply: Option<ExpectsReply>,
    relayer_state: RelayerState,
) -> Result<()> {
    // Send POST request to email server
    let response = relayer_state
        .http_client
        .post(format!("{}/api/sendEmail", relayer_state.config.smtp_url))
        .json(&email)
        .send()
        .await?;

    // Check if the email was sent successfully
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to send email: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    // Handle expected reply if necessary
    if let Some(expects_reply) = expects_reply {
        let response_body: EmailResponse = response.json().await?;

        let message_id = response_body.message_id;
        insert_expected_reply(&relayer_state.db, &message_id, expects_reply.request_id).await?;
    }

    Ok(())
}

/// Represents the response from the email server after sending an email.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EmailResponse {
    status: String,
    message_id: String,
}

/// Represents an expectation of a reply to an email.
pub struct ExpectsReply {
    request_id: Option<String>,
}

impl ExpectsReply {
    /// Creates a new `ExpectsReply` instance with a request ID.
    ///
    /// # Arguments
    ///
    /// * `request_id` - The ID of the request expecting a reply.
    fn new(request_id: Uuid) -> Self {
        Self {
            request_id: Some(request_id.to_string()),
        }
    }

    /// Creates a new `ExpectsReply` instance without a request ID.
    fn new_no_request_id() -> Self {
        Self { request_id: None }
    }
}

/// Checks if the email is a reply to a command that expects a reply.
/// Will return false for duplicate replies.
/// Will return true if the email is not a reply.
///
/// # Arguments
///
/// * `email` - The `ParsedEmail` to be checked.
///
/// # Returns
///
/// A `Result` containing a boolean indicating if the request is valid.
pub async fn check_is_valid_request(email: &ParsedEmail, pool: &PgPool) -> Result<bool> {
    // Check if the email is a reply by looking for the "In-Reply-To" header
    let reply_message_id = match email
        .headers
        .get_header("In-Reply-To")
        .and_then(|v| v.first().cloned())
    {
        Some(id) => id,
        // Email is not a reply, so it's valid
        None => return Ok(true),
    };

    // Check if the reply is valid (not a duplicate) using the database
    let is_valid = is_valid_reply(pool, &reply_message_id).await?;
    Ok(is_valid)
}
