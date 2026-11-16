// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-puppet: puppet component model.
//!
//! Real implementation: puppet_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PuppetItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuppetError {
    EmptyPuppetItemName,
    DuplicatePuppetItem(String),
    EmptyEventSubject,
}

impl fmt::Display for PuppetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPuppetItemName => write!(f, "puppet_item name must not be empty"),
            Self::DuplicatePuppetItem(id) => write!(f, "duplicate puppet_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for PuppetError {}

pub struct Puppet {
    pub name: String,
    puppet_items: BTreeMap<String, PuppetItem>,
    events: BTreeMap<String, Event>,
}

impl Puppet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            puppet_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_puppet_item(&mut self, item: PuppetItem) -> Result<(), PuppetError> {
        if item.name.trim().is_empty() {
            return Err(PuppetError::EmptyPuppetItemName);
        }
        let id = item.id.clone();
        if self.puppet_items.contains_key(&id) {
            return Err(PuppetError::DuplicatePuppetItem(id.clone()));
        }
        self.puppet_items.insert(id, item);
        Ok(())
    }

    pub fn puppet_item_count(&self) -> usize {
        self.puppet_items.len()
    }

    pub fn get_puppet_item(&self, id: &str) -> Option<&PuppetItem> {
        self.puppet_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), PuppetError> {
        if event.subject.trim().is_empty() {
            return Err(PuppetError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_puppet_item(id: &str, name: &str) -> PuppetItem {
        PuppetItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Puppet::new("Puppet");
        assert_eq!(m.name, "Puppet");
        assert_eq!(m.puppet_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_puppet_item() {
        let mut m = Puppet::new("Puppet");
        m.add_puppet_item(sample_puppet_item("a1", "Alpha")).unwrap();
        assert_eq!(m.puppet_item_count(), 1);
        assert!(m.get_puppet_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Puppet::new("Puppet");
        assert!(m.add_puppet_item(sample_puppet_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Puppet::new("Puppet");
        m.add_puppet_item(sample_puppet_item("x", "One")).unwrap();
        assert!(m.add_puppet_item(sample_puppet_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Puppet::new("Puppet");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
