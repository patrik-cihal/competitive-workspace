#[macro_export]
macro_rules! minim {
    ($a: expr, $b: expr) => {
        if $b<$a {
            $a = $b; 
        }
    }
}

#[macro_export]
macro_rules! maxim {
    ($a: expr, $b: expr) => {
        if $b>$a {
            $a = $b; 
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn minim_test() {
        let mut a = 5;
        minim!(a, 3);
        assert_eq!(a, 3);
    }
    #[test]
    fn maxim_test() {
        let mut a = 3;
        let b = 100;
        maxim!(a, b);
        assert_eq!(a, 100);
    }

}