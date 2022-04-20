use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
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

pub const CHUNK_SIZE: i32 = 8;

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
    vec!(1.0, 0.0, 0.0,
         0.0, 1.0, 0.0,
         0.0, 0.0, 1.0)
}

fn rotmatrix(v: f64) -> Vec<f64> {
    vec!(f64::cos(v), f64::sin(v) * -1.0, 0.0,
         f64::sin(v), f64::cos(v),        0.0,
         0.0, 0.0, 1.0)
}

fn txmatrix(x: f64, y: f64) -> Vec<f64> {
    vec!(1.0, 0.0, x,
         0.0, 1.0, y,
         0.0, 0.0, 1.0)
}

fn matmult(a: &Vec<f64>, b: &Vec<f64>) -> Vec<f64> {
    let mut res = vec![0.0; 9];
    for i in 0..3 {
        for j in 0..3 {
            let mut t = 0.0;
            for k in 0..3 {
                t += a[i * 3 + k] * b[k * 3 + j];
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

pub fn create_field_vec() -> Vec<f64> {
    let ld = 5.0;
    let mut result = Vec::new();
    for i in 0..256 {
        let ds = f64::sqrt(i as f64);
        result.push(1.0 / (1.0 + f64::exp(ds - ld)));
    }
    result
}

impl Blop {
    pub fn new(pt: Point, strength: f64) -> Self {
        Blop { at: pt, strength: strength }
    }

    fn transform_by(&self, tx: &Vec<f64>) -> Self {
        Blop { at: transform_pt(tx, &self.at), strength: self.strength }
    }

    fn field_at(&self, field_ref: &Vec<f64>, pt: &Point) -> f64 {
        let dsqr = (sqr(self.at.x - pt.x) + sqr(self.at.y - pt.y)) as usize;
        if dsqr < field_ref.len() {
            self.strength * field_ref[dsqr]
        } else {
            0.0
        }
    }
}

fn add_field(field_ref: &Vec<f64>, pt: &IPoint, blorps: &Vec<Blop>, chunk: &mut Chunk) {
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let fpt = IPoint::new(j, i);
            let fld_pt = Point::new((j + pt.x) as f64, (i + pt.y) as f64);
            for b in blorps {
                let fval = b.field_at(field_ref, &fld_pt);
                chunk.addFieldValue(&fpt, fval);
            }
        }
    }
}

fn identify_chunks(blops: &Vec<Blop>) -> HashSet<IPoint> {
    let mut res = HashSet::new();
    for b in blops.iter() {
        let ipt = chunkCoord(&IPoint::new(b.at.x as i32, b.at.y as i32));
        for i in -1..=1 {
            for j in -1..=1 {
                res.insert(ipt.add(IPoint::new(
                    j * CHUNK_SIZE,
                    i * CHUNK_SIZE
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

        let modelTransform = txmatrix(self.anchor.x, self.anchor.y);

        matmult(&outerTransform, &matmult(&modelTransform, &initialTransform))
    }

    fn realize(&self, field_ref: &Vec<f64>, tx: &Vec<f64>, u: &mut Universe) {
        let transformed_blops = self.blops.iter().map(|b| b.transform_by(tx)).collect();
        let chunks = identify_chunks(&transformed_blops);
        for c in chunks.iter() {
            match u.getChunk(c) {
                Some(chunkref) => {
                    chunkref.replace_with(|chunk| {
                        let mut d = Chunk::dead();
                        swap(chunk, &mut d);
                        add_field(field_ref, &c, &transformed_blops, &mut d);
                        d
                    });
                },
                None => {
                    let mut chunk = Chunk::new();
                    add_field(field_ref, &c, &transformed_blops, &mut chunk);
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

    pub fn addFieldValue(&mut self, pt: &IPoint, v: f64) {
        self.field[(pt.x & (CHUNK_SIZE - 1)) as usize + ((pt.y & (CHUNK_SIZE - 1)) * CHUNK_SIZE) as usize] += v;
    }

    pub fn getData(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(self.field.len() * 3);
        for i in 0..self.field.len() {
            let v = self.field[i];
            let newv =
                if v > 10.0 { 255 as u8 } else if v < 0.0 { 0 as u8 } else {
                    (25.5 * v) as u8
                };
            res.push(255);
            res.push(newv);
            res.push(newv);
            res.push(newv);
        }
        res
    }

    pub fn mse(&self) -> f64 {
        let mut res = 0.0;
        for v in self.field.iter() {
            res += sqr(*v);
        }
        res
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

    pub fn perturb(&mut self, perturb: f64) -> Self {
        let mut next = self.clone();
        for i in 0..self.positions.len() {
            let randval: f64 = rand::random();
            next.positions[i] += ((randval - 0.5) * perturb);
        }
        next
    }

    // render the configuration into a universe.
    pub fn realize(&self, field_ref: &Vec<f64>, ou: &ObjectUniverse) -> Universe {
        let mut u = Universe::new();
        for o in ou.objects.iter() {
            let transform = o.getTransform(self, ou);
            o.realize(field_ref, &transform, &mut u);
        }
        u
    }

    fn getPosition(&self, idx: usize) -> f64 {
        self.positions[idx]
    }

    pub fn plus(&self, other: &ObjectConfiguration) -> ObjectConfiguration {
        let mut t = ObjectConfiguration { positions: self.positions.clone() };
        for i in 0..self.positions.len() {
            t.positions[i] += other.positions[i];
        }
        t
    }

    pub fn minus(&self, other: &ObjectConfiguration) -> ObjectConfiguration {
        let mut t = ObjectConfiguration { positions: self.positions.clone() };
        for i in 0..self.positions.len() {
            t.positions[i] -= other.positions[i];
        }
        t
    }
}

fn chunkCoord(pt: &IPoint) -> IPoint {
    IPoint::new(pt.x & !(CHUNK_SIZE - 1), pt.y & !(CHUNK_SIZE - 1))
}

impl Universe {
    pub fn new()-> Self {
        Universe { chunks: HashMap::new() }
    }

    pub fn getChunk(&self, pt: &IPoint) -> Option<&RefCell<Chunk>> {
        self.chunks.get(&chunkCoord(pt))
    }

    pub fn setChunk(&mut self, pt: &IPoint, newchunk: Chunk) {
        let cpt = chunkCoord(pt);
        match self.chunks.get(&cpt) {
            Some(chunkcell) => {
                chunkcell.replace_with(|_| newchunk);
            },
            None => {
                self.chunks.insert(cpt, RefCell::new(newchunk));
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

    pub fn mse(&self) -> f64 {
        let se = self.tourChunks(|pt,c| c.mse());
        let mut sum = 0.0;
        for v in se.iter() { sum += *v; }
        sum
    }
}

const GENERATIONS: usize = 2;
const FANOUT: usize = 12;
const FANIN: usize = 3;

pub fn update_simulation(
    field_ref: &Vec<f64>,
    c_: &ObjectConfiguration,
    ou: &ObjectUniverse
) -> ObjectConfiguration {
    let mut c = c_.clone();
    let mut ancestors = Vec::new();
    let mut copied_ancestors = Vec::new();
    let mut mses = Vec::new();
    let mut perturb = 0.02;

    for i in 0..FANIN {
        copied_ancestors.push(c.perturb(perturb));
    }

    for g in 0..GENERATIONS {
        perturb *= 0.8;

        ancestors.clear();
        for a in copied_ancestors.iter() {
            for j in 0..FANOUT / FANIN {
                if j != 0 {
                    ancestors.push(a.clone().perturb(perturb));
                } else {
                    ancestors.push(a.clone());
                }
            }
        }

        mses.clear();
        for i in 0..ancestors.len() {
            let a = &ancestors[i];
            let u = a.realize(field_ref, ou);
            mses.push((i, u.mse()));
        }
        mses.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        println!("{}:", g);
        copied_ancestors.clear();
        for i in 0..FANIN {
            copied_ancestors.push(ancestors[mses[0].0].clone());
            println!(" {:?}", mses[i]);
        }
    }

    return ancestors[mses[0].0].clone();
}
