use na::{Vector3, dot};
use rand::{Rng, thread_rng};
use scene::light::{Lighting, LightingType};
use scene::Scene;
use model::{SceneObject, Ray};
use model::intersect::{Intersection, Intersectable};
use util::{Color, get_ndc};

pub fn trace(ray : &Ray, scene : &SceneObject, lightings : &Vec<Lighting>) -> Option<Color> {
    match scene.intersect(ray) {
        None => None,
        Some(int) => Some(lighting_model(&int, scene, lightings))
    }
}

pub fn lighting_model(intersection : &Intersection,
    scene : &SceneObject, lightings : &Vec<Lighting>) -> Color {
    let mut c = Color { intensities: Vector3::zeros() };
    for lighting in lightings {
        let ref l = lighting.light_pos;
        let lc = lighting.color;
        let inc_v = l.get_incident_vec(intersection.intersect_pt);
        let p = dot(&inc_v, &intersection.normal);
        match l {
            LightingType::Ambient =>
                { c = c + lc * intersection.material.ambient_reflectance },
            LightingType::Directional {dir} =>
                { c = c + lc * intersection.material.diffuse_reflectance * p },
            LightingType::Point {loc} =>
                { c = c + lc * intersection.material.diffuse_reflectance * p }
        }
    }
    c
}

//Less core but still important

pub fn gen_ray_antialiased_sample(x: u32, y: u32, scene: &Scene) -> Ray {
    let mut rng = thread_rng();
    let x_sample : f64 = (x as f64) + rng.gen::<f64>();
    let y_sample : f64 = (y as f64) + rng.gen::<f64>();
    let (x_ndc, y_ndc) = get_ndc(x_sample, y_sample, scene.width, scene.height);
    scene.camera.get_ray(x_ndc, y_ndc)
}
