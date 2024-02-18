use super::{Connection};

// I'll use this later ... maybe
// Update: No. Enums cannot have structs as custom values.
// Alternatively, we could define a struct for each piece type
// and have them all impl a common `PieceType`-trait
// However, Rust does not (yet) have trait fields,
// So each field would need a corresponding accessor in the trait.
pub enum PieceType {
  LRail,
  TRail,
  IRail,
  LRoad,
  TRoad,
  IRoad,
  Overpass,
  ITransition,
  LTransition,
  XTRoad,
  XTRail,
  XRoad,
  XRail,
  XL,
  XI,
}

// pub trait PieceVariant {
//   name: String,
//   networks: Vec<[Connection; 4]>,
//   rotations: [bool; 4],
//   flippable: bool,
// }

// impl PieceVariant {
//   fn new(
//     name: &str,
//     networks: Vec<[Connection; 4]>,
//     rotations: [bool; 4],
//     flippable: bool,
//   ) -> Self {
//     Self {
//       name: String::from(name),
//       networks,
//       rotations,
//       flippable,
//     }
//   }
// }