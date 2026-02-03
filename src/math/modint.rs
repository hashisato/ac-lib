#[derive(Clone)]
pub struct ModInt {
    value: i64,
    modulus: i64,
}

impl ModInt {
    pub fn new(value: i64, modulus: i64) -> Self {
        let value = ((value % modulus) + modulus) % modulus;
        ModInt { value, modulus }
    }

    pub fn add(&self, other: &ModInt) -> ModInt {
        assert_eq!(self.modulus, other.modulus, "Moduli must be the same");
        ModInt::new(self.value + other.value, self.modulus)
    }

    pub fn sub(&self, other: &ModInt) -> ModInt {
        assert_eq!(self.modulus, other.modulus, "Moduli must be the same");
        ModInt::new(self.value - other.value, self.modulus)
    }

    pub fn mul(&self, other: &ModInt) -> ModInt {
        assert_eq!(self.modulus, other.modulus, "Moduli must be the same");
        ModInt::new(self.value * other.value, self.modulus)
    }

    pub fn pow(&self, exponent: i64) -> ModInt {
        let mut result = ModInt::new(1, self.modulus);
        let mut base = self.clone();
        let mut exp = exponent;

        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }

        result
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
