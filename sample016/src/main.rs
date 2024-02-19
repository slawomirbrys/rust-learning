fn main() {
    let worker = Worker { name: "Happy worker".to_string() };
    WorkSlow::do_work(&worker);
    WorkFast::do_work(&worker);

    do_slow_worker(&worker);
    do_fast_worker(&worker);
}


trait WorkSlow {
    fn name_get(&self) -> &str;
    fn do_work(&self) {
        println!("I'm {} and working slowly", self.name_get());
    }
}

trait WorkFast {
    fn do_work(&self);
}

struct Worker {
    name: String
}

impl WorkSlow for Worker {
    fn name_get(&self) -> &str {
        self.name.as_str()
    }
}

impl WorkFast for Worker {
    fn do_work(&self) {
        println!("I'm {} and working fast", self.name);
    }
}


fn do_slow_worker<W: WorkSlow>(worker: &W) {
    worker.do_work();
}

fn do_fast_worker<W: WorkFast>(worker: &W) {
    worker.do_work();
}
