use std::rc::Rc;
use model::intersect::Intersectable;
use na::{Point3, Vector3};

pub mod intersect;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>
}

impl Ray {
    pub fn eval(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }

    pub fn normalize_dir(&mut self) -> &Ray {
        self.direction.normalize_mut();
        self
    }
}

pub struct Material {
    pub ambient_reflectance: f64,
    pub diffuse_reflectance: Point3<f64>
}

/* Scene Graph model */
pub enum SceneObject {
    Group { child_nodes: Vec<SceneObject> },
    Node { model: Box<Intersectable> }
}

/* Some concrete models */
pub struct Sphere {
    pub material: Rc<Material>,
    pub center: Point3<f64>,
    pub radius: f64
}
