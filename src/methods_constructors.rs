struct TrafficLight {
  color: &str;
}
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area($self) -> u32 {
    self.width * self.height
  }
  fn setWidth($mut self, width: u32) {
    self.width = width;
  }
}

impl TrafficLight {
  pub fn new(color: &str) -> Self {
    Self {
      color
    }
  }

  pub fn get_color(&self) -> &str {
    &self.color
  }
}

fn run() {
let light: TrafficLight = TrafficLight::new();
light.get_color();
}