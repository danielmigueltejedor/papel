//! Help ▸ Check for updates and the check at start. The flow and its windows are
//! `nodalia-update`'s, shared with the other Nodalia apps; this module supplies the preference, the
//! quit path and when an update may close the app.

#[cfg(not(target_arch = "wasm32"))]
use nodalia_update::egui::{Action, Updater};

use crate::PdfCraftApp;

/// Where every release is listed.
pub const RELEASES_PAGE: &str = "https://github.com/getnodalia/papel/releases";

/// Stands in for the network in tests.
#[cfg(not(target_arch = "wasm32"))]
pub type UpdateSource = nodalia_update::egui::Source;

pub(crate) struct Updates {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// Ask for a newer release at start, unless Preferences says not to; the desktop app turns
    /// it on, so tests never reach the network.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
}

impl Updates {
    /// An update window is open over the whole window: the offer, a check someone asked for, the
    /// download or its outcome. It leaves the menus on.
    pub(crate) fn prompt_open(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updater.is_open()
        }
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
    }
}

impl Default for Updates {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(crate::UPDATE_APP).with_accent([0xb7, 0x02, 0x28]),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
        }
    }
}

impl PdfCraftApp {
    /// Whether the app asks for a newer release on its first frame (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates.startup_check = on;
    }

    /// Replaces the network with `source`, and the wait before the check at start with none (tests).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_update_source(&mut self, source: UpdateSource) {
        self.updates.updater = Updater::new(crate::UPDATE_APP).with_source(source).with_stamp(None).with_startup_delay(std::time::Duration::ZERO);
    }

    /// Help ▸ Check for updates: ask for the latest release and show the outcome.
    pub fn check_for_updates(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.updates.check_requested = true;
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.open_url(RELEASES_PAGE);
        }
    }

    /// Starts the checks and picks up their answers (each frame).
    pub(crate) fn poll_updates(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let Some(ctx) = self.ctx.clone() else { return };
            // Settled on the first frame: a preference turned on later waits for the next start.
            let at_start = std::mem::take(&mut self.updates.startup_check) && self.update_check_at_start;
            let updates = &mut self.updates;
            if std::mem::take(&mut updates.check_requested) {
                updates.updater.check_now(&ctx);
            } else if at_start {
                updates.updater.check_at_start(&ctx);
            }
            let action = updates.updater.poll();
            act(self, &ctx, action);
        }
    }
}

/// Carries out what the update windows asked for.
#[cfg(not(target_arch = "wasm32"))]
fn act(app: &mut PdfCraftApp, ctx: &egui::Context, action: Action) {
    match action {
        Action::None => {}
        Action::OpenUrl(url) => app.open_url(&url),
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. The install
            // only ran once nothing was unsaved, so the quit asks nothing. Exit anyway if the app
            // has not ended soon.
            app.request_quit(ctx);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
    }
}

/// The update windows: the offer, the answer to Help ▸ Check for updates, the download and install.
pub(crate) fn dialog(app: &mut PdfCraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // An installation waits until no document holds unsaved work.
        let clean = app.first_dirty().is_none();
        let language = nodalia_update::Language::from_code(crate::i18n::current().code());
        let action = app.updates.updater.show(ctx, language, clean);
        act(app, ctx, action);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use nodalia_update::egui::Status;
    use pdfcraft_engine::{Edit, FieldValue};

    #[test]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/papel");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
        assert!(RELEASES_PAGE.starts_with("https://github.com/getnodalia/papel/"));
    }

    /// A downloaded update waits while a document holds unsaved work: nothing installs and the
    /// app stays open.
    #[test]
    fn a_ready_update_waits_for_unsaved_work() {
        let mut app = PdfCraftApp::new();
        app.open_bytes("source.pdf", None, include_bytes!("../../tests/data/form.pdf").to_vec()).unwrap();
        assert!(app.apply_edit(Edit::SetFieldValue { name: "name".into(), value: FieldValue::Text("Changed".into()) }));
        let release = nodalia_update::Release {
            tag: "v99.0.0".into(),
            version: "99.0.0".into(),
            notes: String::new(),
            page: "https://github.com/getnodalia/papel/releases/tag/v99.0.0".into(),
            asset: None,
        };
        app.updates.updater.set_status(Status::Ready { release, package: std::path::PathBuf::from("/nowhere") });
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| dialog(&mut app, ui.ctx())).textures_delta.clear();
        assert!(matches!(app.updates.updater.status(), Status::Ready { .. }));
        assert!(app.close_request.is_none());
        assert!(!app.allow_quit);
    }
}
