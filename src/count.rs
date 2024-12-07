pub trait Count {
    fn digits(&self) -> u32;
}

impl Count for i64 {
    fn digits(&self) -> u32 {
        let mut number = *self;
        let mut p = 0;

        while number != 0 {
            number /= 10;
            p += 1;
        }

        p
    }
}
