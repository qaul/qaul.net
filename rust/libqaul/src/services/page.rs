use libp2p::swarm::Swarm;
use serde::{Deserialize, Serialize};
// Async comparison
// https://runrust.miraheze.org/wiki/Async_crate_comparison
// MPSC = Multi-Producer, Single-Consumer FiFo
use async_std::{fs, task};
use futures::channel::mpsc;

use crate::connections::internet::QaulInternetBehaviour;
use crate::connections::lan::QaulLanBehaviour;
use crate::node::Node;
use crate::types::{QaulMessage, QaulMessageType};

const STORAGE_FILE_PATH: &str = "./pages.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;
pub type Pages = Vec<Page>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    id: usize,
    title: String,
    description: String,
    content: String,
    public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PageMode {
    ALL,
    One(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    pub mode: PageMode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResponse {
    pub mode: PageMode,
    pub data: Pages,
}

pub fn respond_with_public_pages(sender: mpsc::UnboundedSender<QaulMessage>, receiver: String) {
    task::spawn(async move {
        match read_local_pages().await {
            Ok(pages) => {
                let page_resp = PageResponse {
                    mode: PageMode::ALL,
                    data: pages.into_iter().filter(|r| r.public).collect(),
                };
                let resp = QaulMessage {
                    sender: "".to_string(),
                    receiver,
                    data: QaulMessageType::Page(page_resp),
                };
                if let Err(e) = sender.unbounded_send(resp) {
                    log::error!("error sending response via channel, {}", e);
                }
            }
            Err(e) => log::error!("error fetching local pages to answer All request, {}", e),
        }
    });
}

#[allow(dead_code)]
pub async fn handle_create_page(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("p create") {
        let elements: Vec<&str> = rest.split("|").collect();
        if elements.len() < 3 {
            log::trace!("too few arguments - Format: title|description|content");
        } else {
            let title = elements.get(0).expect("title is there");
            let description = elements.get(1).expect("description is there");
            let content = elements.get(2).expect("content is there");
            if let Err(e) = create_new_page(title, description, content).await {
                log::error!("error creating page: {}", e);
            };
        }
    }
}

#[allow(dead_code)]
pub async fn handle_publish_page(cmd: &str) {
    if let Some(rest) = cmd.strip_prefix("p publish") {
        match rest.trim().parse::<usize>() {
            Ok(id) => {
                if let Err(e) = publish_page(id).await {
                    log::trace!("error publishing page with id {}, {}", id, e);
                } else {
                    log::trace!("Publish Page with id: {}", id);
                }
            }
            Err(e) => log::error!("invalid id: {}, {}", rest.trim(), e),
        }
    }
}

pub async fn create_new_page(title: &str, description: &str, content: &str) -> Result<()> {
    let mut local_pages = read_local_pages().await?;
    let new_id = match local_pages.iter().max_by_key(|r| r.id) {
        Some(v) => v.id + 1,
        None => 0,
    };
    local_pages.push(Page {
        id: new_id,
        title: title.to_owned(),
        description: description.to_owned(),
        content: content.to_owned(),
        public: false,
    });
    write_local_pages(&local_pages).await?;

    log::trace!("Created page:");
    log::trace!("Title: {}", title);
    log::trace!("Description: {}", description);
    log::trace!("Content: {}", content);

    Ok(())
}

pub async fn publish_page(id: usize) -> Result<()> {
    let mut local_pages = read_local_pages().await?;
    local_pages
        .iter_mut()
        .filter(|r| r.id == id)
        .for_each(|r| r.public = true);
    write_local_pages(&local_pages).await?;
    Ok(())
}

async fn read_local_pages() -> Result<Pages> {
    let content = fs::read(STORAGE_FILE_PATH).await?;
    let result = serde_json::from_slice(&content)?;
    Ok(result)
}

async fn write_local_pages(pages: &Pages) -> Result<()> {
    let json = serde_json::to_string(&pages)?;
    fs::write(STORAGE_FILE_PATH, &json).await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn handle_list_pages(
    cmd: &str,
    lan: &mut Swarm<QaulLanBehaviour>,
    internet: &mut Swarm<QaulInternetBehaviour>,
) {
    let rest = cmd.strip_prefix("p ls ");
    match rest {
        Some("all") => {
            let req = PageRequest {
                mode: PageMode::ALL,
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            lan.behaviour_mut()
                .floodsub
                .publish(Node::get_topic(), json.as_bytes());
            internet
                .behaviour_mut()
                .floodsub
                .publish(Node::get_topic(), json.as_bytes());
        }
        Some(pages_peer_id) => {
            let req = PageRequest {
                mode: PageMode::One(pages_peer_id.to_owned()),
            };
            let json = serde_json::to_string(&req).expect("can jsonify request");
            lan.behaviour_mut()
                .floodsub
                .publish(Node::get_topic(), json.as_bytes());
            internet
                .behaviour_mut()
                .floodsub
                .publish(Node::get_topic(), json.as_bytes());
        }
        None => {
            match read_local_pages().await {
                Ok(v) => {
                    log::trace!("Local Pages ({})", v.len());
                    v.iter().for_each(|r| log::trace!("{:?}", r));
                }
                Err(e) => log::error!("error fetching local pages: {}", e),
            };
        }
    }
}
