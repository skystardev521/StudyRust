/*
struct Bytes {
    buffer: Vec<u8>,
    position: usize,
}

impl Bytes {

    pub fn new(size: usize) -> Bytes {
        Bytes {
            position: 0usize,
            buffer: Vec::with_capacity(size),
        }
    }

    #[inline]
    pub fn get_pos(&self) -> usize {
        self.position
    }
    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        let len = self.buffer.len();
        if pos < len {
            self.position = pos;
        } else {
            self.position = len - 1;
        }
    }
    #[inline]
    pub fn size(&self) -> usize {
        self.buffer.len()
    }
    */

#[inline]
pub fn write_u8(u8s: &mut [u8], pos: &mut usize, n: u8) {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    u8s[*pos - 1] = n as u8;
}

#[inline]
pub fn write_i8(u8s: &mut [u8], pos: &mut usize, n: i8) {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    u8s[*pos - 1] = n as u8;
}

#[inline]
pub fn write_u16(u8s: &mut [u8], pos: &mut usize, n: u16) {
    *pos += 2;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 2] = ns[0];
    u8s[*pos - 1] = ns[1];
}
#[inline]
pub fn write_i16(u8s: &mut [u8], pos: &mut usize, n: i16) {
    *pos += 2;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 2] = ns[0];
    u8s[*pos - 1] = ns[1];
}

#[inline]
pub fn write_u32(u8s: &mut [u8], pos: &mut usize, n: u32) {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 4] = ns[0];
    u8s[*pos - 3] = ns[1];
    u8s[*pos - 2] = ns[2];
    u8s[*pos - 1] = ns[3];
}
#[inline]
pub fn write_i32(u8s: &mut [u8], pos: &mut usize, n: i32) {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 4] = ns[0];
    u8s[*pos - 3] = ns[1];
    u8s[*pos - 2] = ns[2];
    u8s[*pos - 1] = ns[3];
}
#[inline]
pub fn write_u64(u8s: &mut [u8], pos: &mut usize, n: u64) {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 8] = ns[0];
    u8s[*pos - 7] = ns[1];
    u8s[*pos - 6] = ns[2];
    u8s[*pos - 5] = ns[3];
    u8s[*pos - 4] = ns[4];
    u8s[*pos - 3] = ns[5];
    u8s[*pos - 2] = ns[6];
    u8s[*pos - 1] = ns[7];
}

#[inline]
pub fn write_i64(u8s: &mut [u8], pos: &mut usize, n: i64) {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 8] = ns[0];
    u8s[*pos - 7] = ns[1];
    u8s[*pos - 6] = ns[2];
    u8s[*pos - 5] = ns[3];
    u8s[*pos - 4] = ns[4];
    u8s[*pos - 3] = ns[5];
    u8s[*pos - 2] = ns[6];
    u8s[*pos - 1] = ns[7];
}

#[inline]
pub fn write_f32(u8s: &mut [u8], pos: &mut usize, n: f32) {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 4] = ns[0];
    u8s[*pos - 3] = ns[1];
    u8s[*pos - 2] = ns[2];
    u8s[*pos - 1] = ns[3];
}

#[inline]
pub fn write_f64(u8s: &mut [u8], pos: &mut usize, n: f64) {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    let ns = n.to_le_bytes();
    u8s[*pos - 8] = ns[0];
    u8s[*pos - 7] = ns[1];
    u8s[*pos - 6] = ns[2];
    u8s[*pos - 5] = ns[3];
    u8s[*pos - 4] = ns[4];
    u8s[*pos - 3] = ns[5];
    u8s[*pos - 2] = ns[6];
    u8s[*pos - 1] = ns[7];
}

#[inline]
pub fn write_bytes(u8s: &mut [u8], pos: &mut usize, n: &[u8]) {
    let len = n.len();
    if len == 0 {
        debug_assert!(false);
        return ()
    }
    *pos += len;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    unsafe {
        std::ptr::copy_nonoverlapping(n.as_ptr(), u8s[*pos - len .. *pos].as_mut_ptr(), len);
    }
}

#[inline]
pub fn read_u8(u8s: &[u8], pos: &mut usize) -> u8 {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0u8;
    }
    u8s[*pos - 1]
}
#[inline]
pub fn read_i8(u8s: &[u8], pos: &mut usize) -> i8 {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0i8;
    }
    u8s[*pos - 1] as i8
}
#[inline]
pub fn read_u16(u8s: &[u8], pos: &mut usize) -> u16 {
    *pos += 2;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0u16;
    }
    let p: *const u8 = u8s[*pos - 2..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const u16) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const u16) }.swap_bytes()
    }
}
#[inline]
pub fn read_i16(u8s: &[u8], pos: &mut usize) -> i16 {
    *pos += 2;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0i16;
    }
    let p: *const u8 = u8s[*pos - 2..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const i16) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const i16) }.swap_bytes()
    }
}
#[inline]
pub fn read_u32(u8s: &[u8], pos: &mut usize) -> u32 {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0u32;
    }
    let p: *const u8 = u8s[*pos - 4..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const u32) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const u32) }.swap_bytes()
    }
}
#[inline]
pub fn read_i32(u8s: &[u8], pos: &mut usize) -> i32 {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0i32;
    }
    let p: *const u8 = u8s[*pos - 4..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const i32) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const i32) }.swap_bytes()
    }
}

#[inline]
pub fn read_u64(u8s: &[u8], pos: &mut usize) -> u64 {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0u64;
    }
    let p: *const u8 = u8s[*pos - 8..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const u64) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const u64) }.swap_bytes()
    }
}
#[inline]
pub fn read_i64(u8s: &[u8], pos: &mut usize) -> i64 {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0i64;
    }
    let p: *const u8 = u8s[*pos - 8..].as_ptr();
    #[cfg(target_endian = "little")]
    {
        unsafe { *(p as *const i64) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        unsafe { *(p as *const i64) }.swap_bytes()
    }
}

#[inline]
pub fn read_f32(u8s: &[u8], pos: &mut usize) -> f32 {
    *pos += 4;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0f32;
    }
    #[cfg(target_endian = "little")]
    {
        let p: *const u8 = u8s[*pos - 4..].as_ptr();
        unsafe { *(p as *const f32) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        let p: *const u8 = [u8s[*pos - 3], u8s[*pos - 2], u8s[*pos - 1], u8s[*pos]].as_ptr();
        unsafe { *(p as *const f32) }
    }
}

#[inline]
pub fn read_f64(u8s: &[u8], pos: &mut usize) -> f64 {
    *pos += 8;
    if *pos > u8s.len() {
        debug_assert!(false);
        return 0f64;
    }
    #[cfg(target_endian = "little")]
    {
        let p: *const u8 = u8s[*pos - 8..].as_ptr();
        unsafe { *(p as *const f64) }
    }
    #[cfg(not(target_endian = "little"))]
    {
        let p: *const u8 = [
            u8s[*pos - 7],
            u8s[*pos - 6],
            u8s[*pos - 5],
            u8s[*pos - 4],
            u8s[*pos - 3],
            u8s[*pos - 2],
            u8s[*pos - 1],
            u8s[*pos],
        ]
        .as_ptr();
        unsafe { *(p as *const f64) }
    }
}

#[inline]
pub fn read_bytes<'a>(u8s: &'a[u8], pos: &mut usize, size:usize) -> &'a[u8] {
    *pos += size;
    if *pos > u8s.len() {
        debug_assert!(false);
        return &u8s[0..0]
    }
    &u8s[*pos -size .. *pos]
}


pub fn test() {

    let mut vbytes = vec![0u8; 42];

    write_bytes(&mut vbytes, &mut 0, "hello ccc".as_bytes());

    println!("str:{:?}", String::from_utf8_lossy(read_bytes(&mut vbytes, &mut 0, 8)));

    let mut pos = 0;
    let mut vec = vec![0u8; 42];
    println!("Vec Len:{}", vec.len());

    write_u8(&mut vec, &mut pos, 9);
    write_i8(&mut vec, &mut pos, -9);
    write_u16(&mut vec, &mut pos, 999);

    write_i16(&mut vec, &mut pos, -999);
    write_u32(&mut vec, &mut pos, 99999);
    write_i32(&mut vec, &mut pos, -99999);
    write_u64(&mut vec, &mut pos, 9999999999);
    write_i64(&mut vec, &mut pos, -9999999999);
    write_f32(&mut vec, &mut pos, 99999.99);
    write_f64(&mut vec, &mut pos, -99999.99);

    println!("pos:{}", pos);

    let mut pos = 0;
    println!("read_u8:{}", read_u8(&vec, &mut pos));
    println!("read_i8:{}", read_i8(&vec, &mut pos));
    println!("read_u16:{}", read_u16(&vec, &mut pos));
    println!("read_i16:{}", read_i16(&vec, &mut pos));
    println!("read_u32:{}", read_u32(&vec, &mut pos));
    println!("read_i32:{}", read_i32(&vec, &mut pos));
    println!("read_u64:{}", read_u64(&vec, &mut pos));
    println!("read_i64:{}", read_i64(&vec, &mut pos));
    println!("read_f32:{}", read_f32(&vec, &mut pos));
    println!("read_f64:{}", read_f64(&vec, &mut pos));
    println!("pos:{}", pos);

    let f32_bytes = 99.9f32.to_le_bytes();
    println!("leu8_to_f32:{}", read_f32(&f32_bytes, &mut 0));

    let f64_bytes = 99.9f64.to_le_bytes();
    println!("leu8_to_f64:{}", read_f64(&f64_bytes, &mut 0));

    let be = 1u16.to_be_bytes();
    let le = 1u16.to_le_bytes();
    println!(
        "be:{} le:{}",
        read_u16(&be, &mut 0).to_le(),
        read_u16(&le, &mut 0)
    );

    let fle = 1f32.to_le_bytes();
    let fbe = 0.000000000000000000000000000000000000000046006f32.to_be_bytes();
    println!(
        "be:{} le:{}",
        read_f32(&fbe, &mut 0),
        read_f32(&fle, &mut 0)
    );
}
