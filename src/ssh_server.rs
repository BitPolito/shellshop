//! Embedded russh SSH server: one isolated TUI per session.

use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, mpsc};

use ratatui::Terminal;
use russh::keys::PublicKey;
use russh::server::{Auth, Config, Msg, Server, Session};
use russh::{Channel, ChannelId, Pty};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use crate::app::{terminal_for_ssh, App};

/// Async bridge: crossterm writes are batched on `flush` and sent on the SSH channel.
/// Pattern from russh `examples/ratatui_app.rs`.
struct TerminalHandle {
    sender: UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    async fn start(handle: russh::server::Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                if handle
                    .data(channel_id, bytes::Bytes::from(data))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });
        Self {
            sender,
            sink: Vec::new(),
        }
    }
}

impl Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.sink.is_empty() {
            return Ok(());
        }
        self.sender
            .send(std::mem::take(&mut self.sink))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))?;
        Ok(())
    }
}

/// Global listener state: assigns monotonic client ids.
#[derive(Default)]
pub struct ShellShopServer {
    next_id: AtomicUsize,
}

impl Server for ShellShopServer {
    type Handler = ClientHandler;

    fn new_client(&mut self, _: Option<std::net::SocketAddr>) -> Self::Handler {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        ClientHandler::new(id)
    }

    fn handle_session_error(&mut self, error: <Self::Handler as russh::server::Handler>::Error) {
        eprintln!("SSH session error: {error:#}");
    }
}

pub struct ClientHandler {
    id: usize,
    dims: Arc<Mutex<(u16, u16)>>,
    input_tx: Mutex<Option<mpsc::Sender<Vec<u8>>>>,
    shell_started: AtomicBool,
}

impl ClientHandler {
    fn new(id: usize) -> Self {
        Self {
            id,
            dims: Arc::new(Mutex::new((80, 24))),
            input_tx: Mutex::new(None),
            shell_started: AtomicBool::new(false),
        }
    }
}

impl russh::server::Handler for ClientHandler {
    type Error = anyhow::Error;

    async fn auth_none(&mut self, _user: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_password(&mut self, _user: &str, _password: &str) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _public_key: &PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
    }

    async fn channel_open_session(
        &mut self,
        _channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        {
            let mut d = self.dims.lock().unwrap();
            *d = (col_width as u16, row_height as u16);
        }
        session.channel_success(channel)?;
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        if self.shell_started.swap(true, Ordering::SeqCst) {
            session.channel_failure(channel)?;
            return Ok(());
        }

        let handle = session.handle();
        let close_handle = handle.clone();
        let terminal_handle = TerminalHandle::start(handle, channel).await;

        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        *self.input_tx.lock().unwrap() = Some(tx);

        let dims = self.dims.clone();
        let (cols, rows) = *dims.lock().unwrap();
        let client_id = self.id;
        let rt = tokio::runtime::Handle::current();

        std::thread::Builder::new()
            .name(format!("shellshop-tui-{client_id}"))
            .spawn(move || {
                let close_channel = |exit_status: u32| {
                    rt.block_on(async {
                        let _ = close_handle.exit_status_request(channel, exit_status).await;
                        let _ = close_handle.eof(channel).await;
                        let _ = close_handle.close(channel).await;
                    });
                };

                let mut terminal: Terminal<_> = match terminal_for_ssh(terminal_handle, cols, rows) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("terminal init (client {client_id}): {e}");
                        close_channel(1);
                        return;
                    }
                };
                let mut app = App::new(cols, rows);
                let exit = match app.run_over_channel(&mut terminal, &rx, &dims) {
                    Ok(()) => 0,
                    Err(e) => {
                        eprintln!("TUI exit (client {client_id}): {e}");
                        1
                    }
                };
                close_channel(exit);
            })
            .map_err(|e| anyhow::anyhow!("spawn TUI thread: {e}"))?;

        session.channel_success(channel)?;
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        {
            let mut d = self.dims.lock().unwrap();
            *d = (col_width as u16, row_height as u16);
        }
        session.channel_success(channel)?;
        Ok(())
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        if let Some(tx) = self.input_tx.lock().unwrap().as_ref() {
            let _ = tx.send(data.to_vec());
        }
        Ok(())
    }

    async fn channel_close(
        &mut self,
        _channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.input_tx.lock().unwrap().take();
        Ok(())
    }

    async fn channel_eof(
        &mut self,
        _channel: ChannelId,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.input_tx.lock().unwrap().take();
        Ok(())
    }
}

/// Build server config with sensible timeouts for interactive TUI use.
pub fn server_config(host_key: russh::keys::PrivateKey) -> Config {
    Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_millis(500),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![host_key],
        nodelay: true,
        ..Default::default()
    }
}
