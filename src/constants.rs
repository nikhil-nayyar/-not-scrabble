extern crate lazy_static;
use lazy_static::lazy_static;
use std::{collections::HashMap};

lazy_static!{
    pub static ref LETTER_COUNT: HashMap<char,u8> = {
        let mut m = HashMap::with_capacity(26);
        m.insert('A',9);
        m.insert('B',2);
        m.insert('C',2);
        m.insert('D',4);
        m.insert('E',12);
        m.insert('F',2);
        m.insert('G',3);
        m.insert('H',2);
        m.insert('I',9);
        m.insert('J',1);
        m.insert('K',1);
        m.insert('L',4);
        m.insert('M',2);
        m.insert('N',6);
        m.insert('O',8);
        m.insert('P',2);
        m.insert('Q',1);
        m.insert('R',6);
        m.insert('S',4);
        m.insert('T',6);
        m.insert('U',4);
        m.insert('V',2);
        m.insert('W',2);
        m.insert('X',1);
        m.insert('Y',2);
        m.insert('Z',1);
        m
    };
}

lazy_static!{
    pub static ref LETTER_VALUE: HashMap<char,u8> = {
        let mut m = HashMap::with_capacity(26);
        m.insert('A',1);
        m.insert('B',3);
        m.insert('C',3);
        m.insert('D',2);
        m.insert('E',1);
        m.insert('F',4);
        m.insert('G',2);
        m.insert('H',4);
        m.insert('I',1);
        m.insert('J',8);
        m.insert('K',5);
        m.insert('L',1);
        m.insert('M',3);
        m.insert('N',1);
        m.insert('O',1);
        m.insert('P',3);
        m.insert('Q',10);
        m.insert('R',1);
        m.insert('S',1);
        m.insert('T',1);
        m.insert('U',1);
        m.insert('V',4);
        m.insert('W',4);
        m.insert('X',8);
        m.insert('Y',4);
        m.insert('Z',10);
        m
    };
}