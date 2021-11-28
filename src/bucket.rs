use crate::routing_table::RoutingTableEntry;

pub const BUCKET_SIZE: usize = 8;

#[allow(dead_code)]
pub struct Bucket {
    pub min: u128,
    pub max: u128,
    pub nodes: Vec<RoutingTableEntry>,
}

impl Bucket {
    pub fn new(min: u128, max: u128) -> Bucket {
        Bucket {
            nodes: Vec::with_capacity(BUCKET_SIZE),
            min,
            max,
        }
    }

    pub fn insert(&mut self, entry: RoutingTableEntry) {
        self.nodes.push(entry);
    }

    pub fn is_full(&self) -> bool {
        self.nodes.len() >= BUCKET_SIZE
    }

    pub fn split(&mut self) -> Bucket {
        let old_max = self.max;
        let new_range = (self.max - self.min) / 2;

        let hi_nodes = self
            .nodes
            .drain_filter(|entry| entry.node_id > self.min + new_range)
            .collect();

        self.max = self.min + new_range;

        Bucket {
            min: self.min + new_range,
            max: old_max,
            nodes: hi_nodes,
        }
    }

    pub fn node_in_range(&self, node_id: u128) -> bool {
        self.min <= node_id && node_id <= self.max
    }

    pub fn find(&self, node_id: u128) -> Option<RoutingTableEntry> {
        self.nodes
            .iter()
            .cloned()
            .find(|entry| entry.node_id == node_id)
    }
}
