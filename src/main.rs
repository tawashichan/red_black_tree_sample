#![feature(box_syntax, box_patterns)]

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
enum RBT<T: PartialEq + PartialOrd + Clone> {
    Node(Color, Box<RBT<T>>, T, Box<RBT<T>>),
    Leaf,
}

impl<T: PartialEq + PartialOrd + Clone> RBT<T> {
    fn is_empty(&self) -> bool {
        use RBT::Leaf;
        match self {
            Leaf => true,
            _ => false,
        }
    }

    pub fn search(&self, x: T) -> Option<T> {
        use RBT::{Leaf, Node};
        match self {
            Leaf => None,
            Node(_, box left, data, box right) => match x {
                x if &x > data => right.search(x),
                x if &x < data => left.search(x),
                _ => Some(x),
            },
        }
    }

    fn balance(self) -> RBT<T> {
        use Color::{Black, Red};
        use RBT::Node;
        match self {
            Node(Black, box Node(Red, box Node(Red, lll, llx, llr), lx, lr), x, r) => Node(
                Red,
                box Node(Black, lll, llx, llr),
                lx,
                box Node(Black, lr, x, r),
            ),
            Node(Black, box Node(Red, ll, lx, box Node(Red, lrl, lrx, lrr)), x, r) => Node(
                Red,
                box Node(Black, ll, lx, lrl),
                lrx,
                box Node(Black, lrr, x, r),
            ),
            Node(Black, l, x, box Node(Red, box Node(Red, rll, rlx, rlr), rx, rr)) => Node(
                Red,
                box Node(Black, l, x, rll),
                rlx,
                box Node(Black, rlr, rx, rr),
            ),
            Node(Black, l, x, box Node(Red, rl, rx, box Node(Red, rrl, rrx, rrr))) => Node(
                Red,
                box Node(Black, l, x, rl),
                rx,
                box Node(Black, rrl, rrx, rrr),
            ),
            s => s,
        }
    }

    pub fn insert(self, x: T) -> RBT<T> {
        use Color::Black;
        use RBT::{Leaf, Node};
        match self.insert_sub(x) {
            Leaf => panic!(),
            Node(_, left, data, right) => Node(Black, left, data, right),
        }
    }

    pub fn max_height(&self) -> i32 {
        use RBT::{Leaf, Node};
        match self {
            Leaf => 0,
            Node(_, left, _, right) => {
                let left_height = left.max_height();
                let right_height = right.max_height();
                left_height.max(right_height) + 1
            }
        }
    }

    pub fn min_height(&self) -> i32 {
        use RBT::{Leaf, Node};
        match self {
            Leaf => 0,
            Node(_, left, _, right) => {
                let left_height = left.min_height();
                let right_height = right.min_height();
                left_height.min(right_height) + 1
            }
        }
    }

    fn delete_min(self) -> (RBT<T>, Option<(Color, T)>) {
        use RBT::{Leaf, Node};
        match self {
            Leaf => (Leaf, None),
            Node(color, box Leaf, x, box right) => (right, Some((color, x))),
            Node(color, left, x, right) => {
                let (left, result) = left.delete_min();
                (Node(color, box left, x, right), result)
            }
        }
    }

    fn delete_max(self) -> (RBT<T>, Option<(Color, T)>) {
        use RBT::{Leaf, Node};
        match self {
            Leaf => (Leaf, None),
            Node(color, box left, x, box Leaf) => (left, Some((color, x))),
            Node(color, left, x, right) => {
                let (right, result) = right.delete_max();
                (Node(color, left, x, box right).balance(), result)
            }
        }
    }

    fn rotate_left(self) -> (bool, RBT<T>) {
        use Color::{Black, Red};
        use RBT::Node;
        match self {
            Node(color, l, x, box Node(Black, box Node(Red, rll, rlx, rlr), rx, rr)) => (
                false,
                Node(
                    color,
                    box Node(Black, l, x, rll),
                    rlx,
                    box Node(Black, rlr, rx, rr),
                ),
            ),
            Node(color, l, x, box Node(Black, rl, rx, box Node(Red, rrl, rrx, rrr))) => (
                false,
                Node(
                    color,
                    box Node(Black, l, x, rl),
                    rx,
                    box Node(Black, rrl, rrx, rrr),
                ),
            ),
            Node(color, l, x, box Node(Black, rl, rx, rr)) => {
                (color == Black, Node(Black, l, x, box Node(Red, rl, rx, rr)))
            }
            Node(Black, l, x, box Node(Red, rl, rx, rr)) => {
                let (_, left) = Node(Red, l, x, rl).rotate_left();
                (false, Node(Black, box left, rx, rr))
            }
            _ => (false, self),
        }
    }

    fn rotate_right(self) -> (bool, RBT<T>) {
        use Color::{Black, Red};
        use RBT::Node;
        match self {
            Node(color, box Node(Black, ll, lx, box Node(Red, lrl, lrx, lrr)), x, r) => (
                false,
                Node(
                    color,
                    box Node(Black, ll, lx, lrl),
                    lrx,
                    box Node(Black, lrr, x, r),
                ),
            ),
            Node(color, box Node(Black, box Node(Red, lll, llx, llr), lx, lr), x, r) => (
                false,
                Node(
                    color,
                    box Node(Black, lll, llx, llr),
                    lx,
                    box Node(Black, lr, x, r),
                ),
            ),
            Node(color, box Node(Black, ll, lx, lr), x, r) => {
                (color == Black, Node(Black, box Node(Red, ll, lx, lr), x, r))
            }
            Node(Black, box Node(Red, ll, lx, lr), x, r) => {
                let (_, right) = Node(Red, lr, x, r).rotate_right();
                (false, Node(Black, ll, lx, box right))
            }
            _ => (false, self),
        }
    }

    fn delete_sub(self, x: T) -> (bool, RBT<T>) {
        use Color::{Black, Red};
        use RBT::{Leaf, Node};
        match self {
            Leaf => (false, Leaf),
            Node(color, left, data, box right) => match x {
                x if x > data => {
                    let (should_rotate, right) = right.delete_sub(x);
                    if should_rotate {
                        Node(color, left, data, box right).rotate_right()
                    } else {
                        (false, Node(color, left, data, box right))
                    }
                }
                x if x < data => {
                    let (should_rotate, left) = left.delete_sub(x);
                    if should_rotate {
                        Node(color, box left, data, box right).rotate_left()
                    } else {
                        (false, Node(color, box left, data, box right))
                    }
                }
                _ => {
                    let (left, result) = left.delete_max();
                    match result {
                        None => (color == Black, right),
                        Some((Black, x)) => Node(color, box left, x, box right).rotate_left(),
                        Some((Red, x)) => (color == Black, Node(color, box left, x, box right)),
                    }
                }
            },
        }
    }

    pub fn delete(self, x: T) -> RBT<T> {
        use Color::Black;
        use RBT::{Leaf, Node};
        match self.delete_sub(x) {
            (_, Leaf) => panic!(),
            (_, Node(_, left, data, right)) => Node(Black, left, data, right),
        }
    }

    fn insert_sub(self, x: T) -> RBT<T> {
        use Color::Red;
        use RBT::{Leaf, Node};
        match self {
            Leaf => Node(Red, box Leaf, x, box Leaf),
            Node(color, left, data, right) => match x {
                x if x > data => Node(color, left, data, box right.insert_sub(x)).balance(),
                x if x < data => Node(color, box left.insert_sub(x), data, right).balance(),
                _ => Node(color, left, data, right),
            },
        }
    }
}

fn main() {
    use RBT::Leaf;
    let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let tree = num.iter().fold(Leaf, |sum, current| sum.insert(current));
    println!("{:?}", tree);
    println!("{:?}", tree.clone().delete(&6));
    println!("{:?}", tree.delete(&9));
}

#[test]
fn balance_test() {
    use Color::{Black, Red};
    use RBT::{Leaf, Node};
    let case1 = Node(
        Black,
        box Node(Red, box Node(Red, box Leaf, 3, box Leaf), 4, box Leaf),
        6,
        box Leaf,
    );
    let result1 = case1.balance();
    let expected_result_1 = Node(
        Red,
        box Node(Black, box Leaf, 3, box Leaf),
        4,
        box Node(Black, box Leaf, 6, box Leaf),
    );
    assert_eq!(result1, expected_result_1);
}
