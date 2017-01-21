use super::faction::Faction;

pub trait Unit {
    fn id(&self) -> i64;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn speed_x(&self) -> f64;
    fn speed_y(&self) -> f64;
    fn angle(&self) -> f64;
    fn faction(&self) -> Faction;
}

#[macro_export]
macro_rules! unit_impl(
    ($t:ty) => (
        impl Unit for $t {
            fn id(&self) -> i64 {
                self.id()
            }

            fn x(&self) -> f64 {
                self.x()
            }

            fn y(&self) -> f64 {
                self.y()
            }

            fn speed_x(&self) -> f64 {
                self.speed_x()
            }

            fn speed_y(&self) -> f64 {
                self.speed_y()
            }

            fn angle(&self) -> f64 {
                self.angle()
            }

            fn faction(&self) -> Faction {
                self.faction()
            }
        }
    )
);
