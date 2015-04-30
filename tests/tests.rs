#![feature(plugin)]
#![plugin(speculate)]

extern crate diff;
extern crate quickcheck;

pub fn undiff<T: Clone>(diff: &[::diff::Result<&T>]) -> (Vec<T>, Vec<T>) {
    let (mut left, mut right) = (vec![], vec![]);
    for d in diff {
        match d {
            &::diff::Result::Left(l) => left.push(l.clone()),
            &::diff::Result::Both(l, r) => {
                left.push(l.clone());
                right.push(r.clone());
            },
            &::diff::Result::Right(r) => right.push(r.clone()),
        }
    }
    (left, right)
}

pub fn undiff_str<'a>(diff: &[::diff::Result<&'a str>])
                      -> (Vec<&'a str>, Vec<&'a str>) {
    let (mut left, mut right) = (vec![], vec![]);
    for d in diff {
        match d {
            &::diff::Result::Left(l) => left.push(l.clone()),
            &::diff::Result::Both(l, r) => {
                left.push(l.clone());
                right.push(r.clone());
            },
            &::diff::Result::Right(r) => right.push(r.clone()),
        }
    }
    (left, right)
}

speculate! {
    describe "slice" {
        before {
            fn go<T>(left: &[T], right: &[T], len: usize) where
                T: Clone + ::std::fmt::Debug + PartialEq
            {
                let diff = ::diff::slice(&left, &right);
                assert_eq!(diff.len(), len);
                let (left_, right_) = undiff(&diff);
                assert_eq!(left, &left_[..]);
                assert_eq!(right, &right_[..]);
            }
        }

        test "empty slices" {
            let slice: &[()] = &[];
            go(&slice, &slice, 0);
        }

        test "equal + non-empty slices" {
            let slice = [1, 2, 3];
            go(&slice, &slice, 3);
        }

        test "left empty, right non-empty" {
            let slice = [1, 2, 3];
            go(&slice, &[], 3);
        }

        test "left non-empty, right empty" {
            let slice = [1, 2, 3];
            go(&[], &slice, 3);
        }

        test "misc 1" {
            let left = [1, 2, 3, 4, 1, 3];
            let right = [1, 4, 1, 1];
            go(&left, &right, 7);
        }

        test "misc 2" {
            let left = [1, 2, 1, 2, 3, 2, 2, 3, 1, 3];
            let right = [3, 3, 1, 2, 3, 1, 2, 3, 4, 1];
            go(&left, &right, 14);
        }

        test "quickcheck" {
            fn prop(left: Vec<i32>, right: Vec<i32>) -> bool {
                let diff = ::diff::slice(&left, &right);
                let (left_, right_) = undiff(&diff);
                left == &left_[..] && right == &right_[..]
            }

            ::quickcheck::quickcheck(prop as fn(Vec<i32>, Vec<i32>) -> bool);
        }
    }

    describe "lines" {
        before {
            fn go(left: &str, right: &str, len: usize) {
                let diff = ::diff::lines(&left, &right);
                assert_eq!(diff.len(), len);
                let (left_, right_) = undiff_str(&diff);
                assert_eq!(left, left_.connect("\n"));
                assert_eq!(right, right_.connect("\n"));
            }
        }

        test "both empty" {
            go("", "", 0);
        }

        test "one empty" {
            go("foo", "", 1);
            go("", "foo", 1);
        }

        test "both equal and non-empty" {
            go("foo\nbar", "foo\nbar", 2);
        }

        test "misc 1" {
            go("foo\nbar\nbaz", "foo\nbaz\nquux", 4);
        }
    }
}
