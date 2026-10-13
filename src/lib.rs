use zed_extension_api::{self as zed, Command, ContextServerId, Project, Result};

const BRIDGE_PACKAGE: &str = "mcp-remote";
const SERVER_URL: &str = "https://agentnexus.app/mcp";

struct AgentNexusExtension;

impl zed::Extension for AgentNexusExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let installed = zed::npm_package_installed_version(BRIDGE_PACKAGE)?;
        let latest = zed::npm_package_latest_version(BRIDGE_PACKAGE)?;
        if installed.as_deref() != Some(latest.as_str()) {
            zed::npm_install_package(BRIDGE_PACKAGE, &latest)?;
        }
        let script = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join("node_modules")
            .join(BRIDGE_PACKAGE)
            .join("dist")
            .join("proxy.js")
            .to_string_lossy()
            .to_string();
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![script, SERVER_URL.to_string()],
            env: vec![],
        })
    }
}

zed::register_extension!(AgentNexusExtension);
