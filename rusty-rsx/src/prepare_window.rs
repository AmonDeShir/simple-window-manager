use x11rb::atom_manager;
use x11rb::connection::{Connection, RequestConnection};
use x11rb::protocol::xproto::{Screen, Window, ConnectionExt, WindowClass, Visualid, ColormapAlloc, CreateWindowAux, EventMask, AtomEnum, PropMode};
use x11rb::protocol::render::{self, ConnectionExt as _, PictType};
use x11rb::wrapper::{ConnectionExt as _ };
use x11rb::xcb_ffi::{XCBConnection, ReplyOrIdError, ReplyError};
use crate::size::Size;
use crate::xcb_visualtype_t::xcb_visualtype_t;

atom_manager! {
  pub AtomCollection: AtomCollectionCookie {
      WM_PROTOCOLS,
      WM_DELETE_WINDOW,
      _NET_WM_NAME,
      UTF8_STRING,
  }
}

pub fn create_atoms(conn: &XCBConnection) -> Result<AtomCollection, ReplyOrIdError> {
  return Ok(AtomCollection::new(conn)?.reply()?);
}

pub fn create_window(conn: &XCBConnection, screen: &Screen, atoms: &AtomCollection, depth: u8, visual_id: Visualid, size: Size<u16>) -> Result<Window, ReplyOrIdError> {
  let window = conn.generate_id()?;
  let color_map = conn.generate_id()?;

  conn.create_colormap(
    ColormapAlloc::NONE, 
    color_map, 
    screen.root,
     visual_id)?;
  
  conn.create_window(
    depth, 
    window, 
    screen.root, 
    0, 0,
    size.width, size.height,
    10, 
    WindowClass::INPUT_OUTPUT, 
    visual_id,
    &CreateWindowAux::new()
      .background_pixel(x11rb::NONE)
      .border_pixel(screen.black_pixel)
      .colormap(color_map)
      .event_mask(
        EventMask::EXPOSURE |
        EventMask::STRUCTURE_NOTIFY
      )
  )?;

  let title = "Simple Window";

  conn.change_property8(
      PropMode::REPLACE,
      window,
      AtomEnum::WM_NAME,
      AtomEnum::STRING,
      title.as_bytes(),
  )?;

  conn.change_property8(
      PropMode::REPLACE,
      window,
      atoms._NET_WM_NAME,
      atoms.UTF8_STRING,
      title.as_bytes(),
  )?;

  conn.change_property32(
      PropMode::REPLACE,
      window,
      atoms.WM_PROTOCOLS,
      AtomEnum::ATOM,
      &[atoms.WM_DELETE_WINDOW],
  )?;

  conn.change_property8(
      PropMode::REPLACE,
      window,
      AtomEnum::WM_CLASS,
      AtomEnum::STRING,
      b"simple_window\0simple_window\0",
  )?;

  conn.map_window(window)?;
  Ok(window)
}


/// Choose a visual to use. This function tries to find a depth=32 visual and falls back to the
/// screen's default visual.
pub fn choose_visual(conn: &XCBConnection, screen_num: usize) -> Result<(u8, Visualid), ReplyError> {
  let depth = 32;
  let screen = &conn.setup().roots[screen_num];

  // Try to use XRender to find a visual with alpha support
  let has_render = conn.extension_information(render::X11_EXTENSION_NAME)?.is_some();

  if has_render {
    let formats = conn.render_query_pict_formats()?.reply()?;
      
    // Find the ARGB32 format that must be supported.
    let format = formats
      .formats
      .iter()
      .filter(|info| (info.type_, info.depth) == (PictType::DIRECT, depth))
      .filter(|info| {
        let d = info.direct;
        (d.red_mask, d.green_mask, d.blue_mask, d.alpha_mask) == (0xff, 0xff, 0xff, 0xff)
      })
      .find(|info| {
        let d = info.direct;
        (d.red_shift, d.green_shift, d.blue_shift, d.alpha_shift) == (16, 8, 0, 24)
      });

    if let Some(format) = format {
      // Now we need to find the visual that corresponds to this format
      if let Some(visual) = formats.screens[screen_num]
        .depths
        .iter()
        .flat_map(|d| &d.visuals)
        .find(|v| v.format == format.id)
      {
          return Ok((format.depth, visual.visual));
      }
    }
  }

  Ok((screen.root_depth, screen.root_visual))
}

/// Find a `xcb_visualtype_t` based on its ID number
pub fn find_xcb_visualtype(conn: &XCBConnection, visual_id: u32) -> Option<xcb_visualtype_t> {
  for root in &conn.setup().roots {
    for depth in &root.allowed_depths {
      for visual in &depth.visuals {
        if visual.visual_id == visual_id {
            return Some((*visual).into());
        }
      }
    }
  }
  None
}

pub fn composite_manager_running(conn: &XCBConnection, screen_num: usize ) -> Result<bool, ReplyError> {
  let atom = format!("_NET_WM_CM_S{}", screen_num);
  let atom = conn.intern_atom(false, atom.as_bytes())?.reply()?.atom;
  let owner = conn.get_selection_owner(atom)?.reply()?;
  Ok(owner.owner != x11rb::NONE)
}