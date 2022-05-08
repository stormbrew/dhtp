use std::{path::Path, ops::Deref};

use anyhow::Result;
use ssh_key::{known_hosts, public};

/// Represents a peer identified primarily by a public key identity.
pub struct Peer {
    name: Option<String>,
    identities: Vec<ssh_key::public::PublicKey>,
}

trait IdentityEntry {
    /// If the name in the argument matches this entry, return its public key
    fn match_name(&self, name: &str) -> Vec<ssh_key::public::PublicKey>;
}
impl IdentityEntry for known_hosts::Entry {
    fn match_name(&self, name: &str) -> Vec<public::PublicKey> {
        use ssh_key::known_hosts::*;

        match (self.marker(), self.host_patterns()) {
            (None, HostPatterns::Patterns(patterns)) => patterns
                .iter()
                .find(|pattern| pattern == &name)
                .map(|_| self.public_key().clone())
                .iter()
                .collect(),
            (None, HostPatterns::HashedName {..}) => todo!("Implement hashed name parsing"),
            _ => None,
        }
    }
}

impl Peer {


    /// Find a peer description by its hostname in the specified known_hosts files
    pub fn search_known_hosts<P: AsRef<Path>>(in_known_hosts: impl IntoIterator<Item=P>, hostname: &str) -> Result<Self> {
        use ssh_key::known_hosts::*;

        let identities = in_known_hosts
            .into_iter()
            .flat_map(|hosts_file| {
                if let Ok(file) = KnownHosts::read_file(hosts_file) {
                    file.iter().filter_map(|entry| entry.match_name(hostname)).cloned()
                } else {
                    todo!("Log warning about failed file open maybe")
                }
            })
            .collect();

        Ok(Peer {
            name: Some(hostname.to_string()),
            identities,
        })
    }

    /// Find a peer description by its comment in the specified authorized_keys files
    fn search_authorized_keys<P: AsRef<Path>>(in_authorized_keys: Vec<P>, hostname: &str) -> Result<Self> {
        todo!()
    }
}