use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use super::super::pieces::Piece;
use super::{Connected, Connection, Direction, Square};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

/// Has `square: Square`, `piece: u8`, `orientation: Orientation`
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Placement {
    pub square: Square<7>,
    pub piece: u8,
    pub orientation: Orientation,
}

impl From<&Placement> for u32 {
    fn from(placement: &Placement) -> u32 {
        let square = u32::from(placement.square.raw);
        let piece = u32::from(placement.piece) << 8;
        let orientation = u32::from(u8::from(&placement.orientation)) << 16;
        square ^ piece ^ orientation
    }
}

impl Debug for Placement {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let variant = self.orientation.rotation + 4 * u8::from(self.orientation.flip);
        write!(f, "{:?}{:02X?}{variant}", self.square, self.piece)
    }
}

impl Default for Placement {
    fn default() -> Self {
        Self {
            square: Square { raw: 255 },
            piece: 0x00,
            orientation: Orientation::default(),
        }
    }
}

impl FromStr for Placement {
    type Err = ();
    fn from_str(input: &str) -> Result<Placement, Self::Err> {
        let input: Vec<char> = input.chars().collect();
        let x = input.first().ok_or(())?.to_string();
        let x = x.parse::<u8>().expect("Could not parse Placement Square X");

        let y = input.get(1).ok_or(())?.to_string();
        let y = u8::from_str_radix(&y, 17).unwrap_or(255) - 10;

        let square = Square::new(x, y);

        let piece: String = input[2..=3].iter().collect();
        let piece = u8::from_str_radix(&piece, 16).expect("Could not parse Placement Piece");

        let orientation = input[4].to_string();
        let orientation = orientation
            .parse::<u8>()
            .expect("Could not parse Placement Orientation");
        let orientation = match orientation {
            0..=3 => Orientation::new(orientation, false),
            4..=8 => Orientation::new(orientation - 4, true),
            _ => panic!("Could not parse Placement Orientation"),
        };

        Ok(Placement {
            square,
            piece,
            orientation,
        })
    }
}

impl Placement {
    #[must_use]
    pub fn get_networks(&self) -> [Option<[Connection; 4]>; 2] {
        Piece::get_networks(self.piece, self.orientation)
    }

    #[must_use]
    pub fn get_connections_in_network(&self, index: u8) -> Vec<(Direction, Connection)> {
        match self.get_networks()[index as usize] {
            None => vec![],
            Some(connections) => Direction::iter()
                .map(|dir| (dir, connections[dir as usize]))
                .collect(),
        }
    }

    #[must_use]
    pub fn get_connected_network(
        &self,
        direction: &Direction,
        connection: &Connection,
    ) -> Option<u8> {
        for network_index in [0, 1] {
            if let Some(net) = self.get_networks()[network_index as usize] {
                if &net[*direction as usize] == connection {
                    return Some(network_index);
                }
            }
        }
        None
    }
}

impl Connected for Placement {
    fn connection(&self, direction: Direction) -> Connection {
        for net in self.get_networks().iter().flatten() {
            let connection = net[direction as usize];
            if connection != Connection::None {
                return connection;
            }
        }
        Connection::None
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Orientation {
    pub rotation: u8,
    pub flip: bool,
}

impl Orientation {
    #[must_use]
    pub fn new(rotation: u8, flip: bool) -> Self {
        Self { rotation, flip }
    }
}

impl From<&Orientation> for u8 {
    fn from(o: &Orientation) -> u8 {
        o.rotation + if o.flip { 4 } else { 0 }
    }
}
