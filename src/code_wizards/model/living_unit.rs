use super::circular_unit::CircularUnit;
use super::status::Status;

pub trait LivingUnit: CircularUnit {
    fn life(&self) -> i32;
    fn max_life(&self) -> i32;
    fn statuses(&self) -> &Vec<Status>;
}

#[macro_export]
macro_rules! living_unit_impl(
    ($t:ty) => (
        impl LivingUnit for $t {
            fn life(&self) -> i32 {
                self.life()
            }

            fn max_life(&self) -> i32 {
                self.max_life()
            }

            fn statuses(&self) -> &Vec<Status> {
                &self.statuses()
            }
        }
    )
);
