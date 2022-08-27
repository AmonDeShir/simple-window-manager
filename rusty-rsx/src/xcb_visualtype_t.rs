use x11rb::protocol::xproto::Visualtype;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct xcb_visualtype_t {
  pub visual_id: u32,
  pub class: u8,
  pub bits_per_rgb_value: u8,
  pub colormap_entries: u16,
  pub red_mask: u32,
  pub green_mask: u32,
  pub blue_mask: u32,
  pub pad0: [u8; 4],
}

impl From<Visualtype> for xcb_visualtype_t {
  fn from(value: Visualtype) -> xcb_visualtype_t {
    xcb_visualtype_t {
      visual_id: value.visual_id,
      class: value.class.into(),
      bits_per_rgb_value: value.bits_per_rgb_value,
      colormap_entries: value.colormap_entries,
      red_mask: value.red_mask,
      green_mask: value.green_mask,
      blue_mask: value.blue_mask,
      pad0: [0; 4],
    }
  }
}