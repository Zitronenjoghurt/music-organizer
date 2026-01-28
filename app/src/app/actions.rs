use crate::runtime::file_picker::FilePickTarget;
use std::cell::RefCell;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AppAction {
    FilePicked {
        path: PathBuf,
        target: FilePickTarget,
    },
    FilesPicked {
        paths: Vec<PathBuf>,
        target: FilePickTarget,
    },
    ToastError(String),
    ToastSuccess(String),
    ToastWarning(String),
}

#[derive(Default)]
pub struct AppActions {
    queue: RefCell<Vec<AppAction>>,
}

impl AppActions {
    pub fn take_actions(&self) -> Vec<AppAction> {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.drain(..).collect()
        } else {
            vec![]
        }
    }

    pub fn push_action(&self, action: AppAction) {
        if let Ok(mut queue) = self.queue.try_borrow_mut() {
            queue.push(action);
        }
    }

    pub fn file_picked(&self, path: PathBuf, target: FilePickTarget) {
        self.push_action(AppAction::FilePicked { path, target });
    }

    pub fn files_picked(&self, paths: Vec<PathBuf>, target: FilePickTarget) {
        self.push_action(AppAction::FilesPicked { paths, target });
    }

    pub fn toast_error(&self, message: impl Into<String>) {
        self.push_action(AppAction::ToastError(message.into()));
    }

    pub fn toast_success(&self, message: impl Into<String>) {
        self.push_action(AppAction::ToastSuccess(message.into()));
    }

    pub fn toast_warning(&self, message: impl Into<String>) {
        self.push_action(AppAction::ToastWarning(message.into()));
    }
}
