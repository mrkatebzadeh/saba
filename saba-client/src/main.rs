/* main.rs

*
* Author: M.R.Siavash Katebzadeh <mr@katebzadeh.xyz>
* Keywords: Rust
* Version: 0.0.1
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

mod config;
mod signal;
use std::thread;

use crate::config::Config;
use log::{debug, error, info};
use saba_core::client::ConnectionManager;
use tokio::runtime::Runtime;
extern crate daemonize;
use daemonize::Daemonize;
use std::fs::File;
use std::process;

fn main() -> std::io::Result<()> {
    let stdout = File::create("/tmp/saba_interface.out").unwrap();
    let stderr = File::create("/tmp/saba_interface.err").unwrap();
    let pid = "/tmp/saba_interface.pid";

    let config = Config::new();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            match config.verbose {
                0 => simplelog::LevelFilter::Warn,
                1 => simplelog::LevelFilter::Info,
                2 => simplelog::LevelFilter::Debug,
                _ => simplelog::LevelFilter::Trace,
            },
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            stderr,
        ),
    ])
    .unwrap();
    debug!("Config: {:?}", config);

    let daemonize = Daemonize::new()
        .pid_file(pid) // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout)
        // .stderr(stderr)
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            let signal_handler = thread::spawn(move || {
                signal::register_exit_signal(pid).unwrap();
            });

            info!("Saba started");

            {
                let ip = config.ip.clone();
                let port = config.port;
                thread::spawn(move || {
                    let controller_addr = format!("http://{ip}:{port}");
                    let runtime = match Runtime::new() {
                        Ok(runtime) => runtime,
                        Err(err) => {
                            error!("Failed to create Tokio runtime: {err}");
                            return;
                        }
                    };

                    if let Err(err) = runtime.block_on(async move {
                        let manager = ConnectionManager::connect("App1", controller_addr).await?;
                        let priority = manager.register().await?;
                        info!("Registered App1 with priority {priority}");
                        manager.deregister().await
                    }) {
                        error!("Connection manager error: {err}");
                    }
                });
            }
            signal_handler.join().unwrap();
            process::exit(1);
        }
        Err(e) => error!("Failed to daemonize, {}", e),
    }
    Ok(())
}

/* main.rs ends here */
