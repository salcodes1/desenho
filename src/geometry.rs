#[derive(Clone, Copy, Debug)]
struct Physical;
#[derive(Clone, Copy, Debug)]
struct Logical;


#[derive(Clone, Copy, Debug)]
pub(crate) struct Rect<C: Clone + Copy> {
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

pub(crate) struct Region<C: Clone + Copy> {
    pub ops: Vec<RegionOp<C>>,
}

enum RegionOp<C: Clone + Copy> {
    Add(Rect<C>),
    Subtract(Rect<C>),
}

impl<C: Clone + Copy> Region<C> {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn add_rect(&mut self, rect: &Rect<C>) {
        self.ops.push(RegionOp::Add(*rect));
    }

    pub fn subtract_rect(&mut self, rect: &Rect<C>) {
        self.ops.push(RegionOp::Subtract(*rect));
    }

    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }
}