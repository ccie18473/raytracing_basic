use crate::prelude::*;

// canvas_w - canvas width
// canvas_h - canvas height
// canvas_x - canvas x-coordinate
// canvas_y - canvas y-coordinate
// viewport_w - viewport width
// viewport_h - viewport height
// projection_plane_d - distance from camera to viewport

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
}

impl Sphere {
    pub fn new(
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        color: Color,
        specular: f64,
        reflective: f64,
    ) -> Sphere {
        let center = Vector3::new(x, y, z);
        Sphere {
            center,
            radius,
            color,
            specular,
            reflective,
        }
    }
}

pub struct Objects {
    objects: Vec<Sphere>,
}

impl Objects {
    pub fn new() -> Objects {
        let mut objects = Vec::new();
        let mut sphere: Sphere;

        sphere = Sphere::new(0.0, -1.0, 3.0, 1.0, Color::RED, 500.0, 0.2);
        objects.push(sphere);
        sphere = Sphere::new(2.0, 0.0, 4.0, 1.0, Color::BLUE, 500.0, 0.3);
        objects.push(sphere);
        sphere = Sphere::new(-2.0, 0.0, 5.0, 1.0, Color::GREEN, 10.0, 0.4);
        //objects.push(sphere);
        //sphere = Sphere::new(0.0, -5001.0, 0.0, 5000.0, Color::YELLOW, 1000.0, 0.5);
        objects.push(sphere);

        Objects { objects }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LightType {
    Ambient,
    Point,
    Directional,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Light {
    pub ltype: LightType,
    pub intensity: f64,
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
}

impl Light {
    pub fn new(ltype: LightType, intensity: f64, pos: Vector3<f64>, dir: Vector3<f64>) -> Light {
        Light {
            ltype,
            intensity,
            pos,
            dir,
        }
    }
}

pub struct Effects {
    pub lights: Vec<Light>,
}
impl Effects {
    pub fn new() -> Effects {
        let mut lights = Vec::new();
        let mut light;

        light = Light::new(
            LightType::Ambient,
            0.2,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
        );
        lights.push(light);
        light = Light::new(
            LightType::Point,
            0.6,
            Vector3::new(2.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 0.0),
        );
        lights.push(light);
        light = Light::new(
            LightType::Directional,
            0.2,
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(1.0, 4.0, 4.0),
        );
        lights.push(light);

        Effects { lights }
    }
    pub fn compute_lighting(
        &mut self,
        scene_point: Vector3<f64>, // P
        normal: Vector3<f64>,      // N
        object_dir: Vector3<f64>,  // V
        specular: f64,             // s
    ) -> f64 {
        let mut intensity = 0.0;
        let mut light_dir = Vector3::new(0.0, 0.0, 0.0);
        let mut t_max = 0.0;
        let effects = Effects::new();
        //
        //
        //
        for light in effects.lights {
            if light.ltype == LightType::Ambient {
                intensity += light.intensity;
            } else if light.ltype == LightType::Point {
                light_dir = light.pos - scene_point;
                t_max = 1.0;
            } else {
                light_dir = light.dir;
                t_max = 1e30;
            }

            //
            // shadow calculation
            //
            let (_shadow_sphere, _shadow_t) =
                closest_intersection(scene_point, light_dir, 0.001, t_max);
            //if shadow_sphere != None {
            //continue
            //}
            //
            // diffuse calculation
            //
            let n_dot_l = cgmath::dot(normal, light_dir);
            if n_dot_l > 0.0 {
                intensity += light.intensity * n_dot_l / (normal.magnitude() * light_dir.magnitude());
            }
            //
            // specular calculation
            //
            let reflection: Vector3<f64>;
            let r_dot_v: f64;

            if specular != -1.0 {
                reflection = 2.0 * normal * cgmath::dot(normal, light_dir) - light_dir;
                r_dot_v = cgmath::dot(reflection, object_dir);

                if r_dot_v > 0.0 {
                    intensity += light.intensity
                        * (r_dot_v / reflection.magnitude() * object_dir.magnitude())
                            .powf(specular);
                }
            }
        }
        intensity
    }
}

pub fn intersect_ray_sphere(
    pos: Vector3<f64>,
    dir: Vector3<f64>,
    sphere: Sphere,
) -> (f64, f64) {
    let radius = sphere.radius;

    let camera_to_sphere = pos - sphere.center;
    //
    // quadratic equation
    //
    let a = cgmath::dot(dir, dir);
    let b = 2.0 * cgmath::dot(camera_to_sphere, dir);
    let c = cgmath::dot(camera_to_sphere, camera_to_sphere) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (1e30, 1e30);
    }

    let t1 = (-b + (discriminant).sqrt()) / (2.0 * a);
    let t2 = (-b - (discriminant).sqrt()) / (2.0 * a);

    return (t1, t2);
}

pub fn closest_intersection(
    pos: Vector3<f64>,      // O
    dir: Vector3<f64>,      // D
    t_min: f64,
    t_max: f64,
) -> (Sphere, f64) {
    let mut closest_t = 1e30;
    let mut closest_sphere = Sphere::new(0.0, 0.0, 0.0, 0.0, Color::BLACK, 0.0, 0.0);
    let objects = Objects::new();

    for sphere in objects.objects {
        let (t1, t2) = intersect_ray_sphere(pos, dir, sphere);

        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = sphere;
        }

        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = sphere;
        }
    }
    return (closest_sphere, closest_t);
}

// viewport_size = 1 x 1
// projection_plane_d = 1
// gives a FOV of 53 degrees

pub struct Ray {
    pub dir: Vector3<f64>,
    pub color: Color,
}

impl Ray {
    pub fn new() -> Ray {
        let dir = Vector3::new(0.0, 0.0, 0.0);
        let color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        Ray { dir, color }
    }
    pub fn reflect_ray(&mut self, ray_vec: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        return 2.0 * normal * cgmath::dot(normal, ray_vec) - ray_vec;
    }
    pub fn trace_ray(
        &mut self,
        point_pos: Vector3<f64>, // O
        ray_dir: Vector3<f64>,   // D
        t_min: f64,
        t_max: f64,
        recursion_depth: f64,
    ) -> Color {
        let (closest_sphere, closest_t) = closest_intersection(point_pos, ray_dir, t_min, t_max);
        //
        // compute local color
        //
        let scene_point = point_pos + closest_t * ray_dir; //   P
        let normal = scene_point - closest_sphere.center; //    N
        let normal = normal / normal.magnitude();

        let mut lights = Effects::new();

        let intensity =
            lights.compute_lighting(scene_point, normal, -ray_dir, closest_sphere.specular);

        let mut local_color = Color::BLACK;

        local_color.r = closest_sphere.color.r * intensity as f32;
        local_color.g = closest_sphere.color.g * intensity as f32;
        local_color.b = closest_sphere.color.b * intensity as f32;
        //
        // If we hit the recursion limit or the object is not reflective, we're done
        //
        let reflective = closest_sphere.reflective;

        if recursion_depth <= 0.0 || reflective <= 0.0 {
            return local_color;
        }
        //
        // Compute the reflected color
        //
        let reflected_ray = self.reflect_ray(-ray_dir, normal);

        let reflected_color = self.trace_ray(
            scene_point,
            reflected_ray,
            0.001,
            1e30,
            recursion_depth - 1.0,
        );

        local_color.r =
            local_color.r * (1.0 - reflective) as f32 + reflected_color.r * reflective as f32;
        local_color.g =
            local_color.g * (1.0 - reflective) as f32 + reflected_color.b * reflective as f32;
        local_color.b =
            local_color.b * (1.0 - reflective) as f32 + reflected_color.g * reflective as f32;

        local_color
    }
}

pub struct Rays {
    pub buffer: Vec<u8>,
}

impl Rays {
    pub fn new(camera: &mut Camera, canvas_w: isize, canvas_h: isize) -> Rays {
        let mut buffer = vec![0_u8; (canvas_w * canvas_h * 4) as usize];

        for x in 0..canvas_w {
            for y in 0..canvas_h {
                let mut ray = crate::Ray::new();
                ray.dir =
                    camera.canvas_to_viewport(x as f64, y as f64, canvas_w as f64, canvas_h as f64);
                camera.rotate_viewport(&mut ray.dir);

                let color = ray.trace_ray(camera.pos, ray.dir, 0.001, 1e30, 3.0);
                // put pixel
                let offset = ((y * canvas_w + x) * 4) as usize;
                buffer[offset] = (color.r * 255.0) as u8;
                buffer[offset + 1] = (color.g * 255.0) as u8;
                buffer[offset + 2] = (color.b * 255.0) as u8;
                buffer[offset + 3] = (color.a * 255.0) as u8;
            }
        }
        //draw the pixels as a buffer
        Rays { buffer }
    }
}
