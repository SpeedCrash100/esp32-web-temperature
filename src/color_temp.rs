pub struct ColorTemp {
    color_temp: u16,
    r: u8,
    g: u8,
    b: u8,
}

impl ColorTemp {
    #[inline]
    pub fn r(&self) -> u8 {
        self.r
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.g
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.b
    }
}

pub const KELVIN_TABLE: &[ColorTemp] = &[
    ColorTemp {
        color_temp: 1000,
        r: 255,
        g: 56,
        b: 0,
    },
    ColorTemp {
        color_temp: 1100,
        r: 255,
        g: 71,
        b: 0,
    },
    ColorTemp {
        color_temp: 1200,
        r: 255,
        g: 83,
        b: 0,
    },
    ColorTemp {
        color_temp: 1300,
        r: 255,
        g: 93,
        b: 0,
    },
    ColorTemp {
        color_temp: 1400,
        r: 255,
        g: 101,
        b: 0,
    },
    ColorTemp {
        color_temp: 1500,
        r: 255,
        g: 109,
        b: 0,
    },
    ColorTemp {
        color_temp: 1600,
        r: 255,
        g: 115,
        b: 0,
    },
    ColorTemp {
        color_temp: 1700,
        r: 255,
        g: 121,
        b: 0,
    },
    ColorTemp {
        color_temp: 1800,
        r: 255,
        g: 126,
        b: 0,
    },
    ColorTemp {
        color_temp: 1900,
        r: 255,
        g: 131,
        b: 0,
    },
    ColorTemp {
        color_temp: 2000,
        r: 255,
        g: 138,
        b: 18,
    },
    ColorTemp {
        color_temp: 2100,
        r: 255,
        g: 142,
        b: 33,
    },
    ColorTemp {
        color_temp: 2200,
        r: 255,
        g: 147,
        b: 44,
    },
    ColorTemp {
        color_temp: 2300,
        r: 255,
        g: 152,
        b: 54,
    },
    ColorTemp {
        color_temp: 2400,
        r: 255,
        g: 157,
        b: 63,
    },
    ColorTemp {
        color_temp: 2500,
        r: 255,
        g: 161,
        b: 72,
    },
    ColorTemp {
        color_temp: 2600,
        r: 255,
        g: 165,
        b: 79,
    },
    ColorTemp {
        color_temp: 2700,
        r: 255,
        g: 169,
        b: 87,
    },
    ColorTemp {
        color_temp: 2800,
        r: 255,
        g: 173,
        b: 94,
    },
    ColorTemp {
        color_temp: 2900,
        r: 255,
        g: 177,
        b: 101,
    },
    ColorTemp {
        color_temp: 3000,
        r: 255,
        g: 180,
        b: 107,
    },
    ColorTemp {
        color_temp: 3100,
        r: 255,
        g: 184,
        b: 114,
    },
    ColorTemp {
        color_temp: 3200,
        r: 255,
        g: 187,
        b: 120,
    },
    ColorTemp {
        color_temp: 3300,
        r: 255,
        g: 190,
        b: 126,
    },
    ColorTemp {
        color_temp: 3400,
        r: 255,
        g: 193,
        b: 132,
    },
    ColorTemp {
        color_temp: 3500,
        r: 255,
        g: 196,
        b: 137,
    },
    ColorTemp {
        color_temp: 3600,
        r: 255,
        g: 199,
        b: 143,
    },
    ColorTemp {
        color_temp: 3700,
        r: 255,
        g: 201,
        b: 148,
    },
    ColorTemp {
        color_temp: 3800,
        r: 255,
        g: 204,
        b: 153,
    },
    ColorTemp {
        color_temp: 3900,
        r: 255,
        g: 206,
        b: 159,
    },
    ColorTemp {
        color_temp: 4000,
        r: 255,
        g: 209,
        b: 163,
    },
    ColorTemp {
        color_temp: 4100,
        r: 255,
        g: 211,
        b: 168,
    },
    ColorTemp {
        color_temp: 4200,
        r: 255,
        g: 213,
        b: 173,
    },
    ColorTemp {
        color_temp: 4300,
        r: 255,
        g: 215,
        b: 177,
    },
    ColorTemp {
        color_temp: 4400,
        r: 255,
        g: 217,
        b: 182,
    },
    ColorTemp {
        color_temp: 4500,
        r: 255,
        g: 219,
        b: 186,
    },
    ColorTemp {
        color_temp: 4600,
        r: 255,
        g: 221,
        b: 190,
    },
    ColorTemp {
        color_temp: 4700,
        r: 255,
        g: 223,
        b: 194,
    },
    ColorTemp {
        color_temp: 4800,
        r: 255,
        g: 225,
        b: 198,
    },
    ColorTemp {
        color_temp: 4900,
        r: 255,
        g: 227,
        b: 202,
    },
    ColorTemp {
        color_temp: 5000,
        r: 255,
        g: 228,
        b: 206,
    },
    ColorTemp {
        color_temp: 5100,
        r: 255,
        g: 230,
        b: 210,
    },
    ColorTemp {
        color_temp: 5200,
        r: 255,
        g: 232,
        b: 213,
    },
    ColorTemp {
        color_temp: 5300,
        r: 255,
        g: 233,
        b: 217,
    },
    ColorTemp {
        color_temp: 5400,
        r: 255,
        g: 235,
        b: 220,
    },
    ColorTemp {
        color_temp: 5500,
        r: 255,
        g: 236,
        b: 224,
    },
    ColorTemp {
        color_temp: 5600,
        r: 255,
        g: 238,
        b: 227,
    },
    ColorTemp {
        color_temp: 5700,
        r: 255,
        g: 239,
        b: 230,
    },
    ColorTemp {
        color_temp: 5800,
        r: 255,
        g: 240,
        b: 233,
    },
    ColorTemp {
        color_temp: 5900,
        r: 255,
        g: 242,
        b: 236,
    },
    ColorTemp {
        color_temp: 6000,
        r: 255,
        g: 243,
        b: 239,
    },
    ColorTemp {
        color_temp: 6100,
        r: 255,
        g: 244,
        b: 242,
    },
    ColorTemp {
        color_temp: 6200,
        r: 255,
        g: 245,
        b: 245,
    },
    ColorTemp {
        color_temp: 6300,
        r: 255,
        g: 246,
        b: 247,
    },
    ColorTemp {
        color_temp: 6400,
        r: 255,
        g: 248,
        b: 251,
    },
    ColorTemp {
        color_temp: 6500,
        r: 255,
        g: 249,
        b: 253,
    },
    ColorTemp {
        color_temp: 6600,
        r: 254,
        g: 249,
        b: 255,
    },
    ColorTemp {
        color_temp: 6700,
        r: 252,
        g: 247,
        b: 255,
    },
    ColorTemp {
        color_temp: 6800,
        r: 249,
        g: 246,
        b: 255,
    },
    ColorTemp {
        color_temp: 6900,
        r: 247,
        g: 245,
        b: 255,
    },
    ColorTemp {
        color_temp: 7000,
        r: 245,
        g: 243,
        b: 255,
    },
    ColorTemp {
        color_temp: 7100,
        r: 243,
        g: 242,
        b: 255,
    },
    ColorTemp {
        color_temp: 7200,
        r: 240,
        g: 241,
        b: 255,
    },
    ColorTemp {
        color_temp: 7300,
        r: 239,
        g: 240,
        b: 255,
    },
    ColorTemp {
        color_temp: 7400,
        r: 237,
        g: 239,
        b: 255,
    },
    ColorTemp {
        color_temp: 7500,
        r: 235,
        g: 238,
        b: 255,
    },
    ColorTemp {
        color_temp: 7600,
        r: 233,
        g: 237,
        b: 255,
    },
    ColorTemp {
        color_temp: 7700,
        r: 231,
        g: 236,
        b: 255,
    },
    ColorTemp {
        color_temp: 7800,
        r: 230,
        g: 235,
        b: 255,
    },
    ColorTemp {
        color_temp: 7900,
        r: 228,
        g: 234,
        b: 255,
    },
    ColorTemp {
        color_temp: 8000,
        r: 227,
        g: 233,
        b: 255,
    },
    ColorTemp {
        color_temp: 8100,
        r: 225,
        g: 232,
        b: 255,
    },
    ColorTemp {
        color_temp: 8200,
        r: 224,
        g: 231,
        b: 255,
    },
    ColorTemp {
        color_temp: 8300,
        r: 222,
        g: 230,
        b: 255,
    },
    ColorTemp {
        color_temp: 8400,
        r: 221,
        g: 230,
        b: 255,
    },
    ColorTemp {
        color_temp: 8500,
        r: 220,
        g: 229,
        b: 255,
    },
    ColorTemp {
        color_temp: 8600,
        r: 218,
        g: 229,
        b: 255,
    },
    ColorTemp {
        color_temp: 8700,
        r: 217,
        g: 227,
        b: 255,
    },
    ColorTemp {
        color_temp: 8800,
        r: 216,
        g: 227,
        b: 255,
    },
    ColorTemp {
        color_temp: 8900,
        r: 215,
        g: 226,
        b: 255,
    },
    ColorTemp {
        color_temp: 9000,
        r: 214,
        g: 225,
        b: 255,
    },
    ColorTemp {
        color_temp: 9100,
        r: 212,
        g: 225,
        b: 255,
    },
    ColorTemp {
        color_temp: 9200,
        r: 211,
        g: 224,
        b: 255,
    },
    ColorTemp {
        color_temp: 9300,
        r: 210,
        g: 223,
        b: 255,
    },
    ColorTemp {
        color_temp: 9400,
        r: 209,
        g: 223,
        b: 255,
    },
    ColorTemp {
        color_temp: 9500,
        r: 208,
        g: 222,
        b: 255,
    },
    ColorTemp {
        color_temp: 9600,
        r: 207,
        g: 221,
        b: 255,
    },
    ColorTemp {
        color_temp: 9700,
        r: 207,
        g: 221,
        b: 255,
    },
    ColorTemp {
        color_temp: 9800,
        r: 206,
        g: 220,
        b: 255,
    },
    ColorTemp {
        color_temp: 9900,
        r: 205,
        g: 220,
        b: 255,
    },
    ColorTemp {
        color_temp: 10000,
        r: 207,
        g: 218,
        b: 255,
    },
    ColorTemp {
        color_temp: 10100,
        r: 207,
        g: 218,
        b: 255,
    },
    ColorTemp {
        color_temp: 10200,
        r: 206,
        g: 217,
        b: 255,
    },
    ColorTemp {
        color_temp: 10300,
        r: 205,
        g: 217,
        b: 255,
    },
    ColorTemp {
        color_temp: 10400,
        r: 204,
        g: 216,
        b: 255,
    },
    ColorTemp {
        color_temp: 10500,
        r: 204,
        g: 216,
        b: 255,
    },
    ColorTemp {
        color_temp: 10600,
        r: 203,
        g: 215,
        b: 255,
    },
    ColorTemp {
        color_temp: 10700,
        r: 202,
        g: 215,
        b: 255,
    },
    ColorTemp {
        color_temp: 10800,
        r: 202,
        g: 214,
        b: 255,
    },
    ColorTemp {
        color_temp: 10900,
        r: 201,
        g: 214,
        b: 255,
    },
    ColorTemp {
        color_temp: 11000,
        r: 200,
        g: 213,
        b: 255,
    },
    ColorTemp {
        color_temp: 11100,
        r: 200,
        g: 213,
        b: 255,
    },
    ColorTemp {
        color_temp: 11200,
        r: 199,
        g: 212,
        b: 255,
    },
    ColorTemp {
        color_temp: 11300,
        r: 198,
        g: 212,
        b: 255,
    },
    ColorTemp {
        color_temp: 11400,
        r: 198,
        g: 212,
        b: 255,
    },
    ColorTemp {
        color_temp: 11500,
        r: 197,
        g: 211,
        b: 255,
    },
    ColorTemp {
        color_temp: 11600,
        r: 197,
        g: 211,
        b: 255,
    },
    ColorTemp {
        color_temp: 11700,
        r: 197,
        g: 210,
        b: 255,
    },
    ColorTemp {
        color_temp: 11800,
        r: 196,
        g: 210,
        b: 255,
    },
    ColorTemp {
        color_temp: 11900,
        r: 195,
        g: 210,
        b: 255,
    },
    ColorTemp {
        color_temp: 12000,
        r: 195,
        g: 209,
        b: 255,
    },
];

pub fn nearest_color(temp: u16) -> &'static ColorTemp {
    if let Some(color_temp) = KELVIN_TABLE
        .iter()
        .min_by_key(|ct| (ct.color_temp as i32 - temp as i32).abs())
    {
        return color_temp;
    }

    unreachable!();
}
