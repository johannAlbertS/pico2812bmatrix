#[allow(non_upper_case_globals)]
pub static fonts: [&[&[bool]]; 27] = [
    [
        [false, true, true, true, true, true, true].as_slice(), //a
        [true, false, false, false, true, false, false].as_slice(),
        [true, false, false, false, true, false, false].as_slice(),
        [false, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //b
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [false, true, true, false, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(), //c
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //d
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //e
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //f
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
    ]
    .as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(), //g
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, true, false, true].as_slice(),
        [true, false, false, false, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //h
        [false, false, false, true, false, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, false, false, false, false, false, true].as_slice(), //i
        [true, true, true, true, true, true, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, false, false, false, false, true, false].as_slice(), //j
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(),
        [false, false, false, true, false, false, false].as_slice(), //k
        [false, false, true, false, true, false, false].as_slice(),
        [false, true, false, false, false, true, false].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //l
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //m
        [false, true, false, false, false, false, false].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [false, true, false, false, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //n
        [false, true, false, false, false, false, false].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(), //o
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, false, true].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //p
        [true, false, false, true, false, false, false].as_slice(),
        [true, false, false, true, false, false, false].as_slice(),
        [false, true, true, false, false, false, false].as_slice(),
    ]
    .as_slice(),
    [
        [false, true, true, true, true, true, false].as_slice(), //q
        [true, false, false, false, false, false, true].as_slice(),
        [true, false, false, false, false, true, true].as_slice(),
        [false, true, true, true, true, true, false].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //r
        [true, false, false, true, true, false, false].as_slice(),
        [true, false, false, true, false, true, false].as_slice(),
        [false, true, true, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [false, true, true, false, false, false, true].as_slice(), //s
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, false, false, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, false, false, false, false, false, false].as_slice(), //t
        [true, false, false, false, false, false, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
        [true, false, false, false, false, false, false].as_slice(),
        [true, false, false, false, false, false, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, false].as_slice(), //u
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [true, true, true, true, true, true, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, false, false].as_slice(), //v
        [false, false, false, false, false, true, false].as_slice(),
        [false, false, false, false, false, false, true].as_slice(),
        [false, false, false, false, false, true, false].as_slice(),
        [true, true, true, true, true, false, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, true, true, true, true, true].as_slice(), //w
        [false, false, false, false, false, true, false].as_slice(),
        [false, false, false, false, true, false, false].as_slice(),
        [false, false, false, false, false, true, false].as_slice(),
        [true, true, true, true, true, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, false, false, false, true, true].as_slice(), //x
        [false, false, true, false, true, false, false].as_slice(),
        [false, false, false, true, false, false, false].as_slice(),
        [false, false, true, false, true, false, false].as_slice(),
        [true, true, false, false, false, true, true].as_slice(),
    ]
    .as_slice(),
    [
        [true, true, false, false, false, false, false].as_slice(), //y
        [false, false, true, false, false, false, false].as_slice(),
        [false, false, false, true, true, true, true].as_slice(),
        [false, false, true, false, false, false, false].as_slice(),
        [true, true, false, false, false, false, false].as_slice(),
    ]
    .as_slice(),
    [
        [true, false, false, false, false, false, true].as_slice(), //z
        [true, false, false, false, true, false, true].as_slice(),
        [true, false, false, true, false, false, true].as_slice(),
        [true, false, true, false, false, false, true].as_slice(),
        [true, true, false, false, false, false, true].as_slice(),
    ]
    .as_slice(),
    [
        [false, false, false, false, false, false, false].as_slice(),
    ].as_slice(),
];
