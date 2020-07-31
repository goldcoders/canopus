use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct DoSomethingPayload {
  state: String,
  data: u64,
}

// The commands definitions
// Deserialized from JS
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Cmd {
  DoSomething {
    count: u64,
    payload: DoSomethingPayload,
    callback: String,
    error: String,
  },
}

#[derive(Serialize)]
struct Response<'a> {
  value: u64,
  message: &'a str,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            DoSomething { count, payload, callback, error } => tauri::execute_promise(
              _webview,
              move || {
                if count > 5 {
                  let response = Response {
                    value: 5,
                    message: "You Are Good To Go!",
                  };
                  Ok(response)
                } else {
                  Err(CommandError::new("you only have 5 credits left").into())
                }
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
