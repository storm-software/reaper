use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use lambda_http::{run, service_fn, Body, Error, Request};
use mime::Mime;
use std::env;
use std::str::FromStr;
use teloxide::types::ChatAction;
use teloxide::types::Message;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;
use teloxide::{net::Download, prelude::*};
use tracing::{debug, error, info, warn};
use tracing_subscriber::fmt;

const MAX_DURATION: u32 = 30 * 60;
const DEFAULT_DELAY: u64 = 5;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum BotCommand {
  #[command(description = "display this text.")]
  Help,
  #[command(description = "welcome message.")]
  Start,
  #[command(description = "send transaction request.")]
  Send,
  // #[command(description = "transcribe the replied audio file in English.")]
  // English,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Initialize tracing for logging
  fmt().with_max_level(tracing::Level::INFO).with_target(false).without_time().init();

  let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
  let config =
    aws_config::defaults(BehaviorVersion::v2023_11_09()).region(region_provider).load().await;

  let secrets = aws_sdk_secretsmanager::Client::new(&config);
  let telegram_bot_token = secrets
    .get_secret_value()
    .secret_id("storm-trading/eth-execution-telegram-bot-token")
    .send()
    .await?;
  // For a list of exceptions thrown, see
  // https://docs.aws.amazon.com/secretsmanager/latest/apireference/API_GetSecretValue.html

  // Setup telegram bot (we do it here because this place is a cold start)
  // let bot = Bot::new(env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set!"))
  let bot = Bot::new(telegram_bot_token.secret_string());

  // Setup AWS DynamoDB conn
  //   let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
  //   let config = aws_config::defaults(BehaviorVersion::latest()).region(region_provider).load().await;
  //   let dynamodb = aws_sdk_dynamodb::Client::new(&config);

  // Set commands
  let res = bot.set_my_commands(BotCommand::bot_commands()).await;

  if let Err(e) = res {
    warn!("Failed to set commands: {:?}", e);
  }

  // Run the Lambda function
  run(service_fn(|req| handler(req, &bot))).await

  //   run(service_fn(|req| handler(req, &bot, &dynamodb))).await
}

async fn handler(
  req: lambda_http::Request,
  bot: &Bot,
  //   dynamodb: &aws_sdk_dynamodb::Client,
) -> Result<lambda_http::Response<String>, lambda_http::Error> {
  // Parse JSON webhook
  let bot = bot.clone();

  let update = match parse_webhook(req).await {
    Ok(message) => message,
    Err(e) => {
      error!("Failed to parse webhook: {:?}", e);
      return Ok(
        lambda_http::Response::builder()
          .status(400)
          .body("Failed to parse webhook".into())
          .unwrap(),
      );
    }
  };

  // Handle commands
  if let UpdateKind::Message(message) = &update.kind {
    if let Some(text) = &message.text() {
      if let Ok(command) = BotCommand::parse(text, bot.get_me().await.unwrap().username()) {
        return handle_command(bot.clone(), message.clone(), command).await;
      }
    }
  }

  return Ok(
    lambda_http::Response::builder().status(400).body("Failed to process request".into()).unwrap(),
  );
}

async fn handle_command(
  bot: Bot,
  message: Message,
  command: BotCommand,
) -> Result<lambda_http::Response<String>, lambda_http::Error> {
  match command {
    BotCommand::Help => {
      bot.send_message(message.chat.id, BotCommand::descriptions().to_string()).await.unwrap();
    }
    BotCommand::Start => {
      bot.send_message(message.chat.id, "Welcome! Send a voice message or video note to transcribe it. You can also use /help to see all available commands. Currently there are no other commands available.")
                .await
                .unwrap();
    }
    BotCommand::Send => {
      bot
        .send_message(
          message.chat.id,
          "Request a transaction submission with /send and the transaction details.",
        )
        .await
        .unwrap();
    } // BotCommand::English => {
      //     bot.send_message(message.chat.id, "Please reply to an audio message with /english to transcribe it in English.")
      //         .await
      //         .unwrap();
      // }
  }

  Ok(lambda_http::Response::builder().status(200).body(String::new()).unwrap())
}

pub async fn parse_webhook(input: Request) -> Result<Update, Error> {
  let body = input.body();
  let body_str = match body {
    Body::Text(text) => text,
    not => panic!("expected Body::Text(...) got {not:?}"),
  };
  let body_json: Update = serde_json::from_str(body_str)?;
  debug!("Parsed webhook: {:?}", body_json);
  Ok(body_json)
}

pub async fn delete_message_delay(bot: &Bot, msg: &Message, delay: u64) {
  tokio::time::sleep(tokio::time::Duration::from_secs(delay)).await;
  bot.delete_message(msg.chat.id, msg.id).await.unwrap();
}

pub fn parse_groq_ratelimit_error(message: &str) -> Option<u32> {
  // // Body: Object {"error": Object {"code": String("rate_limit_exceeded"), "message": String("Rate limit reached for model `whisper-large-v3` in organization `xxx` on seconds of audio per hour (ASPH): Limit 7200, Used 7182, Requested 23. Please try again in 2.317999999s. Visit https://console.groq.com/docs/rate-limits for more information."), "type": String("seconds")}}
  let re = regex::Regex::new(r"Please try again in (\d+\.\d+)s").unwrap();
  let caps = re.captures(message).unwrap();
  let wait_for = caps.get(1).unwrap().as_str().parse::<f32>().unwrap();
  Some(wait_for as u32)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_groq_ratelimit_error() {
    let message = "Rate limit reached for model `whisper-large-v3` in organization `org_01htnj6w5pf0za49my0yj0sje5` on seconds of audio per hour (ASPH): Limit 7200, Used 7182, Requested 23. Please try again in 2.317999999s.";
    let wait_for = parse_groq_ratelimit_error(message).unwrap();
    assert_eq!(wait_for, 2);
  }
}
