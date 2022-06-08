#[cfg(test)]
mod tests {
    use register::*;

    #[field(all, 0, 31)]
    #[field(first_byte, 0, 7)]
    #[field(fifth_bit, 5, 5)]
    struct BitField(*mut usize);
    
    #[test]
    fn all_bits() {
        let mut i: usize = 1000;
        let i_ptr: *mut usize = &mut i;

        let mut bf = BitField(i_ptr);

        let original = bf.all();
        assert_eq!(original, i);

        bf.set_all(1002);
        assert_ne!(original, i);
        assert_eq!(1002, i);
        assert_eq!(bf.all(), i);
    }

    #[test]
    fn parts() {
        let mut i: usize = 0xFFFF;

        let i_ptr: *mut usize = &mut i;

        let mut bf = BitField(i_ptr);

        let original = bf.all();
        let orig_first_byte = bf.first_byte();
        
        assert_eq!(original, i);
        assert_eq!(orig_first_byte, 0xFF);

        bf.set_first_byte(0xFA);
        assert_ne!(original, i);
        assert_ne!(orig_first_byte, bf.first_byte());
        assert_eq!(0xFFFA, i);
        assert_eq!(bf.first_byte(), 0xFA);
        assert_eq!(bf.all(), 0xFFFA);
    }


    #[test]
    fn single_bit() {
        let mut i: usize = 0;

        let i_ptr: *mut usize = &mut i;

        let mut bf = BitField(i_ptr);

        let original = bf.all();
        let orig_fifth_bit = bf.fifth_bit();
        
        assert_eq!(original, i);
        assert_eq!(orig_fifth_bit, 0);

        bf.set_fifth_bit(1);
        assert_ne!(original, i);
        assert_ne!(orig_fifth_bit, bf.fifth_bit());
        assert_eq!(1 << 5, i);
        assert_eq!(bf.fifth_bit(), 1);
        assert_eq!(bf.all(), 1 << 5);
    }


    #[field(all, 0, 31)]
    struct TestRegister(*mut usize);
    
    #[register(a, TestRegister, 0x0)]
    #[register(b, TestRegister, 0x8)]
    struct TestDevice(*mut usize);   

    #[test]
    fn describes_a_register() {
        
        let mut var = Box::new((1,2));
        let mut ptr: *mut usize = &mut var.0;
        let mut ptr1: *mut usize = &mut var.1;
 
        let device = TestDevice(ptr);

        assert_eq!(device.a().all(), 1);       
        assert_eq!(device.b().all(), 2);
        device.a().set_all(3);
        device.b().set_all(4);
        assert_eq!(*var, (3, 4));
    }
}

