#![feature(asm_const)]

#![no_std]

#![no_main]

const SYS_HELLO: usize = 1;

const SYS_PUTCHAR: usize = 2;

const SYS_TERMINATE: usize = 3;

const SYS_PUT: usize = 4;

// static mut ABI_TABLE: [usize; 16] = [0; 16];
static mut variable:usize = 0;
fn puthello() {
    
  unsafe { 
    core::arch::asm!("        
    ",
    in("a7")variable,
  );
    core::arch::asm!("
    li t0, {abi_num}
    slli t0, t0, 3
    add t1, a7, t0
    ld t1, (t1)
    jalr t1
    ",  
    abi_num = const SYS_HELLO,
    clobber_abi("C")
)
}
}


fn putchar(c: u8) 
{
  // unsafe {
  //   core::arch::asm!("        
  //   ",
  //   in("a7")variable,
  //  );
    // core::arch::asm!("        
    //     li t0, {abi_num}
    //     slli t0, t0, 3 
    //     add t1, a7, t0
    //     ld t1, (t1)
    //     jalr t1 
    //     ",
    //     abi_num = const SYS_PUTCHAR,
    //     in("a0") c, 
    //     clobber_abi("C"))
   //}
}


// fn puts(s: &str) 
// {
//  let len = str.len();
//  for 0..len
//   putchar(str[])
// }


fn terminate()
{
 unsafe {
  core::arch::asm!("        
  ",
  in("a7")variable,
);
  core::arch::asm!("
      li t0, {abi_num}
      slli t0, t0, 3    
      add t1, a7, t0
      ld t1, (t1)
      jalr t1
      ",
      abi_num = const SYS_TERMINATE, 
      clobber_abi("C")  
  )
}
}

fn put(c:usize)
{
  unsafe {
    core::arch::asm!("
        li t0, {abi_num}
        slli t0, t0, 3    
        add t1, a7, t0
        ld t1, (t1)
        jalr t1
        ",
        abi_num = const SYS_PUT,
        in("a0") c, 
        clobber_abi("C")
    )
}
}

#[no_mangle]

unsafe extern "C" fn _start() -> () {
  unsafe { 
    core::arch::asm!(
      "",
      out("a7")variable,
      clobber_abi("C")
    )
  };
//  执行顺序如下:

  puthello();
 
  //puthello();
  //let c = b'T';

  //putchar(c);

  //putchar(c);
  
  //put(variable);
   
  terminate();

}



use core::panic::PanicInfo;

#[panic_handler]

fn panic(_info: &PanicInfo) -> ! {

 loop {}

}