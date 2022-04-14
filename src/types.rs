use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::swap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Clone)]
pub struct Blop {
    at: Point,
    strength: f64
}

#[derive(Debug, Clone)]
pub enum DOFType {
    Rotation,
    Linear(Point)
}

#[derive(Debug, Clone)]
pub struct Object {
    name: String,
    anchor: Point,
    target: Option<String>,
    dof: DOFType,
    blops: Vec<Blop>,
}

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug, Clone, Eq, Hash)]
pub struct IPoint {
    pub x: i32,
    pub y: i32
}

impl PartialEq for IPoint {
    fn eq(&self, rhs: &Self) -> bool {
       self.x == rhs.x && self.y == rhs.y
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x: x, y: y }
    }
}

impl IPoint {
    pub fn new(x: i32, y: i32) -> Self {
        IPoint { x: x, y: y }
    }
    pub fn add(&self, pt: IPoint) -> IPoint {
        IPoint { x: self.x + pt.x, y: self.y + pt.y }
    }
}

#[derive(Debug, Clone)]
pub struct Chunk {
    field: Vec<f64>
}

fn identity() -> Vec<f64> {
    vec!(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
}

fn rotmatrix(v: f64) -> Vec<f64> {
    vec!(f64::cos(v), f64::sin(v) * -1.0, 0.0, f64::sin(v), f64::cos(v), 0.0, 0.0, 0.0, 1.0)
}

fn txmatrix(x: f64, y: f64) -> Vec<f64> {
    vec!(0.0, 0.0, x, 0.0, 0.0, y, 0.0, 0.0, 1.0)
}

fn matmult(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![0.0; 9];
    for i in 0..3 {
        for j in 0..3 {
            let mut t = 0.0;
            for k in 0..3 {
                t += a[i * 3 + j] * b[k * 3 + j];
            }
            res[i * 3 + j] = t;
        }
    }
    res
}

fn transform_pt(v: &Vec<f64>, pt: &Point) -> Point {
    Point::new(v[0] * pt.x + v[1] * pt.y + v[2],
               v[3] * pt.x + v[4] * pt.y + v[5])
}

fn sqr(x: f64) -> f64 { x * x }

impl Blop {
    pub fn new(pt: Point, strength: f64) -> Self {
        Blop { at: pt, strength: strength }
    }

    fn transform_by(&self, tx: &Vec<f64>) -> Self {
        Blop { at: transform_pt(tx, &self.at), strength: self.strength }
    }

    fn field_at(&self, pt: &Point) -> f64 {
        let ld = 3.0;
        let d = sqr(self.at.x - pt.x) + sqr(self.at.y - pt.y);
        let f = self.strength / (1.0 + f64::exp((ld - d) * -1.0));
        //println!("field from {:?} at {:?} = {}", self, pt, f);
        f
    }
}

fn add_field(pt: &IPoint, blorps: &Vec<Blop>, chunk: &mut Chunk) {
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let fld_pt = Point::new((i + pt.x) as f64, (j + pt.y) as f64);
            for b in blorps {
                let fpt = IPoint::new(j, i);
                let fval = b.field_at(&fld_pt);
                chunk.setFieldValue(&fpt, fval);
            }
        }
    }
}

fn identify_chunks(blops: &Vec<Blop>) -> Vec<IPoint> {
    let mut res = Vec::new();
    for b in blops.iter() {
        let ipt = IPoint::new(b.at.x as i32, b.at.y as i32);
        for i in -1..=1 {
            for j in -1..=1 {
                res.push(ipt.add(IPoint::new(
                    i * CHUNK_SIZE,
                    j * CHUNK_SIZE
                )));
            }
        }
    }
    res
}

impl Object {
    pub fn new(
        name: &String,
        anchor: &Point,
        target: Option<String>,
        dof: DOFType,
        blops: Vec<Blop>
    ) -> Self {
        Object {
            name: name.clone(),
            anchor: anchor.clone(),
            target: target.clone(),
            dof: dof,
            blops: blops
        }
    }

    fn getTransform(&self, oc: &ObjectConfiguration, ou: &ObjectUniverse) -> Vec<f64> {
        let outerTransform = self.target.as_ref().map(|name| {
            let parentObject = ou.get(name).unwrap();
            ou.objects[parentObject].getTransform(oc, ou)
        }).unwrap_or_else(|| identity());

        let idx = ou.get(&self.name).unwrap();
        let v = oc.getPosition(idx);

        let initialTransform =
            match &self.dof {
                DOFType::Rotation => rotmatrix(v),
                DOFType::Linear(target) => txmatrix(target.x * v, target.y * v)
            };

        matmult(&outerTransform, &initialTransform)
    }

    fn realize(&self, tx: &Vec<f64>, u: &mut Universe) {
        let transformed_blops = self.blops.iter().map(|b| b.transform_by(tx)).collect();
        let chunks = identify_chunks(&transformed_blops);
        for c in chunks.iter() {
            match u.getChunk(c) {
                Some(chunkref) => {
                    chunkref.replace_with(|chunk| {
                        let mut d = Chunk::dead();
                        swap(chunk, &mut d);
                        add_field(&c, &transformed_blops, &mut d);
                        d
                    });
                },
                None => {
                    let mut chunk = Chunk::new();
                    add_field(&c, &transformed_blops, &mut chunk);
                    u.setChunk(&c, chunk);
                }
            }
        }
    }
}

impl ObjectUniverse {
    pub fn new() -> Self {
        ObjectUniverse { objects: Vec::new() }
    }

    fn get(&self, n: &String) -> Option<usize> {
        for i in 0..self.objects.len() {
            if &self.objects[i].name == n {
                return Some(i)
            }
        }
        None
    }

    fn getIdx(&self, n: usize) -> Rc<Object> {
        self.objects[n].clone()
    }

    pub fn add(&mut self, o: Object) {
        for i in 0..self.objects.len() {
            if self.objects[i].name == o.name {
                self.objects[i] = Rc::new(o);
                return;
            }
        }
        self.objects.push(Rc::new(o));
    }

    pub fn remove(&mut self, name: &String) {
        for i in 0..self.objects.len() {
            if &self.objects[i].name == name {
                self.objects.remove(i);
            }
        }
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { field: vec![0.0; (CHUNK_SIZE * CHUNK_SIZE) as usize] }
    }

    fn dead() -> Self {
        Chunk { field: Vec::new() }
    }

    pub fn getFieldValue(&self, pt: &IPoint) -> f64 {
        self.field[(pt.x & (CHUNK_SIZE - 1) + ((pt.y & (CHUNK_SIZE - 1)) * CHUNK_SIZE)) as usize]
    }

    pub fn setFieldValue(&mut self, pt: &IPoint, v: f64) {
        self.field[(pt.x & (CHUNK_SIZE - 1) + ((pt.y & (CHUNK_SIZE - 1)) * CHUNK_SIZE)) as usize] = v;
    }

    pub fn getData(&self) -> Vec<u8> {
        self.field.iter().map(|v| {
            if *v > 1.0 { 255 as u8 } else if *v < 0.0 { 0 as u8 } else {
                (255.0 * *v) as u8
            }
        }).collect()
    }
}

#[derive(Debug, Clone)]
pub struct Universe {
    chunks: HashMap<IPoint, RefCell<Chunk>>,
}

#[derive(Debug, Clone)]
pub struct ObjectUniverse {
    objects: Vec<Rc<Object>>
}

#[derive(Debug, Clone)]
pub struct ObjectConfiguration {
    pub positions: Vec<f64>
}

impl ObjectConfiguration {
    pub fn new(ou: &ObjectUniverse) -> Self {
        ObjectConfiguration { positions: vec![0.0; ou.objects.len()] }
    }

    pub fn perturb(&mut self) -> Self {
        let mut next = self.clone();
        for i in 0..self.positions.len() {
            let randval: f64 = rand::random();
            next.positions[i] = next.positions[i] + (randval - 0.5) * 0.001;
        }
        next
    }

    // render the configuration into a universe.
    pub fn realize(&self, ou: &ObjectUniverse) -> Universe {
        let mut u = Universe::new();
        for o in ou.objects.iter() {
            let transform = o.getTransform(self, ou);
            o.realize(&transform, &mut u);
        }
        u
    }

    fn getPosition(&self, idx: usize) -> f64 {
        self.positions[idx]
    }
}

fn chunkCoord(pt: &IPoint) -> IPoint {
    IPoint::new(pt.x - pt.x % CHUNK_SIZE, pt.y - pt.y % CHUNK_SIZE)
}

impl Universe {
    pub fn new()-> Self {
        Universe { chunks: HashMap::new() }
    }

    pub fn getChunk(&self, pt: &IPoint) -> Option<&RefCell<Chunk>> {
        self.chunks.get(&chunkCoord(pt))
    }

    pub fn setChunk(&mut self, pt: &IPoint, newchunk: Chunk) {
        match self.chunks.get(&chunkCoord(pt)) {
            Some(chunkcell) => {
                chunkcell.replace_with(|_| newchunk);
            },
            None => {
                self.chunks.insert(chunkCoord(pt), RefCell::new(newchunk));
            }
        }
    }

    pub fn tourChunks<T,F>(&self, f: F) -> Vec<T>
    where F: Fn(&IPoint, &Chunk) -> T {
        let mut res = Vec::new();
        for (pt, c) in self.chunks.iter() {
            res.push(f(pt, &c.borrow()));
        }
        res
    }
}
