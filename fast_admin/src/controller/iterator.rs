use std::clone;

trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
    fn current(&self) -> Option<T>;
    fn has_next(&self) -> bool;
    fn reset(&mut self);
}

struct Container<T> {
    data: Vec<T>,
}

impl<T> Container<T> where T:Clone {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn add_item(&mut self,item:T){
        self.data.push(item);
    }
    fn iter(&self) -> impl Iterator<T> +'_ {
        ConcreteIterator::new(self)
    }

}

struct ConcreteIterator<'a, T> {
    idx: usize,
    container: &'a Container<T>,
}

impl<'a, T: Clone> ConcreteIterator<'a, T> {
    fn new(container: &'a Container<T>) -> Self {
        Self { idx: 0, container }
    }
}

impl<'a, T> Iterator<T> for ConcreteIterator<'a, T>
where
    T: Clone,
{
    fn next(&mut self) -> Option<T> {
        let item = self.container.data.get(self.idx).cloned();
        self.idx += 1;
        item
    }

    fn current(&self) -> Option<T> {
        let item = self.container.data.get(self.idx).cloned();
        return item;
    }

    fn has_next(&self) -> bool {
        self.container.data.len() > self.idx
    }

    fn reset(&mut self) {
        self.idx = 0;
    }
}

#[test]
pub fn test(){
    let mut c = Container::new();
    c.add_item(1);
    c.add_item(2);
    c.add_item(3);

    let mut iter = c.iter();
    let has_next = iter.has_next();
    assert_eq!(has_next,true);
    let item = iter.next();
    println!("item:{:?}",item);
    iter.reset();

    while iter.has_next() {
        let v = iter.next().unwrap();
        println!("item:{}",v);
    }
    let item = iter.next();
    assert_eq!(item,None);
}