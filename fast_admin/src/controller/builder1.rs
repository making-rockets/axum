#[derive(Clone, Debug)]
struct Product {
    parts: Vec<String>,
}

impl Product {
    fn new() -> Self {
        Self { parts: Vec::new() }
    }

    fn list_parts(&self) {
        let parts_list = String::from(" parts ");
        println!("{0}{1}{0}", "*".repeat(10), parts_list);

        for v in &self.parts {
            println!("{}", v);
        }
        println!("{0}{1}{0}", "*".repeat(10), "*".repeat(parts_list.len()));
    }
}

trait Builder {
    fn produce_part_a(&mut self);
    fn produce_part_b(&mut self);
    fn produce_part_c(&mut self);
    fn get_produce(&mut self) -> Product;
}

struct ContreteBuilder1 {
    prodcut: Product,
}

impl ContreteBuilder1 {
    fn new() -> Self {
        Self {
            prodcut: Product::new(),
        }
    }
}

impl Builder for ContreteBuilder1 {
    fn produce_part_a(&mut self) {
        self.prodcut.parts.push("part a1".to_string());
    }

    fn produce_part_b(&mut self) {
        self.prodcut.parts.push("part b1".to_string());
    }

    fn produce_part_c(&mut self) {
        self.prodcut.parts.push("part c1".to_string());
    }

    fn get_produce(&mut self) -> Product {
        let p = self.prodcut.clone();
        self.prodcut = Product::new();
        p
    }
}

struct ContreteBuilder2 {
    product: Product,
}

impl ContreteBuilder2 {
    fn new() -> Self {
        Self {
            product: Product::new(),
        }
    }

    fn produce_part_a(&mut self) {
        self.product.parts.push("part a ~~~~ 2".to_string());
    }
    fn produce_part_b(&mut self) {
        self.product.parts.push("part b ~~~~ 2".to_string());
    }
    fn produce_part_c(&mut self) {
        self.product.parts.push("part c ~~~~ 2".to_string());
    }
    fn get_product(&mut self) -> Product {
        let p = Product {
            parts: self.product.parts.clone(),
            ..self.product
        };
        self.product = Product::new();
        p
    }
}

struct Director {
    builder: Box<dyn Builder>,
}

impl Director {
    fn new(builder: Box<dyn Builder>) -> Self {
        Director { builder }
    }

    fn construct(&mut self) {
        self.builder.produce_part_a();
        self.builder.produce_part_b();
        self.builder.produce_part_c();
    }
}

#[test]
fn test() {
    let builder1 = Box::new(ContreteBuilder1::new());
    let mut direct = Director::new(builder1);
    direct.construct();
    let product = direct.builder.get_produce();
    product.list_parts();
}

