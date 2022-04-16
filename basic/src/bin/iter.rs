fn main() {
    let new_list = (1..=100)
        .rev()
        .map(|v| v * v)
        .filter(|v| v > &16)
        .take(12)
        .window(3)
        .map(|v| v.iter().fold(0, |s, v| s + v))
        .collect::<Vec<i32>>();
    println!("{:?}", new_list);
}

struct Window<I> {
    iter: I,
    win: usize,
}

impl<I> Window<I> {
    fn new(iter: I, win: usize) -> Window<I> {
        if win == 0 {
            panic!("window size is zero");
        }
        Window { iter, win }
    }
}

impl<I> Iterator for Window<I>
where
    I: Iterator,
{
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Vec::with_capacity(self.win);
        for _ in 0..self.win {
            if let Some(v) = self.iter.next() {
                result.push(v);
                continue;
            }
            break;
        }

        if result.len() == 0 {
            return None;
        }

        return Some(result);
    }
}

trait IteratorExt
where
    Self: Sized,
{
    fn window(self, win: usize) -> Window<Self>;
}

impl<T: Iterator> IteratorExt for T {
    fn window(self, win: usize) -> Window<Self> {
        Window::new(self, win)
    }
}
