pub struct Bytes<'a> {
    pos: usize,
    buffer: &'a mut [u8],
}

impl<'a> Bytes<'a> {
    #[inline]
    pub fn new(buffer: &'a mut [u8]) -> Bytes<'a> {
        Bytes {
            pos: 0usize,
            buffer: buffer,
        }
    }

    #[inline]
    pub fn get_pos(&mut self) -> usize {
        self.pos
    }
    #[inline]
    pub fn set_pos(&mut self, pos: usize) {
        let len = self.buffer.len();
        if pos < len {
            self.pos = pos;
        } else {
            self.pos = len - 1;
        }
    }
    #[inline]
    pub fn get_size(&mut self) -> usize {
        self.buffer.len()
    }

    #[inline]
    pub fn write_u8(&mut self, n: u8) {
        self.pos += 1;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        self.buffer[(self.pos - 1)] = n;
    }

    #[inline]
    pub fn write_i8(&mut self, n: i8) {
        self.pos += 1;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        self.buffer[self.pos - 1] = n as u8;
    }

    #[inline]
    pub fn write_u16(&mut self, n: u16) {
        self.pos += 2;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 2] = ns[0];
        self.buffer[self.pos - 1] = ns[1];
    }
    #[inline]
    pub fn write_i16(&mut self, n: i16) {
        self.pos += 2;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 2] = ns[0];
        self.buffer[self.pos - 1] = ns[1];
    }

    #[inline]
    pub fn write_u32(&mut self, n: u32) {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 4] = ns[0];
        self.buffer[self.pos - 3] = ns[1];
        self.buffer[self.pos - 2] = ns[2];
        self.buffer[self.pos - 1] = ns[3];
    }
    #[inline]
    pub fn write_i32(&mut self, n: i32) {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 4] = ns[0];
        self.buffer[self.pos - 3] = ns[1];
        self.buffer[self.pos - 2] = ns[2];
        self.buffer[self.pos - 1] = ns[3];
    }
    #[inline]
    pub fn write_u64(&mut self, n: u64) {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 8] = ns[0];
        self.buffer[self.pos - 7] = ns[1];
        self.buffer[self.pos - 6] = ns[2];
        self.buffer[self.pos - 5] = ns[3];
        self.buffer[self.pos - 4] = ns[4];
        self.buffer[self.pos - 3] = ns[5];
        self.buffer[self.pos - 2] = ns[6];
        self.buffer[self.pos - 1] = ns[7];
    }

    #[inline]
    pub fn write_i64(&mut self, n: i64) {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 8] = ns[0];
        self.buffer[self.pos - 7] = ns[1];
        self.buffer[self.pos - 6] = ns[2];
        self.buffer[self.pos - 5] = ns[3];
        self.buffer[self.pos - 4] = ns[4];
        self.buffer[self.pos - 3] = ns[5];
        self.buffer[self.pos - 2] = ns[6];
        self.buffer[self.pos - 1] = ns[7];
    }

    #[inline]
    pub fn write_f32(&mut self, n: f32) {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 4] = ns[0];
        self.buffer[self.pos - 3] = ns[1];
        self.buffer[self.pos - 2] = ns[2];
        self.buffer[self.pos - 1] = ns[3];
    }

    #[inline]
    pub fn write_f64(&mut self, n: f64) {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        let ns = n.to_le_bytes();
        self.buffer[self.pos - 8] = ns[0];
        self.buffer[self.pos - 7] = ns[1];
        self.buffer[self.pos - 6] = ns[2];
        self.buffer[self.pos - 5] = ns[3];
        self.buffer[self.pos - 4] = ns[4];
        self.buffer[self.pos - 3] = ns[5];
        self.buffer[self.pos - 2] = ns[6];
        self.buffer[self.pos - 1] = ns[7];
    }

    #[inline]
    pub fn write_bytes(&mut self, n: &[u8]) {
        let len = n.len();
        if len == 0 {
            debug_assert!(false);
            return ();
        }
        self.pos += len;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return ();
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                n.as_ptr(),
                self.buffer[self.pos - len..self.pos].as_mut_ptr(),
                len,
            );
        }
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.pos += 1;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0u8;
        }
        self.buffer[self.pos - 1]
    }
    #[inline]
    pub fn read_i8(&mut self) -> i8 {
        self.pos += 1;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0i8;
        }
        self.buffer[self.pos - 1] as i8
    }
    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        self.pos += 2;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0u16;
        }
        let p: *const u8 = self.buffer[self.pos - 2..].as_ptr();
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
    pub fn read_i16(&mut self) -> i16 {
        self.pos += 2;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0i16;
        }
        let p: *const u8 = self.buffer[self.pos - 2..].as_ptr();
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
    pub fn read_u32(&mut self) -> u32 {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0u32;
        }
        let p: *const u8 = self.buffer[self.pos - 4..].as_ptr();
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
    pub fn read_i32(&mut self) -> i32 {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0i32;
        }
        let p: *const u8 = self.buffer[self.pos - 4..].as_ptr();
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
    pub fn read_u64(&mut self) -> u64 {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0u64;
        }
        let p: *const u8 = self.buffer[self.pos - 8..].as_ptr();
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
    pub fn read_i64(&mut self) -> i64 {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0i64;
        }
        let p: *const u8 = self.buffer[self.pos - 8..].as_ptr();
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
    pub fn read_f32(&mut self) -> f32 {
        self.pos += 4;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0f32;
        }
        #[cfg(target_endian = "little")]
        {
            let p: *const u8 = self.buffer[self.pos - 4..].as_ptr();
            unsafe { *(p as *const f32) }
        }
        #[cfg(not(target_endian = "little"))]
        {
            let p: *const u8 = [
                self.buffer[self.pos - 3],
                self.buffer[self.pos - 2],
                self.buffer[self.pos - 1],
                self.buffer[self.pos],
            ]
            .as_ptr();
            unsafe { *(p as *const f32) }
        }
    }

    #[inline]
    pub fn read_f64(&mut self) -> f64 {
        self.pos += 8;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return 0f64;
        }
        #[cfg(target_endian = "little")]
        {
            let p: *const u8 = self.buffer[self.pos - 8..].as_ptr();
            unsafe { *(p as *const f64) }
        }
        #[cfg(not(target_endian = "little"))]
        {
            let p: *const u8 = [
                self.buffer[self.pos - 7],
                self.buffer[self.pos - 6],
                self.buffer[self.pos - 5],
                self.buffer[self.pos - 4],
                self.buffer[self.pos - 3],
                self.buffer[self.pos - 2],
                self.buffer[self.pos - 1],
                self.buffer[self.pos],
            ]
            .as_ptr();
            unsafe { *(p as *const f64) }
        }
    }

    #[inline]
    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        self.pos += size;
        if self.pos > self.buffer.len() {
            debug_assert!(false);
            return &[];
        }
        &self.buffer[(self.pos - size)..self.pos]
    }
}
/*
fn test() {
    let mut vbytes = vec![0u8; 42];

    let mut _bytes = bytes::Bytes::new(vbytes.as_mut_slice());

    _bytes.write_bytes("hello ccc".as_bytes());

    _bytes.set_pos(0);
    println!("str:{}", String::from_utf8_lossy(_bytes.read_bytes(8)));

    let mut vec = vec![0u8; 42];
    println!("Vec Len:{}", vec.len());
    let mut _bytes = bytes::Bytes::new(vec.as_mut_slice());
    println!("bytes size:{}", _bytes.get_size());
    _bytes.write_u8(9);
    _bytes.write_i8(-9);
    _bytes.write_u16(999);

    _bytes.write_i16(-999);
    _bytes.write_u32(99999);
    _bytes.write_i32(-99999);
    _bytes.write_u64(9999999999);
    _bytes.write_i64(-9999999999);
    _bytes.write_f32(99999.99);
    _bytes.write_f64(-99999.99);

    println!("pos:{}", _bytes.get_pos());

    _bytes.set_pos(0);
    println!("read_u8:{}", _bytes.read_u8());
    println!("read_i8:{}", _bytes.read_i8());
    println!("read_u16:{}", _bytes.read_u16());
    println!("read_i16:{}", _bytes.read_i16());
    println!("read_u32:{}", _bytes.read_u32());
    println!("read_i32:{}", _bytes.read_i32());
    println!("read_u64:{}", _bytes.read_u64());
    println!("read_i64:{}", _bytes.read_i64());
    println!("read_f32:{}", _bytes.read_f32());
    println!("read_f64:{}", _bytes.read_f64());

    println!("pos:{}", _bytes.get_pos());

    let mut f32_bytes = 99.9f32.to_le_bytes();
    let mut _bytes = bytes::Bytes::new(&mut f32_bytes);
    println!("leu8_to_f32:{}", _bytes.read_f32());

    let mut f64_bytes = 99.9f64.to_le_bytes();
    let mut _bytes = bytes::Bytes::new(&mut f64_bytes);
    println!("leu8_to_f64:{}", _bytes.read_f64());

    let mut be_bytes = 1u16.to_be_bytes();
    let mut le_bytes = 1u16.to_le_bytes();
    let mut bytes_be = bytes::Bytes::new(&mut be_bytes);
    let mut bytes_le = bytes::Bytes::new(&mut le_bytes);
    println!(
        "be:{} le:{}",
        bytes_be.read_u16().to_le(),
        bytes_le.read_u16()
    );

    let mut fle_bytes = 1f32.to_le_bytes();
    let mut fbe_bytes = 0.000000000000000000000000000000000000000046006f32.to_be_bytes();

    let mut bytes_fle = bytes::Bytes::new(&mut fle_bytes);
    let mut bytes_fbe = bytes::Bytes::new(&mut fbe_bytes);
    println!("be:{} le:{}", bytes_fle.read_f32(), bytes_fbe.read_f32());
}
*/

/*
#[inline]
pub fn Write_u8(u8s: &mut [u8], pos: &mut usize, n: u8) {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    u8s[*pos - 1] = n as u8;
}

#[inline]
pub fn Write_i8(u8s: &mut [u8], pos: &mut usize, n: i8) {
    *pos += 1;
    if *pos > u8s.len() {
        debug_assert!(false);
        return ();
    }
    u8s[*pos - 1] = n as u8;
}

#[inline]
pub fn Write_u16(u8s: &mut [u8], pos: &mut usize, n: u16) {
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
pub fn Write_i16(u8s: &mut [u8], pos: &mut usize, n: i16) {
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
pub fn Write_u32(u8s: &mut [u8], pos: &mut usize, n: u32) {
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
pub fn Write_i32(u8s: &mut [u8], pos: &mut usize, n: i32) {
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
pub fn Write_u64(u8s: &mut [u8], pos: &mut usize, n: u64) {
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
pub fn Write_i64(u8s: &mut [u8], pos: &mut usize, n: i64) {
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
pub fn Write_f32(u8s: &mut [u8], pos: &mut usize, n: f32) {
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

    let mut bytes = self::bytes::new(vbytes);

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
*/
