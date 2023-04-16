use crate::math_helpers::EPSILON;
use crate::rays::Ray;
use crate::shapes::Shape;
use crate::tup::Tup;
use std::ops::Index;

type Object = Box<dyn Shape>;

pub struct Computations {
    intersection: Intersection,
    point: Tup,
    eyev: Tup,
    normalv: Tup,
    inside: bool,
    over_point: Tup,
    under_point: Tup,
    reflectv: Tup,
    n1: f64,
    n2: f64,
}

impl Computations {
    fn new(intersection: &Intersection, ray: &Ray, xs: &Intersections) -> Self {
        let point = ray.position(intersection.t());
        let eyev = -ray.direction();
        let n = intersection.object().normal_at(point);
        let inside = n.dot(&eyev) < 0.0;
        let normalv = if inside { -n } else { n };
        let (n1, n2) = Self::calc_n1_n2(intersection, xs);
        Self {
            intersection: intersection.clone(),
            point,
            eyev,
            normalv,
            inside,
            over_point: point + (normalv * EPSILON),
            under_point: point - (normalv * EPSILON),
            reflectv: ray.direction().reflect(&normalv),
            n1,
            n2,
        }
    }

    fn calc_n1_n2(intersection: &Intersection, xs: &Intersections) -> (f64, f64) {
        let likely_eq = |o1: &Object, o2: &Object| format!("{:?}", o1) == format!("{:?}", o2);
        let mut containers = Vec::new();
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        for i in 0..xs.len() {
            let inter = xs[i].clone();
            let is_hit = intersection.t() == inter.t();
            if is_hit {
                n1 = containers
                    .last()
                    .map_or(1.0, |j: &Object| j.material().refractive_index());
            };

            let index = containers.iter().position(|x| likely_eq(x, inter.object()));
            match index {
                Some(j) => {
                    containers.remove(j);
                }
                None => {
                    containers.push(inter.object().clone());
                }
            }

            if is_hit {
                n2 = containers
                    .last()
                    .map_or(1.0, |j: &Object| j.material().refractive_index());
                break;
            }
        }
        (n1, n2)
    }

    pub fn t(&self) -> f64 {
        self.intersection.t()
    }

    pub fn object(&self) -> &Object {
        self.intersection.object()
    }

    pub fn point(&self) -> Tup {
        self.point
    }

    pub fn eyev(&self) -> Tup {
        self.eyev
    }

    pub fn normalv(&self) -> Tup {
        self.normalv
    }

    pub fn inside(&self) -> bool {
        self.inside
    }

    pub fn over_point(&self) -> Tup {
        self.over_point
    }

    pub fn reflectv(&self) -> Tup {
        self.reflectv
    }

    pub fn n1(&self) -> f64 {
        self.n1
    }

    pub fn n2(&self) -> f64 {
        self.n2
    }

    pub fn under_point(&self) -> Tup {
        self.under_point
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(&self.normalv());
        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio * n_ratio * (1.0 - (cos * cos));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r = (self.n1 - self.n2) / (self.n1 + self.n2);
        let r0 = r * r;
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

#[derive(Clone)]
pub struct Intersection {
    t: f64,
    object: Object,
}

impl Intersection {
    pub fn new<T, U>(t: T, s: U) -> Self
    where
        T: Into<f64>,
        U: 'static + Shape,
    {
        Self {
            t: t.into(),
            object: Box::new(s),
        }
    }

    pub fn from_boxed_shape<T>(t: T, s: Box<dyn Shape>) -> Self
    where
        T: Into<f64>,
    {
        Self {
            t: t.into(),
            object: s,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn prepare_computations(&self, ray: &Ray, xs: &Intersections) -> Computations {
        Computations::new(self, &ray, &xs)
    }
}

pub struct Intersections {
    inters: Vec<Intersection>,
}

impl Intersections {
    pub fn new(inters: &[Intersection]) -> Self {
        Self {
            inters: inters.to_owned(),
        }
    }

    pub fn append(mut self, mut other: Intersections) -> Self {
        self.inters.append(&mut other.inters);
        self.inters
            .sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        self
    }

    pub fn push(mut self, i: Intersection) -> Self {
        self.inters.push(i);
        self
    }

    pub fn len(&self) -> usize {
        self.inters.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.inters
            .iter()
            .filter(|inter| inter.t() > 0.0)
            .min_by(|i1, i2| {
                i1.t()
                    .partial_cmp(&i2.t())
                    .expect("Intersections::hit got NaN")
            })
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Self::Output {
        &self.inters[i]
    }
}

impl Default for Intersections {
    fn default() -> Self {
        Self { inters: Vec::new() }
    }
}

#[cfg(test)]
mod intersections_test {
    use std::f64::consts;

    use super::*;
    use crate::matrix::Mat4;
    use crate::planes::Plane;
    use crate::spheres::Sphere;
    use crate::test_helpers::assert_nearly_eq;
    use crate::transforms;

    #[test]
    fn an_intersection_encapsulates_a_t() {
        let s = Sphere::default();
        let intersection = Intersection::new(3.5, s);
        assert_nearly_eq(3.5, intersection.t())
    }

    #[test]
    fn an_intersection_encapsulates_an_object() {
        let s = Sphere::default();
        let intersection = Intersection::new(3.5, s);
        assert_eq!(s.material(), intersection.object().material());
        assert_eq!(s.transform(), intersection.object().transform());
    }

    #[test]
    fn intersections_can_be_aggregated() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let mut xs = Intersections::default();
        xs = xs.push(i1);
        xs = xs.push(i2);
        assert_eq!(2, xs.len());
        assert_nearly_eq(1.0, xs[0].t());
        assert_nearly_eq(2.0, xs[1].t());
    }

    #[test]
    fn when_all_intersectons_are_positive_hit_returns_the_smallest_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1, s);
        let i2 = Intersection::new(2, s);
        let xs = Intersections::new(&[i1, i2]);
        let i = xs.hit();
        assert_eq!(1.0, i.expect("No hit occured").t());
    }

    #[test]
    fn when_there_are_negative_intersections_hit_returns_smallest_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1, s);
        let i2 = Intersection::new(1, s);
        let xs = Intersections::new(&[i1, i2]);
        let i = xs.hit();
        assert_eq!(1.0, i.expect("No hit occured").t());
    }

    #[test]
    fn when_all_intersections_have_negative_t_hit_returns_nothing() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2, s);
        let i2 = Intersection::new(-1, s);
        let xs = Intersections::new(&[i1, i2]);
        let i = xs.hit();
        assert!(i.is_none());
    }

    #[test]
    fn hit_always_returns_the_smallest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5, s);
        let i2 = Intersection::new(7, s);
        let i3 = Intersection::new(-3, s);
        let i4 = Intersection::new(2, s);
        let xs = Intersections::new(&[i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(2.0, i.expect("No hit occured").t());
    }

    #[test]
    fn the_state_of_an_intersection_can_be_precomputed() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert_eq!(comps.t(), i.t());
        assert_eq!(comps.object().material(), i.object().material());
        assert_eq!(comps.object().transform(), i.object().transform());
        assert_eq!(comps.point(), Tup::point(0, 0, -1));
        assert_eq!(comps.eyev(), Tup::vector(0, 0, -1));
        assert_eq!(comps.normalv(), Tup::vector(0, 0, -1));
    }

    #[test]
    fn the_computation_can_determine_that_the_intersection_occurs_on_outside() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert!(!comps.inside());
    }

    #[test]
    fn the_computation_can_determine_that_the_intersection_occurs_on_inside() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(1, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert_eq!(comps.point(), Tup::point(0, 0, 1));
        assert_eq!(comps.eyev(), Tup::vector(0, 0, -1));
        assert!(comps.inside());
        assert_eq!(comps.normalv(), Tup::vector(0, 0, -1));
    }

    #[test]
    fn the_hit_should_offset_the_point_by_a_small_amount() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default().with_transform(transforms::translation(0, 0, 1));
        let i = Intersection::new(5, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert!(comps.over_point().z < -EPSILON / 2.0);
    }

    #[test]
    fn the_hit_should_offset_in_the_direction_of_the_normal() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default().with_transform(transforms::translation(0, 0, 1));
        let i = Intersection::new(5, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert!(comps.point().z > comps.over_point().z);
    }

    #[test]
    fn the_refection_vector_should_be_precomputed() {
        let shape = Plane::default();
        let rad_2 = 2.0_f64.sqrt();
        let rad_2_over_2 = rad_2 / 2.0;
        let r = Ray::new(
            Tup::point(0, 1, -1),
            Tup::vector(0.0, -rad_2_over_2, rad_2_over_2),
        );
        let i = Intersection::new(rad_2, shape);
        let comps = i.prepare_computations(&r, &Intersections::new(&[i.clone()]));
        assert_eq!(
            comps.reflectv(),
            Tup::vector(0.0, rad_2_over_2, rad_2_over_2)
        );
    }

    fn custom_glass_sphere(refractive_index: f64, transform: Mat4) -> Sphere {
        let sphere = Sphere::glass_sphere().with_transform(transform);
        let material = sphere.material().with_refractive_index(refractive_index);
        sphere.with_material(material)
    }

    fn assert_n1_and_n2_of_at_intersection(i: usize, n1_expected: f64, n2_expected: f64) {
        let outer_a = custom_glass_sphere(1.5, transforms::scaling(2, 2, 2));
        let inner_b = custom_glass_sphere(2.0, transforms::translation(0.0, 0.0, -0.25));
        let inner_c = custom_glass_sphere(2.5, transforms::translation(0.0, 0.0, 0.25));
        let r = Ray::new(Tup::point(0, 0, 4), Tup::vector(0, 0, 1));
        let xs = Intersections::new(&[
            Intersection::new(2, outer_a),
            Intersection::new(2.75, inner_b),
            Intersection::new(3.25, inner_c),
            Intersection::new(4.75, inner_b),
            Intersection::new(5.25, inner_c),
            Intersection::new(6, outer_a),
        ]);
        let comps = xs[i].prepare_computations(&r, &xs);
        println!("n1 = {}, n2 = {}", comps.n1(), comps.n2());
        assert_nearly_eq(n1_expected, comps.n1());
        assert_nearly_eq(n2_expected, comps.n2());
    }

    #[test]
    fn correctly_calculates_n1_and_n2_at_different_iotersections() {
        assert_n1_and_n2_of_at_intersection(0, 1.0, 1.5);
        assert_n1_and_n2_of_at_intersection(1, 1.5, 2.0);
        assert_n1_and_n2_of_at_intersection(2, 2.0, 2.5);
        assert_n1_and_n2_of_at_intersection(3, 2.5, 2.5);
        assert_n1_and_n2_of_at_intersection(4, 2.5, 1.5);
        assert_n1_and_n2_of_at_intersection(5, 1.5, 1.0);
    }

    #[test]
    fn under_point_is_offset_just_below_surface() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let sphere = Sphere::glass_sphere().with_transform(transforms::translation(0, 0, 1));
        let i = Intersection::new(5, sphere);
        let xs = Intersections::new(&[i]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let under_point = comps.under_point();
        assert!(under_point.z > EPSILON / 2.0);
        assert!(comps.point().z < under_point.z);
    }

    #[test]
    fn the_schlick_aprox_under_total_internal_reflection_is_1() {
        let s = Sphere::glass_sphere();
        let r = Ray::new(
            Tup::point(0.0, 0.0, consts::SQRT_2 / 2.0),
            Tup::vector(0, 1, 0),
        );
        let i1 = Intersection::new(-consts::SQRT_2 / 2.0, s);
        let i2 = Intersection::new(consts::SQRT_2 / 2.0, s);
        let xs = Intersections::new(&[i1, i2]);
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();
        assert_nearly_eq(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approx_with_perpendicular_viewing_angle() {
        let s = Sphere::glass_sphere();
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 1, 0));
        let xs = Intersections::new(&[Intersection::new(-1.0, s), Intersection::new(1.0, s)]);
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();
        assert_nearly_eq(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approx_with_small_angle_and_n1_gt_n2() {
        let s = Sphere::glass_sphere();
        let r = Ray::new(Tup::point(0.0, 0.99, -2.0), Tup::vector(0, 0, 1));
        let xs = Intersections::new(&[Intersection::new(1.8589, s)]);
        let comps = xs[0].prepare_computations(&r, &xs);
        let reflectance = comps.schlick();
        assert_nearly_eq(reflectance, 0.48873);
    }
}
