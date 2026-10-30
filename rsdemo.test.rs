use crate::mol::wire::{alive, sync, Auto, Root};

fn views(app: App) -> Vec<Auto> {
	vec![
		Auto::new(move || {
			app.board();
		}),
		Auto::new(move || {
			app.shelf();
		}),
	]
}

#[test]
fn pending_follows_tasks() {
	let app = Root::<App>::default();
	let milk = app.add("Milk");
	app.add("Bread");
	assert_eq!(app.summary(), "2 of 2 left, 0 archived");
	app.task(milk).done(Some(true));
	assert_eq!(app.summary(), "1 of 2 left, 0 archived");
}

#[test]
fn rename_keeps_summary() {
	let app = Root::<App>::default();
	let milk = app.add("Milk");
	let before = app.summary();
	app.task(milk).title(Some("Oat milk".into()));
	assert_eq!(app.summary(), before);
	assert_eq!(app.task(milk).title(None), "Oat milk");
}

#[test]
fn shelved_task_is_same_object() {
	let app = Root::<App>::default();
	let _views = views(**app);
	let milk = app.add("Milk");
	app.add("Bread");
	sync();
	let task = app.task(milk);
	app.shelve(milk);
	sync();
	assert!(task.alive());
	assert!(task == app.task(milk));
	assert_eq!(app.shelf(), vec!["[ ] Milk"]);
	assert_eq!(app.board(), vec!["[ ] Bread"]);
}

#[test]
fn unobserved_task_recreated_from_store() {
	let app = Root::<App>::default();
	let milk = app.add("Milk");
	let task = app.task(milk);
	task.done(Some(true));
	sync();
	assert!(!task.alive());
	assert!(app.task(milk).done(None));
}

#[test]
fn removed_task_is_freed() {
	let app = Root::<App>::default();
	let _views = views(**app);
	sync();
	let base = alive();
	let milk = app.add("Milk");
	app.shelve(milk);
	sync();
	let task = app.task(milk);
	app.remove(milk);
	sync();
	assert!(!task.alive());
	assert!(!app.store.records(None).contains_key(&milk));
	assert_eq!(alive(), base);
}

#[test]
fn app_drop_frees_all() {
	{
		let app = Root::<App>::default();
		let _views = views(**app);
		for i in 0..1000 {
			app.add(&format!("task {}", i));
		}
		sync();
		app.summary();
	}
	sync();
	assert_eq!(alive(), 0);
}
