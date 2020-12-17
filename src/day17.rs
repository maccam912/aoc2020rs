#[derive(Clone)]
struct Quad3d<T> {
    field: Vec<T>,
    xsize: usize,
    ysize: usize,
    zsize: usize,
}

impl<T: Default + Copy> Quad3d<T> {
    pub fn new() -> Self {
        Self {
            field: vec![],
            xsize: 0,
            ysize: 0,
            zsize: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> T {
        if x >= self.xsize || y >= self.ysize || z >= self.zsize {
            T::default()
        } else {
            // In bounds, get val
            let idx = z * (self.xsize * self.ysize) + y * self.xsize + x;
            self.field[idx]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: T) {
        // Expand in Z direction
        if z >= self.zsize {
            let newsize = self.xsize * self.ysize * (z + 1);
            let oldsize = self.xsize * self.ysize * self.zsize;
            let diff = newsize - oldsize;
            self.field.extend(vec![T::default(); diff]);
            self.zsize = z + 1;
        }
        // Expand in y direction
        if y >= self.ysize {
            let newsize = self.xsize * (y + 1) * self.zsize;
            let mut newfield = vec![T::default(); newsize];
            for z_ in 0..self.zsize {
                for y_ in 0..self.ysize {
                    for x_ in 0..self.xsize {
                        newfield[z_ * ((y + 1) * self.xsize) + y_ * self.xsize + x_] =
                            self.get(x_, y_, z_);
                    }
                }
            }
            self.field = newfield;
            self.ysize = y + 1;
        }
        // Expand in x direction
        if x >= self.xsize {
            let newsize = (x + 1) * self.ysize * self.zsize;
            let mut newfield = vec![T::default(); newsize];
            for z_ in 0..self.zsize {
                for y_ in 0..self.ysize {
                    for x_ in 0..self.xsize {
                        newfield[z_ * (self.ysize * (x + 1)) + y_ * (x + 1) + x_] =
                            self.get(x_, y_, z_);
                    }
                }
            }
            self.field = newfield;
            self.xsize = x + 1;
        }
        self.field[z * self.ysize * self.xsize + y * self.xsize + x] = val;
    }
}

struct InfiniteField3d<T> {
    quads: Vec<Quad3d<T>>,
}

impl<T: Default + Copy> InfiniteField3d<T> {
    pub fn new() -> Self {
        let quads = vec![Quad3d::<T>::new(); 8];
        Self { quads }
    }

    pub fn get(&self, x: isize, y: isize, z: isize) -> T {
        let mut idx = 0;
        if x < 0 {
            idx += 1;
        }
        if y < 0 {
            idx += 2;
        }
        if y < 0 {
            idx += 4;
        }
        self.quads[idx].get(x.abs() as usize, y.abs() as usize, z.abs() as usize)
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, v: T) {
        let mut idx = 0;
        if x < 0 {
            idx += 1;
        }
        if y < 0 {
            idx += 2;
        }
        if y < 0 {
            idx += 4;
        }
        self.quads[idx].set(x.abs() as usize, y.abs() as usize, z.abs() as usize, v);
    }
}

#[cfg(test)]
mod tests {
    use crate::day17;
    use proptest::prelude::*;

    #[test]
    fn test_case() {
        assert_eq!(1, 1);
    }

    proptest! {
        #[test]
        fn test_we_get_what_we_set_with_negs(x in -100isize..100,
                                             y in -100isize..100,
                                             z in -100isize..100,
                                             x2 in -100isize..100,
                                             y2 in -100isize..100,
                                             z2 in -100isize..100,
                                             v in 0i64..10000) {

            let mut grid = day17::InfiniteField3d::<i64>::new();
            grid.set(x,y,z,v);
            prop_assert_eq!(grid.get(x,y,z), v);
            grid.set(x2,y2,z2,v+1);
            prop_assert_eq!(grid.get(x2,y2,z2), v+1);
            prop_assert_eq!(grid.get(x,y,z), v);
        }
    }
}
