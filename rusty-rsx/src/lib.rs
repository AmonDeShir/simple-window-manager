
mod prepare_window;
mod xcb_visualtype_t;
pub mod root;
pub mod element;
pub mod size;
pub mod show;
pub mod window;


pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub fn crate_window() {

}


// pub fn main() {
//   let (show, setShow) = createSignal(false);
//
//   let app = rsx!(
//     Root {
//       Window {
//         Div {
//          "click me",
//          click: |v| { setShow(!v) }
//         },
//       },
//
//       Show {
//         when: show(),
//         Window {
//           Div {
//             "text",
//           }
//         }
//       }
//     }
//   );
//
//   app.start();
// }