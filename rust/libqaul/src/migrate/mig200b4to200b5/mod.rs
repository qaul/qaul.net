use super::backup;
use std::path::Path;

pub mod config200b4;

/// migrate beta.4 into beta.5
/// todo: migrate config.internet.peers<InternetPeer>
///       InternetPeer structure added name field
pub struct Mig200b4To200b5 {}
impl Mig200b4To200b5 {
    /// process migrating
    pub fn do_process(old_path: &str, new_path: &str, new_version: &str) -> bool {
        //cleanup dest
        backup::Backup::remove_folder(new_path);

        //create dest
        if let Err(_) = std::fs::create_dir(new_path) {
            println!("failed to creating destinaton folder");
            return false;
        }

        //move unchanged contents
        println!("\t\tmove contents..");
        if Self::move_contents(old_path, new_path) == false {
            return false;
        }

        //create new version file
        println!("\t\tcreate version file..");
        let path = Path::new(new_path).join("version");
        if let Err(_) = std::fs::write(path, new_version) {
            println!("failed to create version file!");
        }

        //update config.yaml
        println!("\t\tmigrating config.yaml..");
        Self::migrate_config(old_path, new_path)
    }

    /// migrate config structure
    fn migrate_config(old_path: &str, new_path: &str) -> bool {
        //load old config
        if let Some(old_cfg) = config200b4::Configuration::load(
            Path::new(old_path).join("config.yaml").to_str().unwrap(),
        ) {
            let node = super::super::storage::configuration::Node {
                initialized: old_cfg.node.initialized,
                id: old_cfg.node.id.clone(),
                keys: old_cfg.node.keys.clone(),
            };
            let lan = super::super::storage::configuration::Lan {
                active: old_cfg.lan.active,
                listen: old_cfg.lan.listen,
            };

            let mut peers: Vec<super::super::storage::configuration::InternetPeer> = vec![];
            for peer in &old_cfg.internet.peers {
                peers.push(super::super::storage::configuration::InternetPeer {
                    address: peer.address.clone(),
                    name: String::from("undefined"),
                    enabled: peer.enabled,
                });
            }
            let internet = super::super::storage::configuration::Internet {
                active: old_cfg.internet.active,
                peers,
                do_listen: old_cfg.internet.do_listen,
                listen: old_cfg.internet.listen,
            };

            let mut user_accounts: Vec<super::super::storage::configuration::UserAccount> = vec![];
            for user in &old_cfg.user_accounts {
                user_accounts.push(super::super::storage::configuration::UserAccount {
                    name: user.name.clone(),
                    id: user.id.clone(),
                    keys: user.keys.clone(),
                    storage: super::super::storage::configuration::StorageOptions {
                        users: user.storage.users.clone(),
                        size_total: user.storage.size_total,
                    },
                });
            }

            let debug = super::super::storage::configuration::DebugOption {
                log: old_cfg.debug.log,
            };
            let routing = super::super::storage::configuration::RoutingOptions {
                sending_table_period: old_cfg.routing.sending_table_period,
                ping_neighbour_period: old_cfg.routing.ping_neighbour_period,
                hop_count_penalty: old_cfg.routing.hop_count_penalty,
                maintain_period_limit: old_cfg.routing.maintain_period_limit,
            };

            let new_config = super::super::storage::configuration::Configuration {
                node,
                lan,
                internet,
                user_accounts,
                debug,
                routing,
            };

            if let Ok(yaml) = serde_yaml::to_string(&new_config) {
                if let Err(_) = std::fs::write(Path::new(new_path).join("config.yaml"), yaml) {
                    println!("Error: creating config.yaml");
                    return false;
                }
            } else {
                println!("Error: config serialize");
                return false;
            }
            return true;
        }

        false
    }

    /// move unchanged contents
    fn move_contents(old_path: &str, new_path: &str) -> bool {
        let mut files: Vec<String> = vec![];
        let mut folders: Vec<String> = vec![];
        for entry_res in std::fs::read_dir(old_path).unwrap() {
            let entry = entry_res.unwrap();
            let file_name_buf = entry.file_name();
            let file_name = file_name_buf.to_str().unwrap();
            if entry.file_type().unwrap().is_dir() {
                if file_name.starts_with(".") {
                    continue;
                }
                let path = String::from(file_name);
                folders.push(path);
            } else {
                if file_name == "version" || file_name == "config.yaml" {
                    continue;
                }
                let path = String::from(file_name);
                files.push(path);
            }
        }
        if backup::Backup::move_files(&files, old_path, new_path) == false {
            return false;
        }
        backup::Backup::move_folders(&folders, old_path, new_path)
    }
}
