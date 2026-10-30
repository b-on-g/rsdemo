use std::cell::RefCell;
use crate::mol::wire::{log, Auto, Root};

thread_local! {
	static RUNNING: RefCell<Option<(Vec<Auto>, Root<App>)>> = const { RefCell::new(None) };
}

fn app() -> App {
	RUNNING.with(|running| **running.borrow().as_ref().expect("call bog_rsdemo_start first").1)
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_start() {
	let root = Root::<App>::default();
	let app = **root;
	let autos = vec![
		Auto::new(move || log(&format!("board: {:?}", app.board()))),
		Auto::new(move || log(&format!("shelf: {:?}", app.shelf()))),
		Auto::new(move || log(&format!("summary: {}", app.summary()))),
	];
	RUNNING.with(|running| *running.borrow_mut() = Some((autos, root)));
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_add() -> u32 {
	let app = app();
	app.add(&format!("task {}", app.tasks(None).len() + 1))
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_done(id: u32) {
	app().task(id).done(Some(true));
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_shelve(id: u32) {
	app().shelve(id);
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_remove(id: u32) {
	app().remove(id);
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_stop() {
	let running = RUNNING.with(|running| running.borrow_mut().take());
	drop(running);
}
