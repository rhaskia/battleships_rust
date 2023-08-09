pub fn js_rand(bottom: u32, top: u32) -> u32 {
  let rand = js_sys::Math::random();
  let base: f64 = rand.try_into().unwrap();
  (base * top as f64).floor() as u32 + bottom
}