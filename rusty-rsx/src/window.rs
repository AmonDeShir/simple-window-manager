use cairo::{XCBSurface};
use x11rb::{connection::Connection, xcb_ffi::XCBConnection, protocol::xproto::ConnectionExt};
use crate::{size::{Size, Sizeable}, root::{WindowToolKit}, prepare_window, element::Element};

pub trait NoSurfaceRender {
  fn set_visibility(&mut self, showed: bool) {}
  fn show(&mut self) {}
  fn no_surface_render(&mut self);
}

pub struct Window<'a> {
  size: Size::<u16>,
  name: String,
  window: u32,
  surface: XCBSurface,
  connection: &'a XCBConnection,
  children: Vec<Box<dyn Element>>,
  hidden: bool,
}

impl<'a> Window<'a> {
  pub fn new(name: &str, size: (u16, u16), root: &WindowToolKit, children: Vec<Box<dyn Element>>) -> Result<Window, String> {
    let screen = &root.conn.setup().roots[root.screen_number.to_owned()];

    let window = prepare_window::create_window(
      root.conn,
      screen,
      root.atoms,
      root.depth.to_owned(), 
      root.visual_id.to_owned(), 
      Size::from_tuple(size)
    );
    
    let window = match window {
      Ok(window) => window,
      Err(e) => return Err(format!("Cannot create Window. Error: {}", e)),
    };

    let surface = cairo::XCBSurface::create(
      root.cairo_connection,
      &cairo::XCBDrawable(window),
      root.visual,
      size.0.into(),
      size.1.into(),
    );

    let surface = match surface {
      Ok(surface) => surface,
      Err(e) => return Err(format!("Cannot create surface. Error: {}", e)),
    };

    Ok(Window {
      name: name.to_string(),
      size: Size::from_tuple(size),
      connection: root.conn,
      window,
      surface,
      children,
      hidden: false,
    })
  }
}

impl<'a> NoSurfaceRender for Window<'a> {
  fn set_visibility(&mut self, showed: bool) {
    if showed == !self.hidden {
      return;
    }

    if showed {
      self.connection.map_window(self.window);
    }
    else {
      self.connection.unmap_window(self.window);
    } 
    
    self.hidden = !showed;
  }

  fn no_surface_render(&mut self) {
    if self.hidden {
      return;
    }

    for child in &self.children {
      child.render();
    }
  }
}