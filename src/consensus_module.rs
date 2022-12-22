use tracing::{info, warn};

pub type ReplicaID = String;

pub trait PeerClient: Sync + Send + std::fmt::Debug {
    fn request_vote(&self, peer_id: ReplicaID);
    fn append_entries(&self, peer_id: ReplicaID);
    fn get_peer_ids(&self) -> Vec<ReplicaID>;
    fn get_id(&self) -> ReplicaID;
}

#[derive(Debug, PartialEq, Eq)]
enum ReplicaType {
    Leader,
    Follower,
    Candidate,
    Dead,
}

#[derive(Debug)]
pub struct ReplicaState {
    state: ReplicaType,
    current_term: u32,
    voted_for: Option<ReplicaID>,
}

#[derive(Debug)]
pub struct ConsensusModule<T>
where
    T: PeerClient,
{
    peer_client: T,
    replica: tokio::sync::RwLock<ReplicaState>,
}

impl<T> ConsensusModule<T>
where
    T: PeerClient,
{
    pub fn new(peer_client: T) -> Self {
        let cm = Self {
            peer_client,
            replica: tokio::sync::RwLock::new(ReplicaState {
                state: ReplicaType::Follower,
                current_term: 0,
                voted_for: None,
            }),
        };

        cm
    }
}

impl<T> ConsensusModule<T>
where
    T: PeerClient,
{
    // Runs in followers & candidates to make sure a new leader is chosen in the case of an election
    // Leaders and dead replicas will not run the election timer
    pub async fn run_election_timer(&self) {
        let term_started = self.replica.read().await.current_term;
        // this loop should continue forever or until:
        // 1.) This replica becomes a leader or dead
        // 2.) A new term has shown up (why?)
        // 3.) The election timeout expires and this Consensus Module becomes a candidate

        loop {
            let replica = self.replica.read().await;

            if replica.state == ReplicaType::Dead || replica.state == ReplicaType::Leader {
                //dead nodes and leaders should not do election timers
                warn!(state = ?replica.state, "In run_election_timer. Bailing due to incorrect replica state");
                return;
            }

            if replica.current_term != term_started {
                warn!(
                    "In run_election_timer. Term changed from {:?} to {:?}, bailing out",
                    term_started, replica.current_term
                );
                return;
            }

            // if its been more than X amount of time since the last election reset event
        }
    }

    fn generate_election_timeout() -> tokio::time::Duration {
        tokio::time::Duration::from_millis(150)
    }
}
