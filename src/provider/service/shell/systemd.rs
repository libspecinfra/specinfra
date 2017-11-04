use std::result::Result;

use backend::Backend;
use backend::command::Command;
use provider::error::Error;
use provider::Output;
use provider::service::shell::ShellProvider;

#[derive(Clone, Debug)]
pub struct Systemd;

impl ShellProvider for Systemd {
    fn is_running(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl is-active {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn is_enabled(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl is-enabled {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn enable(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl enable {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn disable(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl disable {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn start(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl start {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn reload(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl reload {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn restart(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl restart {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn stop(&self, name: &str, b: &Backend) -> Result<Output, Error> {
        let c = Command::new(format!("systemctl stop {}", name));
        let success = match b.run_command(c) {
            Ok(r) => r.success,
            Err(_) => false,
        };
        Ok(Output::Bool(success))
    }

    fn box_clone(&self) -> Box<ShellProvider> {
        Box::new((*self).clone())
    }
}
