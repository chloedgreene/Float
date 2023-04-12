//PLEASE READ BEFORE LOOKING
// this is a culmination of around 6 hours of p a i n and s u f f e r i n g of a human being
// this is a really REALLY bad code, it is messy with no comments, so i beg you
// just take it as a fact, its really fast

//we also make them inline, because functions calls are expensive in rendering code


//STOLE THIS FROM STACK OVERFLOW
// I KNOW ITS UNSAFE, BUT I NEED S P E E D
#[inline]
pub fn to_bits(x: f32) -> u32 {
    unsafe { ::std::mem::transmute::<f32, u32>(x) }
}

// I AM T I R E D, H E L P M E
//I HATE OPTIMIZATIONS AHSASBDOASDVSIKADVKASbjkabadkSB
#[inline]
pub fn from_bits(x: u32) -> f32 {
    unsafe { ::std::mem::transmute::<u32, f32>(x) }
}

#[inline]
pub fn fcos(x: f32) -> f32 {
    const PIHALF: f32 = 0.63661977236758134;
    const CORRECTION: f32 = 0.54641335845679634;

    let v = to_bits(x) & 0x7FFFFFFF;

    let qpprox = 1.0_f32 - PIHALF * from_bits(v);

    qpprox + CORRECTION * qpprox * (1.0_f32 - qpprox * qpprox)
}

//TODO: W H A T D I D I C R E A T E
//TODO: MAKE TIME MECHINE TO ASK PAST SELF HOW I MADE THIS MESS
#[inline]
pub fn fsin(x: f32) -> f32 {
    const FOUROVERPI: f32 = 1.2732395447351627;
    const FOUROVERPISQ: f32 = 0.40528473456935109;
    const Q: f32 = 0.77633023248007499;

    let mut p = to_bits(0.22308510060189463_f32);
    let mut v = to_bits(x);

    let sign: u32 = v & 0x80000000;
    v &= 0x7FFFFFFF;

    let qpprox = FOUROVERPI * x - FOUROVERPISQ * x * from_bits(v);

    p |= sign;

    qpprox * (Q + from_bits(p) * qpprox)
}

