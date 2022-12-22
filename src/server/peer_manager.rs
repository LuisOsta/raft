use crate::consensus_module::ReplicaID;

#[derive(Debug)]
pub struct PeerManager {
    peers: Vec<String>,
    server_id: String,
}

impl crate::consensus_module::PeerClient for PeerManager {
    fn request_vote(&self, _peer_id: ReplicaID) {
        todo!()
    }

    fn append_entries(&self, _peer_id: ReplicaID) {
        todo!()
    }

    fn get_peer_ids(&self) -> Vec<ReplicaID> {
        self.peers.clone()
    }

    fn get_id(&self) -> ReplicaID {
        self.server_id.clone()
    }
}

impl PeerManager {
    pub fn new(peers: Vec<String>, server_id: String) -> Self {
        Self { peers, server_id }
    }
}
