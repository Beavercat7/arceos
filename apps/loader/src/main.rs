#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]
#[cfg(feature = "axstd")]
use axstd::println;
const PLASH_START: usize = 0x22000000;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
  let mut apps_start = PLASH_START as *const u8;
  let mut apps_size;
  unsafe
  {
    apps_start = (PLASH_START + 40) as *const u8; //为了方便指定40为获取文件大小的地方
    apps_size = *apps_start as usize; // Dangerous!!! We need to get accurate size of apps. 
  }
  println!("Load payload ...");
  apps_start =  PLASH_START as *const u8;
  let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size)
};
  println!("content: {:?}:  size: {}", code,apps_size);  
  apps_start = (PLASH_START + 42) as *const u8; //为了方便指定40为获取文件大小的地方
  unsafe
  {
    apps_size = *apps_start as usize; // Dangerous!!! We need to get accurate size of apps. 
  }
  apps_start =  (PLASH_START + 10) as *const u8;
  let code = unsafe { core::slice::from_raw_parts(apps_start, apps_size)};
    println!("content: {:?}:  size: {}", code,apps_size);  
  println!("Load payload ok!");
}