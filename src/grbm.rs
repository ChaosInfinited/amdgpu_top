/* ref: https://gitlab.freedesktop.org/tomstdenis/umr/ */
/* ref: https://gitlab.freedesktop.org/mesa/mesa/-/blob/main/src/gallium/drivers/radeonsi/si_gpu_load.c */

/* ref: https://developer.amd.com/wordpress/media/2013/10/R6xx_R7xx_3D.pdf */
/* ref: http://developer.amd.com/wordpress/media/2013/10/CIK_3D_registers_v2.pdf */

macro_rules! get_bit {
    ($reg: expr, $shift: expr) => {
        (($reg >> $shift) & 0b1) as u8
    };
}

pub struct GRBM {
    pub ta: u8, // Texture Addresser?
    pub gds: u8, // Global Data Share
    pub vgt: u8, // Vertex Grouper and Tessellator
    pub ia: u8, // Input Assembly?
    pub sx: u8, // Shader Export
    pub spi: u8, // Shader Pipe Interpolator
    pub bci: u8, // Barycentric interpolation control
    pub sc: u8, // Scan Convertor
    pub pa: u8, // Primitive Assembly
    pub db: u8, // Depth Block? Depth Buffer?
    pub cp: u8, // Command Processor?
    pub cb: u8, // Color Buffer
    pub gui_active: u8,
}

impl GRBM {
    pub const fn new() -> Self {
        Self {
            ta: 0,
            gds: 0,
            vgt: 0,
            ia: 0,
            sx: 0,
            spi: 0,
            bci: 0,
            sc: 0,
            pa: 0,
            db: 0,
            cp: 0,
            cb: 0,
            gui_active: 0,
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new()
    }

    pub fn acc(&mut self, reg: u32) {
        self.ta += get_bit!(reg, 14);
        self.gds += get_bit!(reg, 15);
        self.vgt += get_bit!(reg, 17);
        self.ia += get_bit!(reg, 19);
        self.sx += get_bit!(reg, 20);
        self.spi += get_bit!(reg, 22);
        self.bci += get_bit!(reg, 23);
        self.sc += get_bit!(reg, 24);
        self.pa += get_bit!(reg, 25);
        self.db += get_bit!(reg, 26);
        self.cp += get_bit!(reg, 29);
        self.cb += get_bit!(reg, 30);
        self.gui_active += get_bit!(reg, 31);
    }

    pub fn stat(&self) -> String {
        format!(
            concat!(
                " TA:{ta:3}%  VGT:{vgt:3}%\n",
                " SX:{sx:3}%  SPI:{spi:3}%\n",
                " DB:{db:3}%   CB:{cb:3}%\n",
                " CP:{cp:3}%  GUI:{gui:3}%\n",
            ),
            ta = self.ta,
            vgt = self.vgt,
            sx = self.sx,
            spi = self.spi,
            db = self.db,
            cb = self.cb,
            cp = self.cp,
            gui = self.gui_active,
        )
    }

    pub fn verbose_stat(&self) -> String {
        format!(
            concat!(
                " Texture Addressor:               {ta:3}% \n",
                " Vertex Grouper and Tessellator:  {vgt:3}% \n",
                " Shader Export:                   {sx:3}% \n",
                " Shader Pipe Interpolator:        {spi:3}% \n",
                " Depth Block:                     {db:3}% \n",
                " Color Buffer:                    {cb:3}% \n",
                " Command Processor:               {cp:3}% \n",
                " GUI Active:                      {gui:3}% \n",
            ),
            ta = self.ta,
            vgt = self.vgt,
            sx = self.sx,
            spi = self.spi,
            db = self.db,
            cb = self.cb,
            cp = self.cp,
            gui = self.gui_active,
        )
    }
}
