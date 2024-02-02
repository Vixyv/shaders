// Followed algorthim from https://www.researchgate.net/publication/216813608_Simplex_noise_demystified

use crate::prelude::*;

// The struct improves the speed by about 1.8x
pub struct PerlinNoise2D {
    grad: [[i8; 2]; 4], // 12 Gradients
    perm: [u32; 512],    // 512 Permutations
}

impl PerlinNoise2D {
    pub fn new() -> Self {
        // The permutation table is doubled to prevent the need for index wrapping
        Self {
            grad: [[0,1],[1,0],[0,-1],[-1,0]],
            perm: [151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,18,151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,18],
        }
    }

    pub fn noise(&self, x: f64, y: f64) -> f64 {
        // Find unit grid cell containing the point, get the releative xyz of that point within the cell
        let (unit_x, unit_y) = (fast_floor(x) as usize, fast_floor(y) as usize);
        let (x, y) = (x - unit_x as f64, y - unit_y as f64);
        // Wrap the integer cells at 255
        let (unit_x, unit_y) = (unit_x & 255, unit_y & 255);
    
        // TODO: Could makes these into loops
        // Calculate set of eight hashed gradient indices
        let gi00 = self.perm[unit_x+self.perm[unit_y as usize] as usize] % 4;
        let gi01 = self.perm[unit_x+self.perm[unit_y+1 as usize] as usize] % 4;
        let gi10 = self.perm[unit_x+1+self.perm[unit_y as usize] as usize] % 4;
        let gi11 = self.perm[unit_x+1+self.perm[unit_y+1 as usize] as usize] % 4;
    
        // Calculate noise contributions from each of the eight corners
        let n00 = Self::dot(self.grad[gi00 as usize], x, y);
        let n10 = Self::dot(self.grad[gi10 as usize], x-1.0, y);
        let n01 = Self::dot(self.grad[gi01 as usize], x, y-1.0);
        let n11 = Self::dot(self.grad[gi11 as usize], x-1.0, y-1.0);
    
        let (u, v) = (Self::fade(x), Self::fade(y));
    
        // Interpolate along x the contributions from each of the corners
        let nx0 = Self::mix(n00, n10, u);
        let nx1 = Self::mix(n01, n11, u);
    
        // Interpolate the two results along y
        let nxy = Self::mix(nx0, nx1, v);
        
        nxy
    }

    fn dot(grad: [i8; 2], x: f64, y: f64) -> f64 {
        (grad[0] as f64 * x) + (grad[1]  as f64 * y)
    }
    
    fn mix(a: f64, b: f64, t: f64) -> f64 {
        (1.0-t)*a + t*b
    }
    
    fn fade(t: f64) -> f64 {
        t*t*t*(t*(t*6.0-15.0)+10.0)
    }
}
