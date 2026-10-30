use crate::mol::wire::{Mem, MemKey, Obj};
use crate::bog::rsdemo::store::Store;
use crate::bog::rsdemo::task::Task;

#[derive(Clone, Copy, Default)]
pub struct App {
	store: Store,
	tasks: Mem<Vec<u32>>,
	archive: Mem<Vec<u32>>,
	task: MemKey<u32, Obj<Task>>,
	pending: Mem<usize>,
	summary: Mem<String>,
	board: Mem<Vec<String>>,
	shelf: Mem<Vec<String>>,
}

impl App {
	pub fn tasks(self, next: Option<Vec<u32>>) -> Vec<u32> {
		self.tasks.call(next, |next| next.unwrap_or_default())
	}

	pub fn archive(self, next: Option<Vec<u32>>) -> Vec<u32> {
		self.archive.call(next, |next| next.unwrap_or_default())
	}

	pub fn task(self, id: u32) -> Obj<Task> {
		let store = self.store;
		self.task.call(&id, None, move |&id, _| Obj::make(move || Task::new(store, id)))
	}

	pub fn add(self, title: &str) -> u32 {
		let id = self.store.insert(title);
		self.tasks(Some([self.tasks(None), vec![id]].concat()));
		id
	}

	pub fn shelve(self, id: u32) {
		self.tasks(Some(self.tasks(None).into_iter().filter(|&i| i != id).collect()));
		self.archive(Some([self.archive(None), vec![id]].concat()));
	}

	pub fn remove(self, id: u32) {
		self.tasks(Some(self.tasks(None).into_iter().filter(|&i| i != id).collect()));
		self.archive(Some(self.archive(None).into_iter().filter(|&i| i != id).collect()));
		self.store.remove(id);
	}

	pub fn pending(self) -> usize {
		self.pending.call(None, move |_| self.tasks(None).into_iter().filter(|&id| !self.task(id).done(None)).count())
	}

	pub fn summary(self) -> String {
		self.summary.call(None, move |_| format!("{} of {} left, {} archived", self.pending(), self.tasks(None).len(), self.archive(None).len()))
	}

	pub fn board(self) -> Vec<String> {
		self.board.call(None, move |_| self.tasks(None).into_iter().map(|id| self.task(id).label()).collect())
	}

	pub fn shelf(self) -> Vec<String> {
		self.shelf.call(None, move |_| self.archive(None).into_iter().map(|id| self.task(id).label()).collect())
	}
}
