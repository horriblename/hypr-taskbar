use hyprland::{
    data::WorkspaceBasic,
    event_listener::{WindowMoveEvent, WindowOpenEvent},
    shared::{Address, HResult, HyprData, WorkspaceType},
};
use icon_lookup::guess_icon;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

// TODO change to something more sensible
const DEFAULT_ICON: &str = "/usr/share/icons/breeze/categories/32/applications-all.svg";

mod icon_lookup;

#[derive(Serialize, Deserialize)]
pub struct Task {
    class: String,
    title: String,
    icon_path: PathBuf,
    address: hyprland::shared::Address,
}

pub struct Taskbar {
    active_workspace: WorkspaceBasic,
}

impl Taskbar {
    pub fn new() -> Result<Taskbar, Box<dyn Error>> {
        // TODO multi-monitor/default monitor

        let mon = hyprland::data::Monitors::get()?;
        Ok(Taskbar {
            active_workspace: mon.collect::<Vec<_>>().remove(0).active_workspace,
        })
    }

    fn update_active_workspace(&mut self) {
        // TODO multi-monitor/default monitor
        if let Ok(mon) = hyprland::data::Monitors::get() {
            self.active_workspace = mon.collect::<Vec<_>>().remove(0).active_workspace
        } else {
            // TODO log error
        }
    }

    /// Query hyprland and return a list of `Task`s of the current workspace
    /// @return `Result<Vec<Task>>` list of tasks in the current workspace
    fn get_tasks(&self) -> HResult<Vec<Task>> {
        let ws = &self.active_workspace.id;
        let apps = hyprland::data::Clients::get()?;

        Ok(apps
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|app| &app.workspace.id == ws)
            .map(|task| {
                let icon = guess_icon(&task.class);
                Task {
                    class: task.class,
                    title: task.title,
                    icon_path: icon,
                    address: task.address,
                }
            })
            .collect())
    }

    /// Query hyprland for current tasks then print it in json
    fn sync_taskbar(&self) {
        let tasks = match self.get_tasks() {
            Ok(tasks) => tasks,
            Err(_err) => return, // TODO log error
        };

        let s = match serde_json::to_string(&tasks) {
            Ok(s) => s,
            Err(_err) => return, // TODO log error
        };

        println!("{}", s)
    }

    pub fn on_workspace_change(&mut self, _ws: WorkspaceType) {
        self.update_active_workspace();
        self.sync_taskbar();
    }

    pub fn on_window_open(&mut self, _: WindowOpenEvent) {
        self.sync_taskbar();
    }

    pub fn on_window_close(&mut self, _: Address) {
        self.sync_taskbar();
    }

    pub fn on_window_moved(&mut self, _: WindowMoveEvent) {
        self.sync_taskbar();
    }
}
