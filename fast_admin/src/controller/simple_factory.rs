pub enum PizzaName {
    CheesePizza,
    ClamPizza,
}

pub trait Pizza {
    fn prepare(&self);
    fn bake(&self);
    fn cut(&self);
    fn pack(&self);
}
pub struct CheesePizza {
    x: String,
}
impl CheesePizza {
    pub fn new(x: String) -> CheesePizza {
        CheesePizza { x }
    }
}
impl Pizza for CheesePizza {
    fn prepare(&self) {
        println!("cheese pizza prepare {}", self.x)
    }

    fn bake(&self) {
        println!("cheese pizza bake")
    }

    fn cut(&self) {
        println!("cheese pizza cut")
    }

    fn pack(&self) {
        println!("cheese pizza pack")
    }
}
pub struct ClamPizza {
    x: String,
}
impl ClamPizza {
    pub fn new(x: String) -> ClamPizza {
        ClamPizza { x }
    }
}
impl Pizza for ClamPizza {
    fn prepare(&self) {
        println!("clam pizza prepare {}", self.x)
    }

    fn bake(&self) {
        println!("clam pizza bake")
    }

    fn cut(&self) {
        println!("clam pizza cut")
    }

    fn pack(&self) {
        println!("clam pizza pack")
    }
}

pub struct SimplePizzaFactory {}
impl SimplePizzaFactory {
    fn create_pizza(&self, pizza_name: PizzaName) -> Box<dyn Pizza> {
        match pizza_name {
            PizzaName::CheesePizza => Box::new(CheesePizza { x: "1".to_string() }),
            PizzaName::ClamPizza => Box::new(ClamPizza { x: "2".to_string() }),
        }
    }
}

struct PizzaStore {
    factory: SimplePizzaFactory,
}

impl PizzaStore {
    fn order_pizza(&self, pizza_name: PizzaName) -> Box<dyn Pizza> {
        let pizza = self.factory.create_pizza(pizza_name);

        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.pack();

        pizza
    }
}
