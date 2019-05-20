use crate::core::DtnPeer;
use config::{Config, File};
use log::debug;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct DtnConfig {
    pub debug: bool,
    pub nodeid: String,
    pub announcement_interval: u64,
    pub janitor_interval: u64,
    pub endpoints: Vec<String>,
    pub clas: Vec<String>,
    pub routing: String,
    pub peer_timeout: u64,
    pub statics: Vec<DtnPeer>,
}

pub fn rnd_node_name() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(10).collect()
}

impl From<PathBuf> for DtnConfig {
    fn from(item: PathBuf) -> Self {
        let mut dtncfg = DtnConfig::new();
        let mut s = Config::default();

        debug!("Loading config: {}", item.to_str().unwrap());

        // Start off by merging in the "default" configuration file
        s.merge(File::new(item.to_str().unwrap(), config::FileFormat::Toml))
            .unwrap();
        dtncfg.debug = s.get_bool("debug").unwrap_or(false);
        if dtncfg.debug {
            //std::env::set_var("RUST_LOG", "dtn7=debug,dtnd=debug");
        }
        debug!("debug: {:?}", dtncfg.debug);

        dtncfg.nodeid = s.get_str("nodeid").unwrap_or(dtncfg.nodeid);
        debug!("nodeid: {:?}", dtncfg.nodeid);

        dtncfg.routing = s.get_str("routing").unwrap_or(dtncfg.routing);
        debug!("routing: {:?}", dtncfg.routing);

        dtncfg.janitor_interval = s
            .get_int("core.janitor")
            .unwrap_or(dtncfg.janitor_interval as i64) as u64;
        debug!("janitor: {:?}", dtncfg.janitor_interval);

        dtncfg.announcement_interval =
            s.get_int("discovery.interval")
                .unwrap_or(dtncfg.announcement_interval as i64) as u64;
        debug!("discovery-interval: {:?}", dtncfg.announcement_interval);
        dtncfg.peer_timeout = s
            .get_int("discovery.peer-timeout")
            .unwrap_or(dtncfg.peer_timeout as i64) as u64;
        debug!("discovery-peer-timeout: {:?}", dtncfg.peer_timeout);

        let peers = s.get_array("statics.peers");
        if peers.is_ok() {
            for m in peers.unwrap().iter() {
                let peer: DtnPeer =
                    crate::core::helpers::parse_peer_url(&m.clone().into_str().unwrap());
                debug!("Peer: {:?}", peer);
                dtncfg.statics.push(peer);
            }
        }
        let endpoints = s.get_table("endpoints.eid");
        if endpoints.is_ok() {
            for (_k, v) in endpoints.unwrap().iter() {
                let eid = v.clone().into_str().unwrap();
                debug!("EID: {:?}", eid);
                dtncfg.endpoints.push(eid);
            }
        }

        let clas = s.get_table("convergencylayers.cla");
        if clas.is_ok() {
            for (k, v) in clas.unwrap().iter() {
                let tab = v.clone().into_table().unwrap();
                let cla_id = tab["id"].clone().into_str().unwrap();
                if crate::cla::convergency_layer_agents().contains(&cla_id.as_str()) {
                    debug!("CLA: {:?}", cla_id);
                    dtncfg.clas.push(cla_id);
                }
            }
        }
        dtncfg
    }
}

impl DtnConfig {
    pub fn new() -> DtnConfig {
        let node_rnd: String = rnd_node_name();
        DtnConfig {
            debug: false,
            nodeid: node_rnd,
            announcement_interval: 2000,
            janitor_interval: 10000,
            endpoints: Vec::new(),
            clas: Vec::new(),
            routing: "epidemic".into(),
            peer_timeout: 2000 * 10,
            statics: Vec::new(),
        }
    }
    pub fn set(&mut self, cfg: DtnConfig) {
        self.debug = cfg.debug;
        self.nodeid = cfg.nodeid;
        self.announcement_interval = cfg.announcement_interval;
        self.janitor_interval = cfg.janitor_interval;
        self.endpoints = cfg.endpoints;
        self.clas = cfg.clas;
        self.routing = cfg.routing;
        self.peer_timeout = cfg.peer_timeout;
        self.statics = cfg.statics;
    }
}