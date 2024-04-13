use iter_tools::Itertools;
use std::{collections::HashMap, fmt::Display};

pub struct Hosts {
    hosts: HashMap<String, String>,
}

impl Hosts {
    pub fn new() -> Self {
        Self {
            hosts: HashMap::new(),
        }
    }
    pub fn extend(&mut self, h: Hosts) {
        self.hosts.extend(h.hosts);
    }
}

impl Default for Hosts {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Hosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hosts_raw = String::new();

        for h in self.hosts.clone().into_iter().sorted() {
            hosts_raw.push_str(&format!("{} {}\n", h.1, h.0));
        }

        // Remove any trailing newline
        if hosts_raw.ends_with('\n') {
            hosts_raw.remove(hosts_raw.len() - 1);
        }

        write!(f, "{}", hosts_raw)?;

        Ok(())
    }
}

impl From<&str> for Hosts {
    fn from(s: &str) -> Self {
        let mut hosts_map = HashMap::new();

        for l in s.lines() {
            // Check if this line is a comment
            if l.starts_with('#') {
                // Skip this line
                continue;
            }

            if let Some((k, v)) = l.split_once(' ') {
                hosts_map.insert(v.to_string(), k.to_string());
            }
        }

        Hosts { hosts: hosts_map }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn to_string_test() {
        let mut hosts_map = HashMap::new();
        hosts_map.insert("localhost".to_string(), "127.0.0.1".to_string());
        hosts_map.insert("example.com".to_string(), "0.0.0.0".to_string());

        let hosts = Hosts {hosts: hosts_map};

        assert_eq!(hosts.to_string(), "0.0.0.0 example.com\n127.0.0.1 localhost");
    }

    #[test]
    fn to_string_conflict_test() {
        let mut hosts_map = HashMap::new();
        hosts_map.insert("localhost".to_string(), "127.0.0.1".to_string());
        hosts_map.insert("example.com".to_string(), "0.0.0.0".to_string());
        hosts_map.insert("example.com".to_string(), "0.0.0.1".to_string());

        let hosts = Hosts {hosts: hosts_map};

        assert_eq!(hosts.to_string(), "0.0.0.1 example.com\n127.0.0.1 localhost");
    }

    #[test]
    fn from_string_test() {
        let hosts_str = "127.0.0.1 localhost\n0.0.0.0 example.com";

        let hosts_map = Hosts::from(hosts_str);

        assert_eq!(
            hosts_map.hosts,
            HashMap::from([
                ("localhost".to_string(), "127.0.0.1".to_string()),
                ("example.com".to_string(), "0.0.0.0".to_string()),
            ])
        );
    }

    #[test]
    fn comment_removal_test() {
        let hosts_str = "127.0.0.1 localhost\n#0.0.0.0 example.com";

        let hosts_map = Hosts::from(hosts_str);

        assert_eq!(
            hosts_map.hosts,
            HashMap::from([("localhost".to_string(), "127.0.0.1".to_string())])
        )
    }
}
