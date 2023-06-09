use std::{io::Write, process::Stdio};

use serde::{Deserialize, Serialize};

use crate::{error::OhNo, response::Response};

#[derive(Debug, Deserialize, Serialize)]
pub struct KakSession {
  pub session_name: String,
  pub client_name: Option<String>,
}

impl KakSession {
  pub fn new(session_name: impl Into<String>, client_name: impl Into<Option<String>>) -> Self {
    Self {
      session_name: session_name.into(),
      client_name: client_name.into(),
    }
  }

  pub fn send_response(&mut self, resp: &Response) -> Result<(), OhNo> {
    let resp = resp.to_kak_cmd(self.client_name.as_deref());

    match resp {
      Some(resp) if !resp.is_empty() => self.send_response_raw(resp),

      _ => Ok(()),
    }
  }

  pub fn send_response_raw(&mut self, resp: impl AsRef<str>) -> Result<(), OhNo> {
    let resp = resp.as_ref();

    let child = std::process::Command::new("kak")
      .args(["-p", self.session_name.as_str()])
      .stdin(Stdio::piped())
      .spawn()
      .map_err(|err| OhNo::CannotSendRequest {
        err: err.to_string(),
      })?;
    let mut child_stdin = child.stdin.ok_or_else(|| OhNo::CannotSendRequest {
      err: "cannot pipe data to kak -p".to_owned(),
    })?;
    child_stdin
      .write_all(resp.as_bytes())
      .map_err(|err| OhNo::CannotSendRequest {
        err: err.to_string(),
      })?;
    child_stdin.flush().map_err(|err| OhNo::CannotSendRequest {
      err: err.to_string(),
    })
  }
}
