use std::collections::HashMap;
use std::rc::Rc;
use crate::mol::wire::{solid, Mem, Value};

#[derive(Clone, PartialEq, Default)]
pub struct Record {
	pub title: String,
	pub done: bool,
}

impl Value for Record {}

#[derive(Clone, Copy, Default)]
pub struct Store {
	records: Mem<Rc<HashMap<u32, Record>>>,
	seq: Mem<u32>,
}

impl Store {
	pub fn records(self, next: Option<Rc<HashMap<u32, Record>>>) -> Rc<HashMap<u32, Record>> {
		self.records.call(next, |next| {
			solid();
			next.unwrap_or_default()
		})
	}

	pub fn seq(self, next: Option<u32>) -> u32 {
		self.seq.call(next, |next| {
			solid();
			next.unwrap_or(0)
		})
	}

	pub fn record(self, id: u32) -> Record {
		self.records(None).get(&id).cloned().unwrap_or_default()
	}

	pub fn patch(self, id: u32, edit: impl FnOnce(&mut Record)) {
		let mut records = (*self.records(None)).clone();
		edit(records.entry(id).or_default());
		self.records(Some(Rc::new(records)));
	}

	pub fn insert(self, title: &str) -> u32 {
		let id = self.seq(None) + 1;
		self.seq(Some(id));
		self.patch(id, |record| record.title = title.into());
		id
	}

	pub fn remove(self, id: u32) {
		let mut records = (*self.records(None)).clone();
		records.remove(&id);
		self.records(Some(Rc::new(records)));
	}
}
