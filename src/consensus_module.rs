pub trait PeerClient: Sync + Send + std::fmt::Debug {
    fn request_vote(&self, peer_id: String);
    fn append_entries(&self, peer_id: String);
    fn get_peer_ids(&self) -> Vec<String>;
}

#[derive(Debug)]
pub struct ConsensusModule<T>
where
    T: PeerClient,
{
    peer_client: T,
}

impl<T> ConsensusModule<T>
where
    T: PeerClient,
{
    pub fn new(peer_client: T) -> Self {
        Self { peer_client }
    }
}
