#[derive(Debug, Clone)]
struct Work {
    id: i32,
}

struct WorkMaster {
    workers: Vec<Box<dyn Worker>>,
}
impl WorkMaster {
    fn register_worker(&mut self, worker: Box<dyn Worker>) {
        self.workers.push(worker)
    }
    fn dispatch_new_work(&self, work: Work) {
        for worker in &self.workers {
            worker.on_new_work(work.clone());
        }
    }
}

trait Worker {
    fn on_new_work(&self, work: Work);
}

struct LocalWorker {}

impl Worker for LocalWorker {
    fn on_new_work(&self, work: Work) {
        println!("local worker receiver new work{:?}", work);
    }
}

struct RemoteWorker {}
impl Worker for RemoteWorker {
    fn on_new_work(&self, work: Work) {
        println!("remote worker receive new work {:?}", work);
    }
}

#[cfg(test)]
mod tests {
    use super::{LocalWorker, RemoteWorker, Work, WorkMaster};

    #[test]
    fn observer_mode_test() {
        let mut master = WorkMaster {
            workers: Vec::new(),
        };
        let local_worker = LocalWorker {};
        let remote_worker = RemoteWorker {};

        master.register_worker(Box::new(local_worker));
        master.dispatch_new_work(Work { id: 1 });
    }
}

trait Beverage {
    fn get_desc(&self) -> String;
    fn cost(&self) -> f64;
}

struct Tee {}
impl Beverage for Tee {
    fn get_desc(&self) -> String {
        return String::from("茶");
    }

    fn cost(&self) -> f64 {
        2.2
    }
}

struct Sugar {
    beverage: Box<dyn Beverage>,
}

impl Sugar {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}

impl Beverage for Sugar {
    fn get_desc(&self) -> String {
        self.beverage.get_desc() + "糖"
    }

    fn cost(&self) -> f64 {
        self.beverage.cost() + 0.8
    }
}

struct Milk {
    beverage: Box<dyn Beverage>,
}

impl Milk {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Self { beverage }
    }
}
impl Beverage for Milk {
    fn get_desc(&self) -> String {
        self.beverage.get_desc() + "牛奶"
    }

    fn cost(&self) -> f64 {
        self.beverage.cost() + 1.1
    }
}
