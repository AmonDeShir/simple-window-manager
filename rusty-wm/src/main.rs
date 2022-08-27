use rusty_rsx::{root::Root, window::Window};

fn main() {
  let root = Root::new(|root| {
    let children = vec![];

    children.push(Window::new("Window", (300, 400), root));

    children
  });
}
