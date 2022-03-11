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
    reflectv: Tup,
}

impl Computations {
    fn new(intersection: &Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.t());
        let eyev = -ray.direction();
        let n = intersection.object().normal_at(point);
        let inside = n.dot(&eyev) < 0.0;
        let normalv = if inside { -n } else { n };
        Self {
            intersection: intersection.clone(),
            point,
            eyev,
            normalv,
            inside,
            over_point: point + (normalv * EPSILON),
            reflectv: ray.direction().reflect(&normalv)
        }
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
    
    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        Computations::new(self, &ray)
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

    pub fn hit(&self) -> Option<&Intersection> {
        self.inters.iter()
            .filter(|inter| inter.t() > 0.0)
            .min_by(|i1, i2| {
                i1.t().partial_cmp(&i2.t()).expect("Intersections::hit got NaN")
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
        Self {
            inters: Vec::new(),
        }
    }
}

#[cfg(test)]
mod intersections_test {
    use super::*;
    use crate::test_helpers::assert_nearly_eq;
    use crate::spheres::Sphere;
    use crate::planes::Plane;
    use crate::transforms::translation;
  
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
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(1.0, i.expect("No hit occured").t());
    }

    #[test]
    fn when_there_are_negative_intersections_hit_returns_smallest_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1, s);
        let i2 = Intersection::new(1, s);
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(1.0, i.expect("No hit occured").t()); 
    }

    #[test]
    fn when_all_intersections_have_negative_t_hit_returns_nothing() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2, s);
        let i2 = Intersection::new(-1, s);
        let xs = Intersections::new(&vec![i1, i2]);
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
        let xs = Intersections::new(&vec![i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(2.0, i.expect("No hit occured").t()); 
    }

    #[test]
    fn the_state_of_an_intersection_can_be_precomputed() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default();
        let i = Intersection::new(4, shape);
        let comps = i.prepare_computations(r);
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

    #[test]
    fn the_hit_should_offset_the_point_by_a_small_amount() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default().with_transform(translation(0, 0, 1));
        let i = Intersection::new(5, shape);
        let comps = i.prepare_computations(r);
        assert!(comps.over_point().z < -EPSILON/2.0);
    }

    #[test]
    fn the_hit_should_offset_in_the_direction_of_the_normal() {
        let r = Ray::new(Tup::point(0, 0, -5), Tup::vector(0, 0, 1));
        let shape = Sphere::default().with_transform(translation(0, 0, 1));
        let i = Intersection::new(5, shape);
        let comps = i.prepare_computations(r);
        assert!(comps.point().z > comps.over_point().z);        
    }

    #[test]
    fn the_refection_vector_should_be_precomputed() {
        let shape = Plane::default();
        let rad_2 = 2.0_f64.sqrt();
        let rad_2_over_2 = rad_2 / 2.0;
        let r = Ray::new(Tup::point(0, 1, -1), Tup::vector(0.0, -rad_2_over_2, rad_2_over_2));
        let i = Intersection::new(rad_2, shape);
        let comps = i.prepare_computations(r);
        assert_eq!(comps.reflectv(), Tup::vector(0.0, rad_2_over_2, rad_2_over_2));        
    }
}
