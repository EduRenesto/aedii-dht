use std::collections::LinkedList;

use actix::Addr;

use crate::{bucket::Bucket, node::Node};

#[derive(PartialEq, Eq, Clone)]
pub struct RoutingTableEntry {
    pub node_id: u128,
    pub address: Addr<Node>,
}

//pub struct RoutingTable(LinkedList<Bucket>);
pub struct RoutingTable {
    parent_node_id: u128,
    buckets: LinkedList<Bucket>,
}

// The memory management here is HORRIBLE.
// Sorry for all those `.clone()`s.
//
// The thing is that I don't currently have time to think this throughly :p

impl RoutingTable {
    pub fn new(node_id: u128) -> RoutingTable {
	let mut buckets = LinkedList::new();
	buckets.push_back(Bucket::new(0, u128::MAX));

	RoutingTable {
	    parent_node_id: node_id,
	    buckets,
	}
    }

    pub fn insert(&mut self, entry: RoutingTableEntry) {
	let mut cursor = self.buckets.cursor_front_mut();

	while let Some(bucket) = cursor.current() {
	    if bucket.node_in_range(entry.node_id) {
		break;
	    }

	    cursor.move_next();
	}

	let mut bucket = cursor.current().unwrap();

	if bucket.is_full() && bucket.node_in_range(self.parent_node_id) {
	    let high_bucket = bucket.split();

	    if high_bucket.node_in_range(entry.node_id) {
		cursor.insert_after(high_bucket);
		cursor.move_next();
		bucket = cursor.current().unwrap();
	    }

	    bucket.insert(entry);
	} else if !bucket.is_full() {
	    bucket.insert(entry);
	}
    }

    pub fn find_exact(&self, node_id: u128) -> Option<RoutingTableEntry> {
	let mut cursor = self.buckets.cursor_front();

	while let Some(bucket) = cursor.current() {
	    if bucket.node_in_range(node_id) {
		break;
	    }

	    cursor.move_next();
	}

	let bucket = cursor.current().unwrap();

	bucket.find(node_id)
    }

    pub fn find_closest(&self, node_id: u128) -> Vec<RoutingTableEntry> {
	let mut cursor = self.buckets.cursor_front();

	while let Some(bucket) = cursor.current() {
	    if bucket.node_in_range(node_id) {
		break;
	    }

	    cursor.move_next();
	}

	let bucket = cursor.current().unwrap();

	bucket.nodes.clone()
    }
}
