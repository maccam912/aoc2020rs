use nom::{branch::alt, bytes::complete::tag, sequence::pair, IResult};

fn parse(i: &str) -> IResult<&str, &str> {
    let _1 = tag("a");
    let _14 = tag("b");
    let _6 = alt((pair(_14, _14), pair(_1, _14)));
    let _15 = alt((_1, _14));
    let _18 = pair(_15, _15);
    let _27 = alt((pair(_1, _6), pair(_14, _18)));
    let _20 = alt((pair(_14, _14), pair(_1, _15)));
    let _22 = pair(_14, _14);
    let _26 = alt((pair(_14, _22), pair(_1, _20)));
    let _9 = alt((pair(_14, _27), pair(_1, _26)));
    let _25 = alt((pair(_1, _1), pair(_1, _14)));
    let _23 = alt((pair(_25, _1), pair(_22, _14)));
    let _5 = alt((pair(_1, _14), pair(_15, _1)));
    let _19 = alt((pair(_14, _1), pair(_14, _14)));
    let _24 = pair(_14, _1);
    let _12 = alt((pair(_24, _14), pair(_19, _1)));
    let _16 = alt((pair(_15, _1), pair(_14, _14)));
    let _28 = pair(_16, _1);
    let _10 = alt((pair(_23, _14), pair(_28, _1)));
    let _42 = alt((pair(_9, _14), pair(_10, _1)));
    let _4 = pair(_1, _1);
    let _2 = alt((pair(_1, _24), pair(_14, _4)));
    let _21 = alt((pair(_14, _1), pair(_1, _14)));
    let _7 = alt((pair(_14, _5), pair(_1, _21)));
    let _17 = alt((pair(_14, _2), pair(_1, _7)));
    let _3 = alt((pair(_5, _14), pair(_16, _1)));
    let _13 = alt((pair(_14, _3), pair(_1, _12)));
    let _31 = alt((pair(_14, _17), pair(_1, _13)));
    let _11 = pair(_4, _31);
    let _8 = _42;
    let _0 = pair(_8, _11);

    _0(i)
}

pub fn day19(contents: &str) -> i64 {
    let mut parts = contents.split("\n\n");
    let _ = parts.next().unwrap();
    let lines = parts.next().unwrap();

    let mut sum = 0;
    for line in lines.split('\n') {
        if parse(line).is_ok() {
            sum += 1;
        }
    }
    sum
}
