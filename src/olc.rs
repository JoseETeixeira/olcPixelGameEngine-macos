//! olcPixelGameEngine Rust API
use crate::cpp;

use std::ffi::CString;
use std::fmt;

pub use cpp::PixelMode;
pub use cpp::Pixel;

//----------------------------------
// Private runnable API
//----------------------------------

/// Binding for the game.
struct Binding<'a> {
  app: &'a mut dyn Application
}

#[no_mangle]
extern "C" fn onUserCreate(binding: *mut cpp::c_void) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.app.on_user_create() {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in the main function
  res
}

#[no_mangle]
extern "C" fn onUserUpdate(binding: *mut cpp::c_void, elapsed_time: cpp::c_float) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.app.on_user_update(elapsed_time) {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in the main function
  res
}

#[no_mangle]
extern "C" fn onUserDestroy(binding: *mut cpp::c_void) -> bool {
  // binding goes out of scope and is dropped
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  match b.app.on_user_destroy() {
    Err(err) => println!("ERROR: {}", err),
    Ok(_) => {}
  }
  true // always return true to finish cleanup
}

//----------------------------------
// Public API
//----------------------------------

#[derive(Clone, Debug)]
pub struct Error {
  msg: String
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl From<std::ffi::NulError> for Error {
  fn from(error: std::ffi::NulError) -> Self {
    Self { msg: format!("{}", error) }
  }
}

/// Application trait, should be extended by an implementation and passed to start function.
pub trait Application {
  /// Called on user create action.
  fn on_user_create(&mut self) -> Result<(), Error>;
  /// Called on user update action for every frame.
  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), Error>;
  /// Called on user destroy action.
  fn on_user_destroy(&mut self) -> Result<(), Error>;
}

/// Starts the main game loop.
pub fn start(
  name: &str,
  app: &mut dyn Application,
  screen_width: i32,
  screen_height: i32,
  pixel_width: i32,
  pixel_height: i32
) -> Result<(), Error>
{
  start_with_full_screen_and_vsync(
    name, app, screen_width, screen_height, pixel_width, pixel_height, false, false)
}

/// Starts the main game loop with configurable full screen and vsync.
pub fn start_with_full_screen_and_vsync(
  name: &str,
  app: &mut dyn Application,
  screen_width: i32,
  screen_height: i32,
  pixel_width: i32,
  pixel_height: i32,
  full_screen: bool,
  vsync: bool
) -> Result<(), Error>
{
  let name = CString::new(name)?;

  let binding = Binding { app };

  let res = unsafe {
    cpp::start(
      name.as_ptr(),
      Box::into_raw(Box::new(binding)) as *mut cpp::c_void,
      screen_width,
      screen_height,
      pixel_width,
      pixel_height,
      full_screen,
      vsync
    )
  };

  match res {
    cpp::RCode::CONSTRUCT_FAIL =>
      Err(Error { msg: format!("Failed to construct the application: FAIL") }),
    cpp::RCode::CONSTRUCT_NO_FILE =>
      Err(Error { msg: format!("Failed to construct the application: NO_FILE") }),
    cpp::RCode::START_FAIL =>
      Err(Error { msg: format!("Failed to start the application: FAIL") }),
    cpp::RCode::START_NO_FILE =>
      Err(Error { msg: format!("Failed to start the application: NO_FILE") }),
    cpp::RCode::OK =>
      Ok(())
  }
}

/// Utility C++ rand function.
pub fn c_rand() -> i32 {
  unsafe { cpp::c_rand() }
}

//----------------------------------
// olcPixelGameEngine API
//----------------------------------

/// Returns the width of the screen in "pixels".
pub fn screen_width() -> i32 {
  unsafe { cpp::ScreenWidth() }
}

/// Returns the height of the screen in "pixels".
pub fn screen_height() -> i32 {
  unsafe { cpp::ScreenHeight() }
}

/// Returns the width of the currently selected drawing target in "pixels".
pub fn get_draw_target_width() -> i32 {
  unsafe { cpp::GetDrawTargetWidth() }
}

/// Returns the height of the currently selected drawing target in "pixels".
pub fn get_draw_target_height() -> i32 {
  unsafe { cpp::GetDrawTargetHeight() }
}

/// Resizes the primary screen sprite.
pub fn set_screen_size(w: i32, h: i32) {
  unsafe { cpp::SetScreenSize(w, h) }
}

/// Gets the current Frames Per Second.
pub fn get_fps() -> u32 {
  unsafe { cpp::GetFPS() }
}

/// Layer API.
/// Sets layer as the main draw target.
pub fn set_draw_target(layer: u8) {
  unsafe { cpp::SetDrawTarget(layer) }
}

/// Layer API.
/// Enables/disables layer.
pub fn enable_layer(layer: u8, b: bool) {
  unsafe { cpp::EnableLayer(layer, b) }
}

/// Layer API.
/// Sets layer offset.
pub fn set_layer_offset(layer: u8, x: f32, y: f32) {
  unsafe { cpp::SetLayerOffset(layer, x, y) }
}

/// Layer API.
/// Sets layer scale.
pub fn set_layer_scale(layer: u8, x: f32, y: f32) {
  unsafe { cpp::SetLayerScale(layer, x, y) }
}

/// Layer API.
/// Sets layer tint.
pub fn set_layer_tint(layer: u8, tint: Pixel) {
  unsafe { cpp::SetLayerTint(layer, tint) }
}

/// Layer API.
/// Creates a new layer.
pub fn create_layer() -> u8 {
  // Layer is supposed to be at most u8
  let layer = unsafe { cpp::CreateLayer() };
  layer as u8
}

/// Changes the pixel mode for different optimisations.
///
/// - PixelMode::NORMAL = No transparency.
/// - PixelMode::MASK   = Transparent if alpha is < 255.
/// - PixelMode::ALPHA   = Full transparency.
pub fn set_pixel_mode(m: PixelMode) {
  unsafe { cpp::SetPixelMode(m) }
}

/// Returns the current pixel mode.
pub fn get_pixel_mode() -> PixelMode {
  unsafe { cpp::GetPixelMode() }
}

/// Changes the blend factor form between 0.0f to 1.0f.
pub fn set_pixel_blend(blend: f32) {
  unsafe { cpp::SetPixelBlend(blend) }
}

/// Draws pixel at coordinates (x, y).
pub fn draw(x: i32, y: i32, p: Pixel) -> bool {
  unsafe { cpp::Draw(x, y, p) }
}

/// Draws a line from (x1, y1) to (x2, y2).
#[inline]
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel) {
  draw_line_with_pattern(x1, y1, x2, y2, p, 0xFFFFFFFF)
}

/// Draws a line from (x1, y1) to (x2, y2).
/// Allows to set pattern.
pub fn draw_line_with_pattern(x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32) {
  unsafe { cpp::DrawLine(x1, y1, x2, y2, p, pattern) }
}

/// Draws a circle located at (x, y) with radius.
#[inline]
pub fn draw_circle(x: i32, y: i32, radius: i32, p: Pixel) {
  draw_circle_with_mask(x, y, radius, p, 0xFF)
}

/// Draws a circle located at (x, y) with radius.
/// Allows to set mask.
pub fn draw_circle_with_mask(x: i32, y: i32, radius: i32, p: Pixel, mask: u8) {
  unsafe { cpp::DrawCircle(x, y, radius, p, mask) }
}

/// Fills a circle located at (x, y) with radius.
pub fn fill_circle(x: i32, y: i32, radius: i32, p: Pixel) {
  unsafe { cpp::FillCircle(x, y, radius, p) }
}

/// Draws a rectangle at (x, y) to (x+w, y+h).
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32, p: Pixel) {
  unsafe { cpp::DrawRect(x, y, w, h, p) }
}

/// Fills a rectangle at (x, y) to (x+w, y+h).
pub fn fill_rect(x: i32, y: i32, w: i32, h: i32, p: Pixel) {
  unsafe { cpp::FillRect(x, y, w, h, p) }
}

/// Draws a triangle between points (x1, y1), (x2, y2) and (x3, y3).
pub fn draw_triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
  unsafe { cpp::DrawTriangle(x1, y1, x2, y2, x3, y3, p) }
}

/// Flat fills a triangle between points (x1, y1), (x2, y2) and (x3, y3).
pub fn fill_triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
  unsafe { cpp::FillTriangle(x1, y1, x2, y2, x3, y3, p) }
}

/// Draws string.
#[inline]
pub fn draw_string(x: i32, y: i32, text: &str, col: Pixel) -> Result<(), Error> {
  draw_string_with_scale(x, y, text, col, 1)
}

/// Draws string.
/// Allows to set scale.
pub fn draw_string_with_scale(x: i32, y: i32, text: &str, col: Pixel, scale: u32) -> Result<(), Error> {
  let ctext = CString::new(text)?;
  unsafe { cpp::DrawString(x, y, ctext.as_ptr(), col, scale) }
  Ok(())
}

/// Clears entire draw target to Pixel.
pub fn clear(p: Pixel) {
  unsafe { cpp::Clear(p) }
}

/// Clears the rendering back buffer.
pub fn clear_buffer(p: Pixel, depth: bool) {
  unsafe { cpp::ClearBuffer(p, depth) }
}
