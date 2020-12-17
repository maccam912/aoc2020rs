#![allow(clippy::many_single_char_names)]
use std::fmt;

#[derive(Debug, Clone)]
pub struct Quad4d<T> {
    pub field: Vec<T>,
    xsize: usize,
    ysize: usize,
    zsize: usize,
    wsize: usize,
}

impl<T: Default + Copy + std::ops::Add<Output = T>> Quad4d<T> {
    pub fn new() -> Self {
        Self {
            field: vec![],
            xsize: 0,
            ysize: 0,
            zsize: 0,
            wsize: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize, w: usize) -> T {
        if x >= self.xsize || y >= self.ysize || z >= self.zsize || w >= self.wsize {
            T::default()
        } else {
            // In bounds, get val
            let idx = w * (self.zsize * self.ysize * self.xsize)
                + z * (self.xsize * self.ysize)
                + y * self.xsize
                + x;
            self.field[idx]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, w: usize, val: T) {
        // Expand in w direction
        if w >= self.wsize {
            let newsize = self.xsize * self.ysize * self.zsize * (w + 1);
            let oldsize = self.xsize * self.ysize * self.zsize * self.wsize;
            let diff = newsize - oldsize;
            self.field.extend(vec![T::default(); diff]);
            self.wsize = w + 1;
        }
        // Expand in z direction
        if z >= self.zsize {
            let newsize = self.xsize * self.ysize * (z + 1) * self.wsize;
            let mut newfield = vec![T::default(); newsize];
            for w_ in 0..self.wsize {
                for z_ in 0..self.zsize {
                    for y_ in 0..self.ysize {
                        for x_ in 0..self.xsize {
                            newfield[w_ * ((z + 1) * self.ysize * self.xsize)
                                + z_ * (self.ysize * self.xsize)
                                + y_ * self.xsize
                                + x_] = self.get(x_, y_, z_, w_);
                        }
                    }
                }
            }
            self.field = newfield;
            self.zsize = z + 1;
        }
        // Expand in y direction
        if y >= self.ysize {
            let newsize = self.xsize * (y + 1) * self.zsize * self.wsize;
            let mut newfield = vec![T::default(); newsize];
            for w_ in 0..self.wsize {
                for z_ in 0..self.zsize {
                    for y_ in 0..self.ysize {
                        for x_ in 0..self.xsize {
                            newfield[w_ * (self.zsize * (y + 1) * self.xsize)
                                + z_ * ((y + 1) * self.xsize)
                                + y_ * self.xsize
                                + x_] = self.get(x_, y_, z_, w_);
                        }
                    }
                }
            }
            self.field = newfield;
            self.ysize = y + 1;
        }
        // Expand in x direction
        if x >= self.xsize {
            let newsize = (x + 1) * self.ysize * self.zsize * self.wsize;
            let mut newfield = vec![T::default(); newsize];
            for w_ in 0..self.wsize {
                for z_ in 0..self.zsize {
                    for y_ in 0..self.ysize {
                        for x_ in 0..self.xsize {
                            newfield[w_ * (self.zsize * self.ysize * (x + 1))
                                + z_ * (self.ysize * (x + 1))
                                + y_ * (x + 1)
                                + x_] = self.get(x_, y_, z_, w_);
                        }
                    }
                }
            }
            self.field = newfield;
            self.xsize = x + 1;
        }

        self.field[w * self.zsize * self.ysize * self.xsize
            + z * self.ysize * self.xsize
            + y * self.xsize
            + x] = val;
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
pub struct InfiniteField4d<T> {
    pub quads: Vec<Quad4d<T>>,
    pub xlims: (i64, i64),
    pub ylims: (i64, i64),
    pub zlims: (i64, i64),
    pub wlims: (i64, i64),
}

impl<
        T: Default
            + Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::cmp::PartialEq
            + fmt::Debug,
    > InfiniteField4d<T>
{
    pub fn new() -> Self {
        let quads = vec![Quad4d::<T>::new(); 16];
        Self {
            quads,
            xlims: (0, 0),
            ylims: (0, 0),
            zlims: (0, 0),
            wlims: (0, 0),
        }
    }

    pub fn get(&self, x: isize, y: isize, z: isize, w: isize) -> T {
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
        if w < 0 {
            idx += 8;
        }
        self.quads[idx].get(
            x.abs() as usize,
            y.abs() as usize,
            z.abs() as usize,
            w.abs() as usize,
        )
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, w: isize, v: T) {
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
        if w < self.wlims.0 as isize {
            self.wlims.0 = (w - 1) as i64;
        }
        if w > self.wlims.1 as isize {
            self.wlims.1 = (w + 1) as i64;
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
        if w < 0 {
            idx += 8;
        }
        self.quads[idx].set(
            x.abs() as usize,
            y.abs() as usize,
            z.abs() as usize,
            w.abs() as usize,
            v,
        );
    }

    pub fn num_neighbors(&self, x: isize, y: isize, z: isize, w: isize) -> T {
        let mut sum = T::default();
        for xdir in -1..=1 {
            for ydir in -1..=1 {
                for zdir in -1..=1 {
                    for wdir in -1..=1 {
                        if !(xdir == 0 && ydir == 0 && zdir == 0 && wdir == 0) {
                            sum = sum + self.get(x + xdir, y + ydir, z + zdir, w + wdir);
                        }
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
    > fmt::Display for InfiniteField4d<T>
{
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for wlayer in self.wlims.0 - 1..=self.wlims.1 + 1 {
            s += &format!("W layer {:?}:\n", wlayer);
            for zlayer in self.zlims.0 - 1..=self.zlims.1 + 1 {
                s += &format!("Z layer {:?}:\n", zlayer);
                for y in self.ylims.0 - 1..=self.ylims.1 + 1 {
                    for x in self.xlims.0 - 1..=self.xlims.1 + 1 {
                        if self.get(x as isize, y as isize, zlayer as isize, wlayer as isize)
                            == T::default()
                        {
                            if x == 0 && y == 0 && zlayer == 0 && wlayer == 0 {
                                s += "+";
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
        }
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::{cell::RefCell, collections::HashMap};

    use super::InfiniteField4d;

    #[test]
    fn test_case() {
        let grid = RefCell::new(InfiniteField4d::<i8>::new());
        let answers: RefCell<HashMap<(isize, isize, isize, isize), i8>> =
            RefCell::new(HashMap::new());

        proptest!(|(x in -100isize..100, y in -100isize..100, z in -100isize..100, w in -100isize..100, v in -8i8..8)| {
            grid.borrow_mut().set(x, y, z, w, v);

            prop_assert!(grid.borrow().xlims.0 <= x as i64);
            prop_assert!(grid.borrow().xlims.1 >= x as i64);
            prop_assert!(grid.borrow().ylims.0 <= y as i64);
            prop_assert!(grid.borrow().ylims.1 >= y as i64);
            prop_assert!(grid.borrow().zlims.0 <= z as i64);
            prop_assert!(grid.borrow().zlims.1 >= z as i64);
            prop_assert!(grid.borrow().wlims.0 <= w as i64);
            prop_assert!(grid.borrow().wlims.1 >= w as i64);

            let mut myanswers = answers.borrow_mut();
            myanswers.insert((x,y,z,w), v);
        });
    }
}
