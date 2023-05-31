use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{MsgState, beam_id::AppOrProxyId, serialize_time, MsgId, Msg, DecryptableMsg, Plain, Encrypted, EncryptableMsg, HasWaitId};




#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MsgSocketRequest<State>
where State: MsgState {
    pub from: AppOrProxyId,
    // TODO: Tell serde to serialize only one
    pub to: Vec<AppOrProxyId>,
    #[serde(with="serialize_time", rename="ttl")]
    pub expire: SystemTime,
    pub id: MsgId,
    pub secret: State,
    pub metadata: Value,
}

impl<State: MsgState> Msg for MsgSocketRequest<State> {
    fn get_from(&self) -> &AppOrProxyId {
        &self.from
    }

    fn get_to(&self) -> &Vec<AppOrProxyId> {
        &self.to
    }

    fn get_metadata(&self) -> &Value {
        &self.metadata
    }
}

impl DecryptableMsg for MsgSocketRequest<Encrypted> {
    type Output = MsgSocketRequest<Plain>;

    fn get_encryption(&self) -> Option<&Encrypted> {
        Some(&self.secret)
    }

    fn convert_self(self, body: String) -> Self::Output {
        let Self { from, to, expire, metadata, id, .. } = self;
        Self::Output { from, to, expire, secret: body.into(), metadata, id }
    }
}

impl EncryptableMsg for MsgSocketRequest<Plain> {
    type Output = MsgSocketRequest<Encrypted>;

    fn convert_self(self, body: Encrypted) -> Self::Output {
        let Self { from, to, expire, metadata, id, .. } = self;
        Self::Output { from, to, expire, metadata, secret: body, id }
    }

    fn get_plain(&self) -> &Plain {
        &self.secret
    }
}

impl<State: MsgState> HasWaitId<MsgId> for MsgSocketRequest<State> {
    fn wait_id(&self) -> MsgId {
        self.id
    }
}
