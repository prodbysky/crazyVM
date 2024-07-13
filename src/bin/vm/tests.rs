use crate::data_structures::rom::Rom;

#[cfg(test)]
#[test]
fn ram_write() {
    use crate::data_structures::ram::Ram;

    let mut ram = Ram::new(1024);

    ram.write(69, 0).unwrap();
    ram.write(11, 1).unwrap();

    ram.write_many(&[4, 5], 2).unwrap();
    let mut expected = [0; 1024];
    expected[0] = 69;
    expected[1] = 11;
    expected[2] = 4;
    expected[3] = 5;

    assert_eq!(ram.get_data(), expected)
}

#[test]
#[should_panic]
fn raw_write_fail() {
    use crate::data_structures::ram::Ram;

    let mut ram = Ram::new(1024);
    ram.write(0, 1024).unwrap();
}

#[test]
#[should_panic]
fn raw_write_many_fail() {
    use crate::data_structures::ram::Ram;

    let mut ram = Ram::new(1024);
    ram.write_many(&[0, 1], 1024).unwrap();
}

#[test]
fn raw_read_fail() {
    use crate::data_structures::ram::Ram;

    let mut ram = Ram::new(1024);
    ram.write_many(&[0, 1], 0).unwrap();

    let single_read = ram.read(0).unwrap();
    let multiple_read = ram.read_many(0, 2).unwrap();

    assert_eq!(single_read, 0);
    assert_eq!(multiple_read, &[0, 1]);
}

#[test]
fn register_write() {
    use crate::data_structures::registers::{Register, Registers};
    let mut registers = Registers::new();

    registers[Register::A] = 1;

    assert_eq!(registers[Register::A], 1);
}

#[test]
fn rom_read() {
    let rom = Rom::new(Vec::from(&[1, 2, 3, 4]));
    let single_read = rom.read(0).unwrap();
    let multiple_read = rom.read_many(1, 3).unwrap();
    assert_eq!(single_read, 1);
    assert_eq!(multiple_read, &[2, 3, 4]);
}
