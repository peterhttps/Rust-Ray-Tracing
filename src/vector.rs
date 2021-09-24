use nalgebra::{Vector3, ArrayStorage, Matrix, U1, U3};

#[derive(Copy, Clone)]
pub struct Ray {
  pub origin: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
  pub direction: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
}

impl Ray {
  pub fn new(origin: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, direction: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) -> Ray {
    Ray {
      origin,
      direction
    }
  }
}

#[derive(Copy, Clone)]
pub struct Sphere {
  pub center: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, radius: f32) -> Sphere {
    Sphere {
      center,
      radius
    }
  }
}

#[derive(Clone)]
pub struct SceneList {
  pub objects: Vec<Sphere>
}

impl SceneList {
  pub fn new() -> SceneList {
    SceneList {
      objects: vec![]
    }
  }

  pub fn push(&mut self, object: Sphere) {
    self.objects.push(object);
  }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
  pub p: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>,
  pub t: f32,
  pub normal: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>
}

impl HitRecord {
  pub fn new(p: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, t: f32, normal:Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) -> HitRecord {
    HitRecord {
      p,
      t,
      normal
    }
  }

  pub fn default() -> HitRecord {
    Self::new(Vector3::new(0.0, 0.0, 0.0), 0.0, Vector3::new(0.0, 0.0, 0.0))
  }
}


pub fn unitvector(v : Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) ->  Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>{
  v / v.norm()
}

pub fn ray_at(ray: Ray, t: f32) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  ray.origin + t * ray.direction
}

pub fn hit(sphere: Sphere, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
  let a = ray.direction.norm_squared();
  let oc = ray.origin - sphere.center;
  let halfb = ray.direction.dot(&oc);
  let c = oc.norm_squared() - f32::powf(sphere.radius, 2.0);
  let discriminant = halfb*halfb - a*c;

  if discriminant < 0.0 {
    return false;
  } else {
    let sqrd = discriminant.sqrt();
    let t = (-halfb - sqrd) / a;

    if t < t_min || t > t_max {
      let t = (-halfb + sqrd) / a;

      if t < t_min || t > t_max {
        return false;
      }
    }
    record.t = t;
    record.p = ray_at(ray, t);
    let outward_normal = unitvector(record.p - sphere.center);
    let front_face = ray.direction.dot(&outward_normal) < 0.0;

     if front_face {
      record.normal = outward_normal
    } else {
      record.normal = -outward_normal;
    }
    return true;
  }
}

pub fn scene_list_hit(scenelist: SceneList, ray: Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
  let mut hit_anything = false;
  let mut temprecord = HitRecord::default();
  let mut closest_so_far = t_max;

  for object in scenelist.objects {
    if hit(object, ray, t_min, closest_so_far, &mut temprecord) {
      hit_anything = true;
      closest_so_far = temprecord.t;

      record.p = temprecord.p;
      record.t = temprecord.t;
      record.normal = temprecord.normal;
    }
  }

  hit_anything
}

pub fn reflect(dir: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>, normal: Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>>) -> Matrix<f32, U3, U1, ArrayStorage<f32, U3, U1>> {
  dir - 2.0 * dir.dot(&normal) * normal
}