use crate::spheres::Sphere;
use std::ops::Index;

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

    pub fn intersections(i1: Self, i2: Self) -> Vec<Self> {
        vec![i1, i2]
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> Sphere {
        self.object
    }
}


pub struct Intersections {
    inters: Vec<Intersection>,
}

impl Intersections {
    fn new(inters: &[Intersection]) -> Self {
        Self {
            inters: inters.to_vec(),
        }
    }
    
    fn push(mut self, i: Intersection) -> Intersections {
        self.inters.push(i);
        self
    }

    fn len(&self) -> usize {
        self.inters.len()
    }

    fn hit(&self) -> Option<Intersection> {
        self.inters.iter()
            .filter(|inter| inter.t() > 0.0)
            .min_by(|i1, i2| {
                i1.t().partial_cmp(&i2.t()).expect("Intersections::hit got NaN")
            })
            .map(|is| is.to_owned())
    }
}

impl Default for Intersections {
    fn default() -> Self {
        Self {
            inters: Vec::new(),
        }
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
}
