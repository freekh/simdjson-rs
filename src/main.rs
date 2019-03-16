#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use std::io::Read;

///
///
unsafe fn sse_strstr(reader: Read) -> Option<usize> {
  fn a() -> bool {
    true
  }
  
  for i in (0..hs.len()).step_by(32) {
    // _mm256_loadu_si256();

        // const __m256i block_first = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(s + i));
        // const __m256i block_last  = _mm256_loadu_si256(reinterpret_cast<const __m256i*>(s + i + k - 1));

        // const __m256i eq_first = _mm256_cmpeq_epi8(first, block_first);
        // const __m256i eq_last  = _mm256_cmpeq_epi8(last, block_last);
  }
  None
}

fn main() {

  // let needle_bytes = needle.as_bytes();
  // let first = _mm256_set1_epi8(needle_bytes[0] as i8);
  // let last = _mm256_set1_epi8(needle_bytes[needle.len() - 1] as i8);

  unsafe {
    println!("{:?}", sse_strstr("absdgaslkfjsalfjsahellolkfasdfjsd".as_bytes()));
  }
}
