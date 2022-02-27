use crate::rays::Ray;
use crate::spheres::Sphere;
use crate::tup::Tup;
use std::ops::Index;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Computations {
    t: f64,
    object: Sphere,
    point: Tup,
    eyev: Tup,
    normalv: Tup,
    inside: bool,
}

impl Computations {
    fn new(t: f64, object: Sphere, point: Tup, eyev: Tup, normalv: Tup, inside: bool) -> Self {
        Self { t, object, point, eyev, normalv, inside }
    }
    
    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> Sphere {
        self.object
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Sphere,
}

impl Intersection {
    pub fn new<T: Into<f64>>(t: T, s: Sphere) -> Self {
        Self {
            t: t.into(),
            object: s
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> Sphere {
        self.object
    }
    
    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let pt = ray.position(self.t());
        let eyev = -ray.direction();
        let normalv = self.object().normal_at(pt);
        let inside = normalv.dot(&eyev) < 0.0;
        Computations::new(
            self.t(),
            self.object(),
            pt,
            eyev,
            if inside { -normalv } else { normalv },
            inside
        )
    }

}

#[derive(Debug, Clone, Default)]
pub struct Intersections {
    inters: Vec<Intersection>,
}

impl Intersections {
    pub fn new(inters: &[Intersection]) -> Self {
        Self {
            inters: inters.to_vec(),
        }
    }

    pub fn append(mut self, mut other: Intersections) -> Self {
        self.inters.append(&mut other.inters);
        self.inters.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        self
    }
    
    pub fn push(mut self, i: Intersection) -> Self {
        self.inters.push(i);
        self
    }

    pub fn len(&self) -> usize {
        self.inters.len()
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.inters.iter()
            .filter(|inter| inter.t() > 0.0)
            .min_by(|i1, i2| {
                i1.t().partial_cmp(&i2.t()).expect("Intersections::hit got NaN")
            })
            .map(|is| is.to_owned())
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Self::Output {
        &self.inters[i]
    }
}

#[cfg(test)]
mod intersections_test {
    use super::*;
    use crate::math_helpers::nearly_eq;
  
    fn assert_nearly_eq(a: f64, b: f64) {
        assert!(nearly_eq(a, b));
    }
    
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
        assert_eq!(s, intersection.object())
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
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(Some(i1), i);
    }

    #[test]
    fn when_there_are_negative_intersections_hit_returns_smallest_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1, s);
        let i2 = Intersection::new(1, s);
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(Some(i2), i); 
    }

    #[test]
    fn when_all_intersections_have_negative_t_hit_returns_nothing() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2, s);
        let i2 = Intersection::new(-1, s);
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(None, i); 
    }

    #[test]
    fn hit_always_returns_the_smallest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5, s);
        let i2 = Intersection::new(7, s);
        let i3 = Intersection::new(-3, s);
        let i4 = Intersection::new(2, s);
        let xs = Intersections::new(&vec![i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(Some(i4), i); 
    }

    #[test]
    fn the_state_of_an_intersection_can_be_precomputed() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.t(), i.t());
        assert_eq!(comps.object(), i.object());
        assert_eq!(comps.point(), Tup::point(0, 0, -1));
        assert_eq!(comps.eyev(), Tup::vector(0, 0, -1));
        assert_eq!(comps.normalv(), Tup::vector(0, 0, -1));
    }

    #[test]
    fn the_computation_can_determine_that_the_intersection_occurs_on_outside() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(r);
        assert!(!comps.inside());
    }

    #[test]
    fn the_computation_can_determine_that_the_intersection_occurs_on_inside() {
        let r = Ray::new(Tup::point(0, 0, 0), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(1, shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.point(), Tup::point(0, 0, 1));
        assert_eq!(comps.eyev(), Tup::vector(0, 0, -1));
        assert!(comps.inside());
        assert_eq!(comps.normalv(), Tup::vector(0, 0, -1));
    }
}
