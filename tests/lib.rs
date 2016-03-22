extern crate zero85;
extern crate quickcheck;

use zero85::*;

use quickcheck::quickcheck;
use quickcheck::TestResult;


#[test]
fn decode() {
    let t = "HelloWorld".from_z85().unwrap();
    let r = vec![0x86, 0x4F, 0xD2, 0x6F,
               0xB5, 0x59, 0xF7, 0x5B];
    assert_eq!(r, t);
}

#[test]
fn encode() {
    let t1 = &[0x86, 0x4F, 0xD2, 0x6F,
               0xB5, 0x59, 0xF7, 0x5B];
    let r = t1.to_z85().unwrap();
    assert_eq!(r, "HelloWorld");
}

#[test]
fn encode_quickcheck() {
    fn prop(xs: Vec<u8>) -> TestResult {
        let len = xs.len();
        if len == 0 || len % 4 != 0 {
            return TestResult::discard()
        }
        let r = xs.to_z85().unwrap();
        let newxs = r.from_z85().unwrap();
        TestResult::from_bool(xs == newxs)
    }
    quickcheck(prop as fn(Vec<u8>) -> TestResult);
}
