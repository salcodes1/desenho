struct Physical;
struct Logical;



struct Rect<C> {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    _marker: std::marker::PhantomData<C>,
}

impl Rect<Physical> {
    pub fn to_logical(&self, scale_factor: f64) -> Rect<Logical> {
        Rect {
            x: (self.x as f64 / scale_factor) as i32,
            y: (self.y as f64 / scale_factor) as i32,
            width: (self.width as f64 / scale_factor) as u32,
            height: (self.height as f64 / scale_factor) as u32,
            _marker: std::marker::PhantomData,
        }
    }
}

impl Rect<Logical> {
    pub fn to_physical(&self, scale_factor: f64) -> Rect<Physical> {
        Rect {
            x: (self.x as f64 * scale_factor) as i32,
            y: (self.y as f64 * scale_factor) as i32,
            width: (self.width as f64 * scale_factor) as u32,
            height: (self.height as f64 * scale_factor) as u32,
            _marker: std::marker::PhantomData,
        }
    }
}