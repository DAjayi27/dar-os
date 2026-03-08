
pub trait Write {
    fn new() -> Self;
    fn write(&mut self);
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
    
    pub fn write(&mut self){
        self.writer_driver.write_char('a');
    }

    pub fn write_char(&mut self, char: char) {
        self.writer_driver.write_char(char);
    }

}