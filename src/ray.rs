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
    pub center: Vector3<f32>,
    pub radius: f32,
    pub color: Color,
    pub specular: f32,
    pub reflective: f32,
}

impl Sphere {
    pub fn new(
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
        color: Color,
        specular: f32,
        reflective: f32,
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
        sphere = Sphere::new(-2.0, 0.0, 4.0, 1.0, Color::GREEN, 10.0, 0.4);
        objects.push(sphere);
        sphere = Sphere::new(0.0, -5001.0, 0.0, 5000.0, Color::YELLOW, 1000.0, 0.5);
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
    pub intensity: f32,
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Light {
    pub fn new(ltype: LightType, intensity: f32, pos: Vector3<f32>, dir: Vector3<f32>) -> Light {
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
        point_pos: Vector3<f32>, // P
        normal: Vector3<f32>,    // N
        point_dir: Vector3<f32>, // V
        specular: f32,           // s
    ) -> f32 {
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
                light_dir = light.pos - point_pos;
                t_max = 1.0;
            } else {
                light_dir = light.dir;
                t_max = 1e30;
            }
            //
            // shadow calculation
            //
            let (shadow_sphere, _shadow_t) =
                closest_intersection(point_pos, light_dir, 0.001, t_max);
            if !shadow_sphere.is_none() {
                continue;
            }
            //
            // diffuse calculation
            //
            let n_dot_l = cgmath::dot(normal, light_dir);
            if n_dot_l > 0.0 {
                intensity +=
                    light.intensity * n_dot_l / (normal.magnitude() * light_dir.magnitude());
            }
            //
            // specular calculation
            //
            let reflection: Vector3<f32>;
            let r_dot_v: f32;

            if specular != -1.0 {
                reflection = 2.0 * normal * cgmath::dot(normal, light_dir) - light_dir;
                r_dot_v = cgmath::dot(reflection, point_dir);

                if r_dot_v > 0.0 {
                    intensity += light.intensity
                        * (r_dot_v / (reflection.magnitude() * point_dir.magnitude())).powf(specular);
                }
            }
        }
        intensity
    }
}

pub fn intersect_ray_sphere(pos: Vector3<f32>, dir: Vector3<f32>, sphere: Sphere) -> (f32, f32) {
    let radius = sphere.radius;

    let ray_to_sphere = pos - sphere.center;
    //
    // quadratic equation
    //
    let a = cgmath::dot(dir, dir);
    let b = 2.0 * cgmath::dot(ray_to_sphere, dir);
    let c = cgmath::dot(ray_to_sphere, ray_to_sphere) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (1e30, 1e30);
    }

    let t1 = (-b + (discriminant).sqrt()) / (2.0 * a);
    let t2 = (-b - (discriminant).sqrt()) / (2.0 * a);

    (t1, t2)
}

pub fn closest_intersection(
    pos: Vector3<f32>, // O
    dir: Vector3<f32>, // D
    t_min: f32,
    t_max: f32,
) -> (Option<Sphere>, f32) {
    let mut closest_t = 1e30;
    let mut closest_sphere = None;
    let objects = Objects::new();

    for sphere in objects.objects {
        let (t1, t2) = intersect_ray_sphere(pos, dir, sphere);

        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }

        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }
    return (closest_sphere, closest_t);
}

pub struct Ray {
    pub dir: Vector3<f32>,
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
    pub fn reflect_ray(&mut self, ray_dir: Vector3<f32>, normal: Vector3<f32>) -> Vector3<f32> {
        2.0 * normal * cgmath::dot(normal, ray_dir) - ray_dir
    }
    pub fn trace_ray(
        &mut self,
        point_pos: Vector3<f32>, // O, P
        ray_dir: Vector3<f32>,   // D
        t_min: f32,
        t_max: f32,
        recursion_depth: f32,
    ) -> Color {
        let (closest_sphere, closest_t) = closest_intersection(point_pos, ray_dir, t_min, t_max);

        if closest_sphere.is_none() {
            return Color::BLACK;
        }
        let sphere = closest_sphere.unwrap();
        //
        // compute local color
        //
        let point_pos = point_pos + closest_t * ray_dir; //   P
        let normal = point_pos - sphere.center; //    N
        let normal = normal / normal.magnitude();
        let mut local_color = Color::BLACK;
        //
        // compute light
        //
        let mut lights = Effects::new();

        let intensity = lights.compute_lighting(point_pos, normal, -ray_dir, sphere.specular);
        //
        // apply light
        //
        local_color.r = sphere.color.r * intensity;
        local_color.g = sphere.color.g * intensity;
        local_color.b = sphere.color.b * intensity;
        //
        // if we hit the recursion limit or the object is not reflective, we're done
        //
        let reflective = sphere.reflective;

        if recursion_depth <= 0.0 || reflective <= 0.0 {
            return local_color;
        }
        //
        // compute the reflected color
        //
        let reflected_ray = self.reflect_ray(-ray_dir, normal);

        let reflected_color =
            self.trace_ray(point_pos, reflected_ray, 0.001, 1e30, recursion_depth - 1.0);
        //
        // apply reflexion
        //
        local_color.r = local_color.r * (1.0 - reflective) + reflected_color.r * reflective;
        local_color.g = local_color.g * (1.0 - reflective) + reflected_color.b * reflective;
        local_color.b = local_color.b * (1.0 - reflective) + reflected_color.g * reflective;
        //
        // return the ray color
        //
        local_color
    }
}

pub struct Rays {
    pub buffer: Vec<u8>,
}

impl Rays {
    pub fn new(camera: &mut Camera, canvas_w: isize, canvas_h: isize) -> Rays {
        let mut ray = crate::Ray::new();
        let mut color: Color;
        let mut offset: usize;
        let mut buffer = vec![0_u8; (canvas_w * canvas_h * 4) as usize];

        for x in 0..canvas_w {
            for y in 0..canvas_h {
                ray.dir =
                    camera.canvas_to_viewport(x as f32, y as f32, canvas_w as f32, canvas_h as f32);
                // apply rotation
                camera.rotate_viewport(&mut ray.dir);
                // run raytracing algorithm
                color = ray.trace_ray(camera.pos, ray.dir, 0.001, 1e30, 3.0);
                // convert color 0..1 to 0..255
                let (r, g, b) = color.to_rgb();
                // put pixel on buffer
                offset = ((y * canvas_w + x) * 4) as usize;
                buffer[offset] = r;
                buffer[offset + 1] = g;
                buffer[offset + 2] = b;
                buffer[offset + 3] = 255;
            }
        }
        //draw the pixels as a buffer
        Rays { buffer }
    }
}
