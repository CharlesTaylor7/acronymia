use crate::types::{PlayerId, SessionId};
use ::std::collections::HashMap;

#[cfg(not(feature = "impersonation"))]
#[derive(Debug, Default)]
pub struct Sessions {
    session_ids: HashMap<PlayerId, SessionId>,
    player_ids: HashMap<SessionId, PlayerId>,
}

#[cfg(not(feature = "impersonation"))]
impl Sessions {
    pub fn new() -> Sessions {
        Sessions {
            session_ids: HashMap::new(),
            player_ids: HashMap::new(),
        }
    }

    pub fn connect(&mut self, session_id: SessionId, player_id: PlayerId) -> Result<(), SessionId> {
        use ::std::collections::hash_map;
        match self.session_ids.entry(player_id.clone()) {
            hash_map::Entry::Vacant(entry) => {
                entry.insert(session_id.clone());
                self.player_ids.insert(session_id, player_id);
                Ok(())
            }
            hash_map::Entry::Occupied(_) => Err(session_id),
        }
    }

    pub fn remove(&mut self, session_id: &SessionId) {
        let player_id = self.player_ids.remove(session_id);
        if let Some(player_id) = player_id {
            self.session_ids.remove(&player_id);
        }
    }

    pub fn player_id(&self, session_id: &SessionId) -> Option<&PlayerId> {
        self.player_ids.get(session_id)
    }
}

/// Compatibility layer around a single hashmap
/// The regular sessions struct actually prevents stolen identity
#[cfg(feature = "impersonation")]
#[derive(Debug, Default)]
pub struct Sessions {
    player_ids: HashMap<SessionId, PlayerId>,
}

#[cfg(feature = "impersonation")]
impl Sessions {
    pub fn new() -> Sessions {
        Sessions {
            player_ids: HashMap::new(),
        }
    }

    pub fn connect(&mut self, session_id: SessionId, player_id: PlayerId) -> Result<(), SessionId> {
        self.player_ids.insert(session_id, player_id);
        Ok(())
    }

    pub fn remove(&mut self, session_id: &SessionId) {
        self.player_ids.remove(session_id);
    }

    pub fn player_id(&self, session_id: &SessionId) -> Option<&PlayerId> {
        self.player_ids.get(session_id)
    }
}
