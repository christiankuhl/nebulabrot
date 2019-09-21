use num::Complex;

fn in_mandelbrot_set(c: &Complex<f64>) -> bool {
   (c.re >  -1.2 && c.re <=  -1.1 && c.im >  -0.1 && c.im < 0.1)
    || (c.re >  -1.1 && c.re <=  -0.9 && c.im >  -0.2 && c.im < 0.2)
    || (c.re >  -0.9 && c.re <=  -0.8 && c.im >  -0.1 && c.im < 0.1)
    || (c.re > -0.69 && c.re <= -0.61 && c.im >  -0.2 && c.im < 0.2)
    || (c.re > -0.61 && c.re <=  -0.5 && c.im > -0.37 && c.im < 0.37)
    || (c.re >  -0.5 && c.re <= -0.39 && c.im > -0.48 && c.im < 0.48)
    || (c.re > -0.39 && c.re <=  0.14 && c.im > -0.55 && c.im < 0.55)
    || (c.re >  0.14 && c.re <   0.29 && c.im > -0.42 && c.im < -0.07)
    || (c.re >  0.14 && c.re <   0.29 && c.im >  0.07 && c.im < 0.42)
}


pub struct PlotRange {
    pub top_left: Complex<f64>,
    pub bottom_right: Complex<f64>,
    pub buffer: Box<Vec<u32>>,
    pub output_height: u32,
    pub output_width: u32
}

impl PlotRange {
    pub fn renormalize(&mut self, colour_function: fn(usize, &u32, &[u32; 3]) -> u8) -> Box<Vec<u8>> {
        let pixels = (self.output_width * self.output_height) as usize;
        let mut result = Box::new(Vec::with_capacity(3*pixels));
        for _ in 0..3*pixels {
            result.push(0);
        }
        let mut channel_maxima: [u32; 3] = [0, 0, 0];
        for channel in 0..3 {
            let max = self.buffer.iter().enumerate().max_by_key(|&(i, v)| if i % 3 == channel {*v} else {0}).unwrap();
            channel_maxima[channel] = *max.1;
        }
        println!("{:?}", channel_maxima);
        for (index, val) in self.buffer.iter().enumerate() {
            result[index] = colour_function(index % 3, val, &channel_maxima);
        }
        result
    }
    fn index_to_point(&self, index: &usize) -> Complex<f64> {
        Complex {re: ((*index % (self.output_width as usize)) as f64) / ((self.output_width - 1) as f64)
                        * self.width() + self.top_left.re,
                 im: self.top_left.im -(((*index / (self.output_width as usize)) as f64).ceil()) / ((self.output_height - 1) as f64)
                                                                                    * self.height()}
    }
    fn point_to_index(&self, point: &Complex<f64>) -> Option<usize> {
        let index_f: f64 = ((self.top_left.im - point.im) / self.height() * ((self.output_height - 1) as f64)).floor() * (self.output_width as f64)
                                    + (point.re - self.top_left.re) / self.width() * ((self.output_width - 1) as f64);
        if index_f < 0.0 || index_f >= (self.output_width * self.output_height) as f64 {
            return None
        } else {
            return Some(index_f as usize)
        }
    }
    fn height(&self) -> f64 {
        self.top_left.im - self.bottom_right.im
    }
    fn width(&self) -> f64 {
        self.bottom_right.re - self.top_left.re
    }
    pub fn iterate(&mut self, max_iterations: [usize; 3]) {
        let pixels = (self.output_width * self.output_height) as usize;
        for _ in 0..3*pixels {
            self.buffer.push(0);
        }
        let iteration_limit = max_iterations.iter().max().unwrap();
        println!("Calculating {} iterations...", iteration_limit);
        for index in 0..pixels {
            if index % 50000 == 0 {
                println!("{:.2}% complete...", 100.0 * (index as f32) / (pixels as f32));
            }
            let c = self.index_to_point(&(index as usize));
            if in_mandelbrot_set(&c) { continue }
            let mut z = Complex {re: 0.0, im: 0.0};
            let mut tr: Vec<usize> = Vec::with_capacity(*iteration_limit);
            for iter_count in 0..*iteration_limit {
                z = z*z + c;
                if let Some(idx) = self.point_to_index(&z) {tr.push(idx)};
                if z.norm_sqr() > 4.0 {
                    for idx in tr.iter() {
                        for (channel, iterations) in max_iterations.iter().enumerate() {
                            if *iterations >= iter_count {
                                self.buffer[3 * idx + channel] += 1;
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}
