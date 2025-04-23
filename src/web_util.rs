use web_sys::window;

pub fn is_mobile_device() -> bool {
  window()
    .and_then(|window| window.navigator().user_agent().ok())
    .is_some_and(|navigator| {
      navigator.contains("Mobi") || navigator.contains("Android") || navigator.contains("iPhone")
    })
}
