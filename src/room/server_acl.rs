//! Types for the *m.room.server_acl* event.

use ruma_events_macros::ruma_event;

ruma_event! {
    /// An event to indicate which servers are permitted to participate in the room.
    ServerAclEvent {
        kind: StateEvent,
        event_type: "m.room.server_acl",
        content: {
            /// True to allow server names that are IP address literals. False to deny.
            ///
            /// This is strongly recommended to be set to false as servers running with IP literal
            /// names are strongly discouraged in order to require legitimate homeservers to be
            /// backed by a valid registered domain name.
            #[serde(
                default = "ruma_serde::default_true",
                skip_serializing_if = "ruma_serde::is_true"
            )]
            pub allow_ip_literals: bool,

            /// The server names to allow in the room, excluding any port information. Wildcards may
            /// be used to cover a wider range of hosts, where `*` matches zero or more characters
            /// and `?` matches exactly one character.
            ///
            /// **This defaults to an empty list when not provided, effectively disallowing every
            /// server.**
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub allow: Vec<String>,

            /// The server names to disallow in the room, excluding any port information. Wildcards may
            /// be used to cover a wider range of hosts, where * matches zero or more characters and ?
            /// matches exactly one character.
            ///
            /// This defaults to an empty list when not provided.
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub deny: Vec<String>,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_value as from_json_value, json};

    use super::ServerAclEvent;
    use crate::EventJson;

    #[test]
    fn default_values() {
        let json_data = json!({
            "content": {},
            "event_id": "$h29iv0s8:example.com","origin_server_ts":1,
            "sender": "@carl:example.com",
            "state_key": "",
            "type": "m.room.server_acl"
        });
        let server_acl_event: ServerAclEvent = from_json_value::<EventJson<_>>(json_data)
            .unwrap()
            .deserialize()
            .unwrap();

        assert_eq!(server_acl_event.content.allow_ip_literals, true);
        assert!(server_acl_event.content.allow.is_empty());
        assert!(server_acl_event.content.deny.is_empty());
    }
}
