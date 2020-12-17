use std::fmt;

#[derive(Debug, Clone)]
pub struct Quad3d<T> {
    pub field: Vec<T>,
    xsize: usize,
    ysize: usize,
    zsize: usize,
}

impl<T: Default + Copy + std::ops::Add<Output = T>> Quad3d<T> {
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

    pub fn sum(&self) -> T {
        let mut sum = T::default();
        for v in &self.field {
            sum = sum + *v;
        }
        sum
    }
}

#[derive(Debug, Clone)]
pub struct InfiniteField3d<T> {
    pub quads: Vec<Quad3d<T>>,
    pub xlims: (i64, i64),
    pub ylims: (i64, i64),
    pub zlims: (i64, i64),
    curr: usize,
}

impl<
        T: Default
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::PartialEq
            + fmt::Debug,
    > InfiniteField3d<T>
{
    pub fn new() -> Self {
        let quads = vec![Quad3d::<T>::new(); 8];
        Self {
            quads,
            xlims: (0, 0),
            ylims: (0, 0),
            zlims: (0, 0),
            curr: 0,
        }
    }

    pub fn get(&self, x: isize, y: isize, z: isize) -> T {
        let mut idx = 0;
        if x < 0 {
            idx += 1;
        }
        if y < 0 {
            idx += 2;
        }
        if z < 0 {
            idx += 4;
        }
        self.quads[idx].get(x.abs() as usize, y.abs() as usize, z.abs() as usize)
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, v: T) {
        if x < self.xlims.0 as isize {
            self.xlims.0 = (x - 1) as i64;
        }
        if x > self.xlims.1 as isize {
            self.xlims.1 = (x + 1) as i64;
        }
        if y < self.ylims.0 as isize {
            self.ylims.0 = (y - 1) as i64;
        }
        if y > self.ylims.1 as isize {
            self.ylims.1 = (y + 1) as i64;
        }
        if z < self.zlims.0 as isize {
            self.zlims.0 = (z - 1) as i64;
        }
        if z > self.zlims.1 as isize {
            self.zlims.1 = (z + 1) as i64;
        }
        let mut idx = 0;
        if x < 0 {
            idx += 1;
        }
        if y < 0 {
            idx += 2;
        }
        if z < 0 {
            idx += 4;
        }
        self.quads[idx].set(x.abs() as usize, y.abs() as usize, z.abs() as usize, v);
    }

    pub fn num_neighbors(&self, x: isize, y: isize, z: isize) -> T {
        let mut sum = T::default();
        for xdir in -1..=1 {
            for ydir in -1..=1 {
                for zdir in -1..=1 {
                    if !(xdir == 0 && ydir == 0 && zdir == 0) {
                        sum = sum + self.get(x + xdir, y + ydir, z + zdir);
                    }
                }
            }
        }
        sum
    }

    pub fn sum(&self) -> T {
        let mut sum = T::default();
        for quad in &self.quads {
            sum = sum + quad.sum();
        }
        sum
    }
}

impl<
        T: Default
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::PartialEq
            + fmt::Debug,
    > fmt::Display for InfiniteField3d<T>
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for zlayer in self.zlims.0 - 1..=self.zlims.1 + 1 {
            s += &format!("Z layer {:?}:\n", zlayer);
            for y in self.ylims.0 - 1..=self.ylims.1 + 1 {
                for x in self.xlims.0 - 1..=self.xlims.1 + 1 {
                    if self.get(x as isize, y as isize, zlayer as isize) == T::default() {
                        if x == 0 && y == 0 && zlayer == 0 {
                            s += "+";
                        } else if x == 0 && y == 0 {
                            s += "*";
                        } else {
                            s += ".";
                        }
                    } else {
                        s += "#";
                    }
                }
                s += "\n";
            }
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::{cell::RefCell, collections::HashMap};

    use super::InfiniteField3d;

    #[test]
    fn test_case() {
        let grid = RefCell::new(InfiniteField3d::<i64>::new());
        let answers: RefCell<HashMap<(isize, isize, isize), i64>> = RefCell::new(HashMap::new());

        proptest!(|(x in -10isize..10, y in -10isize..10, z in -10isize..10, v in -42i64..42)| {
            grid.borrow_mut().set(x, y, z, v);

            prop_assert!(grid.borrow().xlims.0 <= x as i64);
            prop_assert!(grid.borrow().xlims.1 >= x as i64);
            prop_assert!(grid.borrow().ylims.0 <= y as i64);
            prop_assert!(grid.borrow().ylims.1 >= y as i64);
            prop_assert!(grid.borrow().zlims.0 <= z as i64);
            prop_assert!(grid.borrow().zlims.1 >= z as i64);

            let mut myanswers = answers.borrow_mut();
            myanswers.insert((x,y,z), v);
        });
    }
}
