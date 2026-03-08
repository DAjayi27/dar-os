
use core::fmt;

pub trait Write {
    fn new() -> Self;
    fn write(&mut self,string: &str);
    fn write_char(&mut self, char:char);
    fn clear_row(&mut self, row:usize );
    fn clear_screen(&mut self);
}

pub struct Writer<T:Write> {
    writer_driver: T,
}

impl <T: Write> Writer<T>{

    pub fn new() -> Self{
        return Self{
            writer_driver: T::new(),
        }
    }

    pub fn write(&mut self, string: &str){
        self.writer_driver.write(string);
    }

    pub fn write_char(&mut self, char: char) {
        self.writer_driver.write_char(char);
    }

}

impl <T: Write> fmt::Write for Writer<T>{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        return Ok(());
    }
}