use creamcone::types::{
    Point,
    Blop,
    DOFType,
    Object,
    ObjectUniverse,
    ObjectConfiguration,
    Universe
};

fn main() {
    let mut ou = ObjectUniverse::new();
    ou.add(Object::new(
        &"test1".to_string(),
        &Point::new(0.0, 0.0),
        None,
        DOFType::Rotation,
        vec!(
            Blop::new(Point::new(100.0, 0.0), 1.0),
            Blop::new(Point::new(0.0, 100.0), 0.5),
            Blop::new(Point::new(-100.0, 0.0), 2.0),
            Blop::new(Point::new(0.0, -100.0), 3.0)
        )
    ));
    let c = ObjectConfiguration::new(&ou);
    let u = c.realize(&ou);
    u.tourChunks(|pt,c| {
        println!("{:?}: {:?}", pt, c);
    });
}
