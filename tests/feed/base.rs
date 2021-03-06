#![allow(dead_code)]
use rust_tensortrade::{
    feed::{Group, Stream},
    oms::exchanges::StreamLike,
};

#[test]
fn test_streams() {
    let mut stream_a = Stream::source(std::ops::Range { start: 3, end: 5 }.into_iter());
    let mut stream_b = Stream::source(std::ops::Range { start: 1, end: 5 }.into_iter());

    stream_a.rename("STREAM_A".to_string());
    stream_b.rename("STREAM_B".to_string());

    let mapped = Stream::apply(stream_a, |n| n * 2);

    let group = Group::new(Vec::from([mapped, stream_b]));

    for i in group {
        println!("{:?}", i);
    }
}
