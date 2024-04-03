use std::alloc::alloc;
use std::alloc::Layout;
use std::mem::align_of;

type Align = [u8; 16];

enum Header {
  Data {
    size: usize,
    is_free: bool,
    next: *mut Header,
  },
  Stub(Align),
}

type Block = Header;

fn malloc(size: usize) -> *mut u8 {
  //  create space in heap for size bytes
  let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();
  let ptr = unsafe { alloc(layout) };
  let pthread_mutex_block = Box::new(Block::Data { size, is_free: false, next: std::ptr::null_mut() });

  let header = Box::new(Block::Stub([0; 16]))

  if header.size >= size {
    //  split the block
    //  create a new block with the remaining space
    //  set the new block's next to the current block's next
    //  set the current block's next to the new block
    //  set the current block's size to the requested size
    //  set the new block's size to the remaining size
  }
  //   ptr
}

fn main() {
  println!("Hello, world!");
}
