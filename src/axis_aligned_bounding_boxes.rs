use crate::interval::Interval;
use crate::ray::Ray;
pub struct AxisAlignedBoundingBox {
    pub interval_x: Interval,
    pub interval_y: Interval,
    pub interval_z: Interval,
}
impl AxisAlignedBoundingBox {
    pub fn new(interval_x: Interval, interval_y: Interval, interval_z: Interval) -> Self {
        Self {
            interval_x,
            interval_y,
            interval_z,
        }
    }
    pub fn merge(box1: &AxisAlignedBoundingBox, box2: &AxisAlignedBoundingBox) -> Self {
        AxisAlignedBoundingBox {
            interval_x: Interval::new(
                box1.interval_x.min.min(box2.interval_x.min),
                box1.interval_x.max.max(box2.interval_x.max),
            ),
            interval_y: Interval::new(
                box1.interval_y.min.min(box2.interval_y.min),
                box1.interval_y.max.max(box2.interval_y.max),
            ),
            interval_z: Interval::new(
                box1.interval_z.min.min(box2.interval_z.min),
                box1.interval_z.max.max(box2.interval_z.max),
            ),
        }
    }
    pub fn longest_axis(&self) -> usize {
        if self.interval_x.size() > self.interval_y.size() {
            if self.interval_z.size() > self.interval_x.size() {
                return 2;
            }
            0
        } else {
            if self.interval_y.size() > self.interval_z.size() {
                return 1;
            }
            2
        }
    }
    fn get_interval(&self, n: i32) -> &Interval {
        if n == 0 {
            return &self.interval_x;
        }
        if n == 1 {
            return &self.interval_y;
        }
        &self.interval_z
    }
    fn get_origin(&self, n: i32, ray: &Ray) -> f64 {
        if n == 0 {
            return ray.origin.x;
        }
        if n == 1 {
            return ray.origin.y;
        }
        ray.origin.z
    }
    fn get_direction(&self, n: i32, ray: &Ray) -> f64 {
        if n == 0 {
            return ray.direction.x;
        }
        if n == 1 {
            return ray.direction.y;
        }
        ray.direction.z
    }
    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        for i in 0..3 {
            let origin = self.get_origin(i, ray);
            let direction = self.get_direction(i, ray);
            let interval = self.get_interval(i);
            if direction == 0.0 {
                if origin < interval.min || origin > interval.max {
                    return false;
                }
                continue;
            }
            let t_0 = (interval.min - origin) / direction;
            let t_1 = (interval.max - origin) / direction;
            if t_0 < t_1 {
                if t_0 > t_min {
                    t_min = t_0;
                }
                if t_1 < t_max {
                    t_max = t_1;
                }
            }
            if t_0 > t_1 {
                if t_0 < t_max {
                    t_max = t_0;
                }
                if t_1 > t_min {
                    t_min = t_1;
                }
            }
        }
        if t_min >= t_max {
            return false;
        }
        true
    }
}
