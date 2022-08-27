use std::error::Error;
use cairo::{XCBVisualType, XCBConnection as XCBConnectionCario};
use prepare_window::find_xcb_visualtype;
use x11rb::xcb_ffi::XCBConnection;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Screen};
use crate::size::{Size, Sizeable};
use crate::prepare_window::{AtomCollection, self};
use crate::window::NoSurfaceRender;
use crate::xcb_visualtype_t::xcb_visualtype_t;

pub trait RootElement {  
  fn start(&mut self) {

  }

  fn event_loop(&mut self) {
  }
}

pub struct Root {
  conn: XCBConnection,
  atoms: AtomCollection,
  depth: u8,
  visual_id: u32,
  transparency: bool,
  screen_number: usize,
  visual: XCBVisualType,
  cairo_connection: XCBConnectionCario,
  children: Vec<Box<dyn NoSurfaceRender>>
}

pub struct  WindowToolKit<'a> {
  pub conn: &'a XCBConnection,
  pub atoms: &'a AtomCollection,
  pub depth: &'a u8,
  pub visual_id: &'a u32,
  pub transparency: &'a bool,
  pub screen_number: &'a usize,
  pub visual: &'a XCBVisualType,
  pub cairo_connection: &'a XCBConnectionCario,
}

impl<'a> WindowToolKit<'a> {
  pub fn from_root(root: &Root) -> WindowToolKit {
    WindowToolKit { 
      conn: &root.conn, 
      atoms: &root.atoms, 
      depth: &root.depth, 
      visual_id: &root.visual_id, 
      transparency: &root.transparency,
      screen_number: &root.screen_number,
      visual: &root.visual,
      cairo_connection: &root.cairo_connection 
    }
  }
}

impl Root {
  pub fn new(children: fn(WindowToolKit) -> Vec<Box<dyn NoSurfaceRender>>) -> Result<Root, String> {

    let (conn, screen_num) = match XCBConnection::connect(None) {
      Ok(data) => data,
      Err(e) => { return Err(format!("Cannot connect to the X11 server. Error: {}", e)) }
    };

    let atoms = match prepare_window::create_atoms(&conn) {
      Ok(data) => data,
      Err(e) => return Err(format!("Cannot create the window atom collection. Error: {}", e))
    };
  
    let (depth, visual_id) = match prepare_window::choose_visual(&conn, screen_num) {
      Ok(data) => data,
      Err(e) => return Err(format!("Cannot find the screen's visual. Error: {}", e))
    };
    
    let transparency = match prepare_window::composite_manager_running(&conn, screen_num) {
      Ok(data) => data,
      Err(e) => return Err(format!("There was an error while looking for compositor. Error:: {}", e))
    };

    let mut visual = match find_xcb_visualtype(&conn, visual_id) {
      Some(data) => data,
      None => return Err("Cannot find xcb visual.".to_string())
    };
  
    let cairo_conn: XCBConnectionCario = unsafe { cairo::XCBConnection::from_raw_none(conn.get_raw_xcb_connection() as _) };
    let cairo_visual: XCBVisualType = unsafe { cairo::XCBVisualType::from_raw_none(&mut visual as *mut _ as _) };

    let mut root = Root {
      conn,
      atoms,
      depth,
      visual_id,
      transparency,
      screen_number: screen_num,
      visual: cairo_visual,
      cairo_connection: cairo_conn,
      children: vec![],
    };

    root.children = children(WindowToolKit::from_root(&root));
    Ok(root)
  }
}

impl RootElement for Root {
  fn start(&mut self) {
    for window in &self.children {
      
    }
  }

  fn event_loop(&mut self) {

  }
}