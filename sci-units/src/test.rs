#[macro_export]
#[cfg(feature = "std")]
#[cfg(test)]
macro_rules! basic {
    ($name: ident, $type_name:ident) => {
        #[test]
        fn $name() {
            let v0 = $type_name::from(5.0 as NativeType);
            println!("v0 = {}", v0);
            let mut v1 = v0;
            assert_eq!(v1, v0);
            v1 = $type_name::from(10.0 as NativeType);
            assert!(v0 < v1);
            assert!(v1 > v0);
            assert!(v0 <= v1);
            assert!(v1 >= v0);
            let mut v2 = v0.clone();
            assert_eq!(v0, v2);
            v2 = $type_name::from(20.0 as NativeType);
            let v3 = v1 + v2;
            let v4 = (v3 - v0) * crate::Scalar::from(40.0 as NativeType);
            assert_ne!(v0, v1);
            assert_eq!(v3, $type_name::from(30.0 as NativeType));
            let v5 = v1 / v2;
            assert_eq!(TypeId::of::<crate::Scalar>(), v5.type_id());
            println!("[{},{},{},{},{}]", v0, v1, v2, v3, v4);
        }
    };
}

#[macro_export]
#[cfg(feature = "std")]
#[cfg(test)]
macro_rules! invert {
    ($name: ident, $type_name:ident, $inverted_type_name: ident) => {
        #[test]
        fn $name() {
            let v0 = $type_name::from(5.0 as NativeType);
            println!("v0 = {}", v0);
            let v1 = Scalar::from(1.0 as NativeType) / v0;
            assert_eq!(TypeId::of::<$inverted_type_name>(), v1.type_id());
            let v2 = Scalar::from(1.0 as NativeType) / v1;
            assert_eq!(TypeId::of::<$type_name>(), v2.type_id());
        }
    };
}

#[macro_export]
#[cfg(feature = "std")]
#[cfg(test)]
macro_rules! divide {
    ($name: ident, $type_name:ident, $lhs: ident, $rhs: ident) => {
        #[test]
        fn $name() {
            let lhs = $lhs::from(100.0 as NativeType);
            let rhs = $rhs::from(10.0 as NativeType);
            let v0 = lhs / rhs;
            assert_eq!(v0, $type_name::from(10.0 as NativeType));
            let v1 = rhs * v0;
            assert_eq!(v1, $lhs::from(100.0 as NativeType));
            let v2 = v0 * rhs;
            assert_eq!(v2, $lhs::from(100.0 as NativeType));
        }
    };
}

#[macro_export]
#[cfg(feature = "std")]
#[cfg(test)]
macro_rules! multiply {
    ($name: ident, $type_name:ident, $lhs: ident, $rhs: ident) => {
        #[test]
        fn $name() {
            let lhs = $lhs::from(100.0 as NativeType);
            let rhs = $rhs::from(10.0 as NativeType);
            let v0 = lhs * rhs;
            assert_eq!(v0, $type_name::from(1000.0 as NativeType));
            let v1 = v0 / rhs;
            assert_eq!(v1, $lhs::from(100.0 as NativeType));
            let v2 = v0 / lhs;
            assert_eq!(v2, $rhs::from(10.0 as NativeType));
        }
    };
}
