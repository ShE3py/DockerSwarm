use serde::{Deserialize, Deserializer};
use std::ffi::OsStr;
use std::fmt::{self, Formatter};
use std::net::IpAddr;
use serde::de::{Error, Visitor};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    pub networks_attachments: Vec<NetworkAttachments>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkAttachments {
    pub network: Network,
    pub addresses: Vec<Address>,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub spec: NetworkSpec,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSpec {
    pub name: String,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Address(IpAddr);


/// Return low-level information on Docker objects.
pub fn inspect(id: impl AsRef<OsStr>) -> Option<Container> {
    let stdout = super::docker(["inspect", "--format", "json"], &id);
    
    match serde_json::from_slice::<Vec<Container>>(&stdout) {
        Ok(mut v) => v.pop(),
        Err(e) => {
            eprintln!("error: failed to parse the following as a task: {e}");
            eprintln!("{}", String::from_utf8_lossy(&stdout));
            None
        }
    }
}


impl Container {
    pub fn get_ip(&self, network: &str) -> Option<IpAddr> {
        self.networks_attachments.iter()
            .find(|net_at| net_at.network.spec.name == network)
            .and_then(|net_at| net_at.addresses.first().copied().map(|addr| addr.0))
    }
}

// Ignore le masque “/24”
impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct StrVisitor;
        impl Visitor<'_> for StrVisitor {
            type Value = Address;
            
            fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str("IP address")
            }
            
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let addr = v.split_once('/').map_or(v, |(addr, _mask)| addr);
                
                match addr.parse() {
                    Ok(v) => Ok(Address(v)),
                    Err(e) => Err(E::custom(e)),
                }
            }
        }
        deserializer.deserialize_str(StrVisitor)
    }
}
