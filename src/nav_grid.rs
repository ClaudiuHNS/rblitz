use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use nalgebra::Vector3;

use std::{fs, io, path::Path};

bitflags! {
    pub struct NavCellFlags: u16 {
        const GRASS = 0b00000001;
        const UNPASSABLE = 0b00000010;
        const BUSY = 0b00000100;
        const TARGETTED = 0b00001000;
        const MARKED = 0b00010000;
        const PATHED = 0b00100000;
        const SEE_THROUGH = 0b01000000;
        const OTHER_DIRECTION_END_TO_START = 0b10000000;
    }
}

#[derive(Debug)]
struct NavGrid {
    pub header: NavGridHeader,
    pub cells: Vec<NavGridCell>,
    pub sampled_height_dist_x: f32,
    pub sampled_height_dist_y: f32,
    pub sampled_heights: Vec<f32>,
    pub hint_grid: Vec<(Vec<f32>, i16, i16)>,
    pub dimensions: Vector3<f32>,
}

impl NavGrid {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut data = &fs::read(path)?[..];

        let major_version = data.read_u8()?;
        let header = NavGridHeader::new(&mut data)?;
        let cells = (0..header.cell_count_x * header.cell_count_y)
            .into_iter()
            .map(|_| NavGridCell::new(&mut data))
            .collect::<Result<Vec<_>, _>>()?;
        let sampled_height_count_x = data.read_i32::<LE>()?;
        let sampled_height_count_y = data.read_i32::<LE>()?;
        let sampled_height_dist_x = data.read_f32::<LE>()?;
        let sampled_height_dist_y = data.read_f32::<LE>()?;
        let sampled_heights = (0..sampled_height_count_x * sampled_height_count_y)
            .into_iter()
            .map(|_| data.read_f32::<LE>())
            .collect::<Result<Vec<_>, _>>()?;
        let hint_grid = (0..900)
            .into_iter()
            .map(|_| {
                let buf = (0..900)
                    .into_iter()
                    .map(|_| data.read_f32::<LE>())
                    .collect::<Result<Vec<f32>, io::Error>>()?;
                Ok((buf, data.read_i16::<LE>()?, data.read_i16::<LE>()?))
            })
            .collect::<Result<Vec<_>, io::Error>>()?;
        let dimensions = header.max_grid_positions - header.min_grid_positions;
        Ok(Self {
            header,
            cells,
            sampled_height_dist_x,
            sampled_height_dist_y,
            sampled_heights,
            hint_grid,
            dimensions,
        })
    }
}

#[derive(Debug)]
pub struct NavGridCell {
    pub center_height: f32,
    pub session_id: i32,
    pub arrival_cost: f32,
    pub is_open: bool,
    pub heuristic: f32,
    pub _actor_list: u32,
    pub x: u16,
    pub y: u16,
    pub additional_cost: f32,
    pub hint_as_good: f32,
    pub additional_cost_ref_count: i32,
    pub good_cell_session_id: i32,
    pub ref_hint_weight: f32,
    pub arrival_direction: i8,
    pub flag: NavCellFlags,
    pub ref_hint_node: [u16; 2],
}

impl NavGridCell {
    fn new(data: &mut &[u8]) -> io::Result<Self> {
        Ok(NavGridCell {
            center_height: data.read_f32::<LE>()?,
            session_id: data.read_i32::<LE>()?,
            arrival_cost: data.read_f32::<LE>()?,
            is_open: data.read_u32::<LE>()? != 0,
            heuristic: data.read_f32::<LE>()?,
            _actor_list: {
                data.read_u32::<LE>()?;
                0
            },
            x: data.read_u16::<LE>()?,
            y: data.read_u16::<LE>()?,
            additional_cost: data.read_f32::<LE>()?,
            hint_as_good: data.read_f32::<LE>()?,
            additional_cost_ref_count: data.read_i32::<LE>()?,
            good_cell_session_id: data.read_i32::<LE>()?,
            ref_hint_weight: data.read_f32::<LE>()?,
            arrival_direction: data.read_i16::<LE>()? as i8,
            flag: NavCellFlags::from_bits_truncate(data.read_u16::<LE>()?),
            ref_hint_node: [data.read_u16::<LE>()?, data.read_u16::<LE>()?],
        })
    }
}

#[derive(Debug)]
pub struct NavGridHeader {
    pub min_grid_positions: Vector3<f32>,
    pub max_grid_positions: Vector3<f32>,
    pub cell_size: f32,
    pub cell_count_x: u32,
    pub cell_count_y: u32,
}

impl NavGridHeader {
    fn new(data: &mut &[u8]) -> io::Result<Self> {
        Ok(NavGridHeader {
            min_grid_positions: Vector3::new(
                data.read_f32::<LE>()?,
                data.read_f32::<LE>()?,
                data.read_f32::<LE>()?,
            ),
            max_grid_positions: Vector3::new(
                data.read_f32::<LE>()?,
                data.read_f32::<LE>()?,
                data.read_f32::<LE>()?,
            ),
            cell_size: data.read_f32::<LE>()?,
            cell_count_x: data.read_u32::<LE>()?,
            cell_count_y: data.read_u32::<LE>()?,
        })
    }
}
