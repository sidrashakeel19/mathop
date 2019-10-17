pub mod simp_op {
    mod sum {
        pub fn isum(iin1: i32, iin2: i32) -> i32 {
            iin1 + iin2
        } 

        pub fn fsum(fin1: f32, fin2: f32) -> f32 {
            fin1 + fin2
        }
    }

    mod diff {
        pub fn idiff(iin1: i32, iin2: i32 ) -> i32 {
            iin1 - iin2
        }
        pub fn fdiff(fin1: f32, fin2: f32) -> f32 {
            fin1 - fin2
        }
    }

    mod multiply {
        pub fn imul(iin1: i32, iin2: i32) -> i32 {
            iin1 * iin2
        }
        pub fn fmul(fin1: f32, fin2: f32) -> f32 {
            fin1 * fin2
        }
    }

    mod divide {
        pub fn idiv(iin1: i32, iin2: i32) -> i32 {
            iin1/iin2
        }
        pub fn fdiv(fin1: f32, fin2: f32) -> f32 {
            fin1/fin2
        }
    }
}