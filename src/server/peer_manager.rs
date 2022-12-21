#[derive(Debug)]
pub struct PeerManager {
    peers: Vec<String>,
}

impl crate::consensus_module::PeerClient for PeerManager {
    fn request_vote(&self, _peer_id: String) {
        todo!()
    }

    fn append_entries(&self, _peer_id: String) {
        todo!()
    }

    fn get_peer_ids(&self) -> Vec<String> {
        self.peers.clone()
    }
}

impl PeerManager {
    pub fn new(peers: Vec<String>) -> Self {
        Self { peers }
    }
}
