use std::io::{Read, Result};

use std::io::{BufWriter, Write};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }

    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let mut writer = MyWriter::new("127.0.0.1:8080");
    writer.write("hello world!");
}

/// 可以在不同的实现下逐步添加约束；
/// `new`方法不对泛型参数进行约束；
/// 只对`Reader<R:Read>`实现`process`方法；
struct Reader<R> {
    reader: R,
    buf: String,
}

impl<R> Reader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

impl<R> Reader<R>
where
    R: Read,
{
    fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

trait Api {
    type O: Default + Sized;
    fn do_something();
}

struct G<T: Api<O = i32>> {
    v: T,
}

impl<T: Api<O = i32>> G<T> {
    fn do_something() -> <T as Api>::O {
        // <T as Api> 对泛行类型做强制类型转换
        <T as Api>::O::default()
    }
}
