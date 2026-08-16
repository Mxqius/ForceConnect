use std::sync::Arc;

use tokio::sync::Mutex;

use crate::elevate::client::HelperSession;

/// Application state shared across commands.
#[derive(Default)]
pub struct AppState {
    /// The elevated helper, established lazily.
    ///
    /// `None` until the first privileged action, which is what keeps browsing
    /// the app free of UAC prompts. The mutex is held across the whole
    /// establish sequence so two commands racing to connect cannot each show
    /// the user a prompt.
    helper: Mutex<Option<Arc<HelperSession>>>,
}

impl AppState {
    /// Return the live session, or start one — prompting for elevation.
    pub async fn helper(
        &self,
        helper_path: std::path::PathBuf,
        parent_hwnd: isize,
    ) -> Result<Arc<HelperSession>, crate::elevate::ElevationError> {
        let mut guard = self.helper.lock().await;

        if let Some(existing) = guard.as_ref() {
            return Ok(Arc::clone(existing));
        }

        let session = HelperSession::establish(helper_path, parent_hwnd).await?;
        *guard = Some(Arc::clone(&session));
        Ok(session)
    }

    /// Version and process id of the live helper, if there is one.
    pub async fn helper_info(&self) -> Option<(String, u32)> {
        self.helper
            .lock()
            .await
            .as_ref()
            .map(|session| (session.helper_version.clone(), session.pid))
    }

    /// Drop the session, so the next privileged call re-establishes it.
    ///
    /// Used when the pipe breaks — the helper exiting is normal on parent
    /// death, but a mid-session disconnect means the cached handle is dead and
    /// keeping it would fail every subsequent call.
    pub async fn drop_helper(&self) {
        self.helper.lock().await.take();
    }
}
