use crate::mol::wire::{after, alive, log, Auto, Root};

pub fn main() {
	let root = Root::<App>::default();
	let app = **root;

	let views = (
		Auto::new(move || log(&format!("board: {:?}", app.board()))),
		Auto::new(move || log(&format!("shelf: {:?}", app.shelf()))),
		Auto::new(move || log(&format!("summary: {}", app.summary()))),
	);

	let milk = app.add("Milk");
	app.add("Bread");
	let handle = app.task(milk);

	after(move || {
		app.task(milk).done(Some(true));
	});

	after(move || app.shelve(milk));

	after(move || {
		log(&format!("shelved milk is same object: {}", handle == app.task(milk) && handle.alive()));
		app.remove(milk);
	});

	after(move || {
		log(&format!("removed milk alive: {}", handle.alive()));
		drop((views, root));
	});

	after(|| log(&format!("nodes alive: {}", alive())));
}
