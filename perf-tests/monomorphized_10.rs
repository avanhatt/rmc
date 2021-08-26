// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

trait A {
    fn f(&self, x: i32) -> i32;
}

struct Struct1 {}

impl A for Struct1 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct2 {}

impl A for Struct2 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

struct Struct3 {}

impl A for Struct3 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct4 {}

impl A for Struct4 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

struct Struct5 {}

impl A for Struct5 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct6 {}

impl A for Struct6 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

struct Struct7 {}

impl A for Struct7 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct8 {}

impl A for Struct8 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

struct Struct9 {}

impl A for Struct9 {
    fn f(&self, x: i32) -> i32 {
        if x == 9999 {
            return -1;
        }
        return 1;
    }
}

struct Struct10 {}

impl A for Struct10 {
    fn f(&self, x: i32) -> i32 {
        if x == 9998 {
            return -1;
        }
        return 1;
    }
}

fn main() {
    for i in 0..10000 {
        if i % 10 == 1 {
            let s1 = Struct1 {};
            assert!(s1.f(i) == 1);
        } else if i % 10 == 2 {
            let s2 = Struct2 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 3 {
            let s2 = Struct3 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 4 {
            let s2 = Struct4 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 5 {
            let s2 = Struct5 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 6 {
            let s2 = Struct6 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 7 {
            let s2 = Struct7 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 8 {
            let s2 = Struct8 {};
            assert!(s2.f(i) == 1);
        } else if i % 10 == 9 {
            let s2 = Struct9 {};
            assert!(s2.f(i) == 1);
        } else {
            let s2 = Struct10 {};
            assert!(s2.f(i) == 1);
        }
    }
}
