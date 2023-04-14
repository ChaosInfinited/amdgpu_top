use libdrm_amdgpu_sys::AMDGPU::DeviceHandle;
use crate::Opt;
mod utils;
use utils::*;

const PANEL_WIDTH: usize = 70;

pub const GFX10_GRBM_INDEX: &[(&str, usize)] = &[
    ("Graphics Pipe", 31),
    ("Texture Pipe", 14),
    // ("Command Processor", 29),
    // ("Global Data Share", 15),
    ("Shader Export", 20),
    ("Shader Processor Interpolator", 22),
    ("Primitive Assembly", 25),
    ("Depth Block", 26),
    ("Color Block", 30),
    ("Geometry Engine", 21),
];

pub const GRBM_INDEX: &[(&str, usize)] = &[
    ("Graphics Pipe", 31),
    ("Texture Pipe", 14),
    // ("Command Processor", 29),
    // ("Global Data Share", 15),
    ("Shader Export", 20),
    ("Shader Processor Interpolator", 22),
    ("Primitive Assembly", 25),
    ("Depth Block", 26),
    ("Color Block", 30),
    ("Vertext Grouper / Tessellator", 17),
    ("Input Assembly", 19),
    ("Work Distributor", 21),
];

pub const GRBM2_INDEX: &[(&str, usize)] = &[
    ("Texture Cache", 25),
    ("Command Processor -  Fetcher", 28),
    ("Command Processor -  Compute", 29),
    ("Command Processor - Graphics", 30),
];

mod pc_type;
pub use pc_type::*;

mod perf_counter;
pub use perf_counter::*;

mod fdinfo;
pub use fdinfo::*;

mod vram_usage;
pub use vram_usage::*;

mod sensors;
pub use sensors::*;

mod gpu_metrics;
pub use gpu_metrics::*;

mod pcie_bw;
pub use pcie_bw::*;
