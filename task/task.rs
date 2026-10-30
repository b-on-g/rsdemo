use crate::mol::wire::Mem;
use crate::bog::rsdemo::store::Store;

#[derive(Clone, Copy)]
pub struct Task {
	store: Store,
	id: u32,
	label: Mem<String>,
}

impl Task {
	pub fn new(store: Store, id: u32) -> Self {
		Self { store, id, label: Mem::default() }
	}

	pub fn id(self) -> u32 {
		self.id
	}

	pub fn title(self, next: Option<String>) -> String {
		if let Some(title) = next {
			self.store.patch(self.id, |record| record.title = title);
		}
		self.store.record(self.id).title
	}

	pub fn done(self, next: Option<bool>) -> bool {
		if let Some(done) = next {
			self.store.patch(self.id, |record| record.done = done);
		}
		self.store.record(self.id).done
	}

	pub fn label(self) -> String {
		self.label.call(None, move |_| format!("[{}] {}", if self.done(None) { "x" } else { " " }, self.title(None)))
	}
}
