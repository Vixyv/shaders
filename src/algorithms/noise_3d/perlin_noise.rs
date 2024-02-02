// Followed algorthim from https://www.researchgate.net/publication/216813608_Simplex_noise_demystified

use crate::prelude::*;

// The struct improves the speed by about 1.8x
pub struct PerlinNoise3D {
    grad: [[i8; 3]; 12], // 12 Gradients
    perm: [u32; 512],    // 512 Permutations
}

impl PerlinNoise3D {
    pub fn new() -> Self {
        // The permutation table is doubled to prevent the need for index wrapping
        Self {
            grad: [[1,1,0],[-1,1,0],[1,-1,0],[-1,-1,0],[1,0,1],[-1,0,1],[1,0,-1],[-1,0,-1],[0,1,1],[0,-1,1],[0,1,-1],[0,-1,-1]],
            perm: [151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,18,151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190,6,148,247,120,234,75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,143,54,65,25,63,161,1,216,80,73,209,76,132,187,208,89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,173,186,3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152,2,44,154,163,70,221,153,101,155,167,43,172,9,129,22,39,253,19,98,108,110,79,113,224,232,178,185,112,104,218,246,97,228,251,34,242,193,238,210,144,12,191,179,162,241,81,51,145,235,249,14,239,107,49,192,214,31,181,199,106,157,184,84,204,176,115,121,50,45,127,4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,18],
        }
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        // Find unit grid cell containing the point, get the releative xyz of that point within the cell
        let (unit_x, unit_y, unit_z) = (fast_floor(x) as usize, fast_floor(y) as usize, fast_floor(z) as usize);
        let (x, y, z) = (x - unit_x as f64, y - unit_y as f64, z - unit_z as f64);
        // Wrap the integer cells at 255
        let (unit_x, unit_y, unit_z) = (unit_x & 255, unit_y & 255, unit_z & 255);
    
        // TODO: Could makes these into loops
        // Calculate set of eight hashed gradient indices
        let gi000 = self.perm[unit_x+self.perm[unit_y+self.perm[unit_z] as usize] as usize] % 12;
        let gi001 = self.perm[unit_x+self.perm[unit_y+self.perm[unit_z+1] as usize] as usize] % 12;
        let gi010 = self.perm[unit_x+self.perm[unit_y+1+self.perm[unit_z] as usize] as usize] % 12;
        let gi011 = self.perm[unit_x+self.perm[unit_y+1+self.perm[unit_z+1] as usize] as usize] % 12;
        let gi100 = self.perm[unit_x+1+self.perm[unit_y+self.perm[unit_z] as usize] as usize] % 12;
        let gi101 = self.perm[unit_x+1+self.perm[unit_y+self.perm[unit_z+1] as usize] as usize] % 12;
        let gi110 = self.perm[unit_x+1+self.perm[unit_y+1+self.perm[unit_z] as usize] as usize] % 12;
        let gi111 = self.perm[unit_x+1+self.perm[unit_y+1+self.perm[unit_z+1] as usize] as usize] % 12;
    
        // Calculate noise contributions from each of the eight corners
        let n000 = Self::dot(self.grad[gi000 as usize], x, y, z);
        let n100 = Self::dot(self.grad[gi100 as usize], x-1.0, y, z);
        let n010 = Self::dot(self.grad[gi010 as usize], x, y-1.0, z);
        let n110 = Self::dot(self.grad[gi110 as usize], x-1.0, y-1.0, z);
        let n001 = Self::dot(self.grad[gi001 as usize], x, y, z-1.0);
        let n101 = Self::dot(self.grad[gi101 as usize], x-1.0, y, z-1.0);
        let n011 = Self::dot(self.grad[gi011 as usize], x, y-1.0, z-1.0);
        let n111 = Self::dot(self.grad[gi111 as usize], x-1.0, y-1.0, z-1.0);
    
        let (u, v, w) = (Self::fade(x), Self::fade(y), Self::fade(z));
    
        // Interpolate along x the contributions from each of the corners
        let nx00 = Self::mix(n000, n100, u);
        let nx01 = Self::mix(n001, n101, u);
        let nx10 = Self::mix(n010, n110, u);
        let nx11 = Self::mix(n011, n111, u);
    
        // Interpolate the four results along y
        let nxy0 = Self::mix(nx00, nx10, v);
        let nxy1 = Self::mix(nx01, nx11, v);
    
        // Interpolate the two last results along z
        let nxyz = Self::mix(nxy0, nxy1, w);
        
        nxyz
    }

    fn dot(grad: [i8; 3], x: f64, y: f64, z: f64) -> f64 {
        (grad[0] as f64 * x) + (grad[1]  as f64 * y) + (grad[2] as f64 * z)
    }
    
    fn mix(a: f64, b: f64, t: f64) -> f64 {
        (1.0-t)*a + t*b
    }
    
    fn fade(t: f64) -> f64 {
        t*t*t*(t*(t*6.0-15.0)+10.0)
    }
}
