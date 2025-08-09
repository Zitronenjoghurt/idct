pub mod mass;
pub mod temperature;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CuriosityProperty {
    Mass,
    Temperature,
}
