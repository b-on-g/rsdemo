use crate::mol::wire::{alive, log, sync, Auto, Root};

pub fn main() {
	let root = Root::<App>::default();
	let app = **root;

	let board = Auto::new(move || log(&format!("board: {:?}", app.board())));
	let shelf = Auto::new(move || log(&format!("shelf: {:?}", app.shelf())));
	let summary = Auto::new(move || log(&format!("summary: {}", app.summary())));

	let milk = app.add("Milk");
	app.add("Bread");
	sync();

	app.task(milk).done(Some(true));
	sync();

	let handle = app.task(milk);
	app.shelve(milk);
	sync();
	log(&format!("shelved milk is same object: {}", handle == app.task(milk) && handle.alive()));

	app.remove(milk);
	sync();
	log(&format!("removed milk alive: {}", handle.alive()));

	drop((board, shelf, summary));
	drop(root);
	sync();
	log(&format!("nodes alive: {}", alive()));
}
