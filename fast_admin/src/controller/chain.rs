trait Handler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a>;
    fn hande(&self, request: &str);
}

struct AHander<'a> {
    neme: String,
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> AHander<'a> {
    fn new(neme: String) -> Self {
        Self { neme, next: None }
    }
}

impl<'a> Handler<'a> for AHander<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }

    fn hande(&self, request: &str) {
        println!("{} handle the request: {}", self.neme, request);
        if let Some(v) = self.next {
            v.hande(request);
        }
    }
}

struct BHandler<'a> {
    next: Option<&'a dyn Handler<'a>>,
}

impl<'a> BHandler<'a> {
    fn new(next: Option<&'a dyn Handler<'a>>) -> Self {
        Self { next }
    }
}

impl<'a> Handler<'a> for BHandler<'a> {
    fn set_next(&mut self, next: &'a dyn Handler<'a>) -> &mut dyn Handler<'a> {
        self.next = Some(next);
        self
    }

    fn hande(&self, request: &str) {
        println!("Bhandler handle the request{}", request);
        if let Some(v) = self.next {
            v.hande(request)
        }
    }
}

struct Client;
impl<'a> Client {
    fn handle<T: Handler<'a>>(h: &T) {
        h.hande("do somethig");
    }
}

#[test]
pub fn test() {
    let a1 = AHander::new("dog".to_string());
    Client::handle(&a1);
    println!();
    let mut b = BHandler::new(None);
    let mut a2 = AHander::new("cat".to_string());
    a2.set_next(&b);
    Client::handle(&a2);
}
