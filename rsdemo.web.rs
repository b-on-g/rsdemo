use std::cell::RefCell;
use crate::mol::wire::{text, Auto, Root};

#[link(wasm_import_module = "bog_rsdemo")]
extern "C" {
	fn render(ptr: *const u8, len: usize);
}

thread_local! {
	static RUNNING: RefCell<Option<(Auto, Root<App>)>> = const { RefCell::new(None) };
}

fn app() -> App {
	RUNNING.with(|running| **running.borrow().as_ref().expect("call bog_rsdemo_start first").1)
}

fn quote(text: &str) -> String {
	let mut out = String::from("\"");
	for char in text.chars() {
		match char {
			'"' => out.push_str("\\\""),
			'\\' => out.push_str("\\\\"),
			char if (char as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", char as u32)),
			char => out.push(char),
		}
	}
	out.push('"');
	out
}

fn list(app: App, ids: Vec<u32>) -> String {
	let items: Vec<String> = ids
		.into_iter()
		.map(|id| {
			let task = app.task(id);
			format!("{{\"id\":{},\"title\":{},\"done\":{}}}", id, quote(&task.title(None)), task.done(None))
		})
		.collect();
	format!("[{}]", items.join(","))
}

fn state(app: App) -> String {
	format!(
		"{{\"board\":{},\"shelf\":{},\"summary\":{}}}",
		list(app, app.tasks(None)),
		list(app, app.archive(None)),
		quote(&app.summary()),
	)
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_start() {
	let root = Root::<App>::default();
	let app = **root;
	let view = Auto::new(move || {
		let json = state(app);
		unsafe { render(json.as_ptr(), json.len()) }
	});
	RUNNING.with(|running| *running.borrow_mut() = Some((view, root)));
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_add(ptr: *mut u8, len: usize) -> u32 {
	app().add(&text(ptr, len))
}

#[no_mangle]
pub extern "C" fn bog_rsdemo_done(id: u32, done: u32) {
	app().task(id).done(Some(done != 0));
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
