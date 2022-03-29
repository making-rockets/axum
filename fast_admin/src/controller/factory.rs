trait Button {
    fn paint(&self);
}

trait Checkbox {
    fn paint(&self);
}

trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

struct WinFactory;
struct MacFactory;

impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox)
    }
}

struct WinButton;
struct MacButton;

impl Button for WinButton {
    fn paint(&self) {
        println!("windows on button");
    }
}

impl Button for MacButton {
    fn paint(&self) {
        println!("mac on button");
    }
}

struct WinCheckbox;
struct MacCheckbox;

impl Checkbox for WinCheckbox {
    fn paint(&self) {
        println!("windows on checkbox");
    }
}
impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("mac on checkbox");
    }
}
