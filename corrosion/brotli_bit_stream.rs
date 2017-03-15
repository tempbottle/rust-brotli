extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliConvertBitDepthsToSymbols(
        depth : *const u8, len : usize, bits : *mut u16
    );
    fn BrotliCreateHuffmanTree(
        data : *const u32,
        length : usize,
        tree_limit : i32,
        tree : *mut HuffmanTree,
        depth : *mut u8
    );
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn BrotliSetDepth(
        p : i32, pool : *mut HuffmanTree, depth : *mut u8, max_depth : i32
    ) -> i32;
    fn BrotliWriteHuffmanTree(
        depth : *const u8,
        num : usize,
        tree_size : *mut usize,
        tree : *mut u8,
        extra_bits_data : *mut u8
    );
    fn __assert_fail(
        __assertion : *const u8,
        __file : *const u8,
        __line : u32,
        __function : *const u8
    );
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static mut kInsBase
    : [u32; 24]
    = [   0u32,
          1u32,
          2u32,
          3u32,
          4u32,
          5u32,
          6u32,
          8u32,
          10u32,
          14u32,
          18u32,
          26u32,
          34u32,
          50u32,
          66u32,
          98u32,
          130u32,
          194u32,
          322u32,
          578u32,
          1090u32,
          2114u32,
          6210u32,
          22594u32
      ];

static mut kInsExtra
    : [u32; 24]
    = [   0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          1u32,
          1u32,
          2u32,
          2u32,
          3u32,
          3u32,
          4u32,
          4u32,
          5u32,
          5u32,
          6u32,
          7u32,
          8u32,
          9u32,
          10u32,
          12u32,
          14u32,
          24u32
      ];

static mut kCopyBase
    : [u32; 24]
    = [   2u32,
          3u32,
          4u32,
          5u32,
          6u32,
          7u32,
          8u32,
          9u32,
          10u32,
          12u32,
          14u32,
          18u32,
          22u32,
          30u32,
          38u32,
          54u32,
          70u32,
          102u32,
          134u32,
          198u32,
          326u32,
          582u32,
          1094u32,
          2118u32
      ];

static mut kCopyExtra
    : [u32; 24]
    = [   0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          0u32,
          1u32,
          1u32,
          2u32,
          2u32,
          3u32,
          3u32,
          4u32,
          4u32,
          5u32,
          5u32,
          6u32,
          7u32,
          8u32,
          9u32,
          10u32,
          24u32
      ];

static mut kUTF8ContextLookup
    : [u8; 512]
    = [   0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          8i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          20i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          24i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          32i32 as (u8),
          12i32 as (u8),
          36i32 as (u8),
          12i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          32i32 as (u8),
          32i32 as (u8),
          24i32 as (u8),
          40i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8)
      ];

static mut kSigned3BitContextLookup
    : [u8; 256]
    = [   0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          7i32 as (u8)
      ];

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kCodeLengthDepth
    : [u8; 18]
    = [   4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          4i32 as (u8)
      ];

static mut kStaticCommandCodeDepth
    : [u8; 704]
    = [   9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          9i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8),
          11i32 as (u8)
      ];

static mut kStaticDistanceCodeDepth
    : [u8; 64]
    = [   6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8)
      ];

static mut kCodeLengthBits
    : [u32; 18]
    = [   0u32,
          8u32,
          4u32,
          12u32,
          2u32,
          10u32,
          6u32,
          14u32,
          1u32,
          9u32,
          5u32,
          13u32,
          3u32,
          15u32,
          31u32,
          0u32,
          11u32,
          7u32
      ];

static mut kZeroRepsBits
    : [usize; 704]
    = [   0x0usize,
          0x0usize,
          0x0usize,
          0x7usize,
          0x17usize,
          0x27usize,
          0x37usize,
          0x47usize,
          0x57usize,
          0x67usize,
          0x77usize,
          0x770usize,
          0xb87usize,
          0x1387usize,
          0x1b87usize,
          0x2387usize,
          0x2b87usize,
          0x3387usize,
          0x3b87usize,
          0x397usize,
          0xb97usize,
          0x1397usize,
          0x1b97usize,
          0x2397usize,
          0x2b97usize,
          0x3397usize,
          0x3b97usize,
          0x3a7usize,
          0xba7usize,
          0x13a7usize,
          0x1ba7usize,
          0x23a7usize,
          0x2ba7usize,
          0x33a7usize,
          0x3ba7usize,
          0x3b7usize,
          0xbb7usize,
          0x13b7usize,
          0x1bb7usize,
          0x23b7usize,
          0x2bb7usize,
          0x33b7usize,
          0x3bb7usize,
          0x3c7usize,
          0xbc7usize,
          0x13c7usize,
          0x1bc7usize,
          0x23c7usize,
          0x2bc7usize,
          0x33c7usize,
          0x3bc7usize,
          0x3d7usize,
          0xbd7usize,
          0x13d7usize,
          0x1bd7usize,
          0x23d7usize,
          0x2bd7usize,
          0x33d7usize,
          0x3bd7usize,
          0x3e7usize,
          0xbe7usize,
          0x13e7usize,
          0x1be7usize,
          0x23e7usize,
          0x2be7usize,
          0x33e7usize,
          0x3be7usize,
          0x3f7usize,
          0xbf7usize,
          0x13f7usize,
          0x1bf7usize,
          0x23f7usize,
          0x2bf7usize,
          0x33f7usize,
          0x3bf7usize,
          0x1c387usize,
          0x5c387usize,
          0x9c387usize,
          0xdc387usize,
          0x11c387usize,
          0x15c387usize,
          0x19c387usize,
          0x1dc387usize,
          0x1cb87usize,
          0x5cb87usize,
          0x9cb87usize,
          0xdcb87usize,
          0x11cb87usize,
          0x15cb87usize,
          0x19cb87usize,
          0x1dcb87usize,
          0x1d387usize,
          0x5d387usize,
          0x9d387usize,
          0xdd387usize,
          0x11d387usize,
          0x15d387usize,
          0x19d387usize,
          0x1dd387usize,
          0x1db87usize,
          0x5db87usize,
          0x9db87usize,
          0xddb87usize,
          0x11db87usize,
          0x15db87usize,
          0x19db87usize,
          0x1ddb87usize,
          0x1e387usize,
          0x5e387usize,
          0x9e387usize,
          0xde387usize,
          0x11e387usize,
          0x15e387usize,
          0x19e387usize,
          0x1de387usize,
          0x1eb87usize,
          0x5eb87usize,
          0x9eb87usize,
          0xdeb87usize,
          0x11eb87usize,
          0x15eb87usize,
          0x19eb87usize,
          0x1deb87usize,
          0x1f387usize,
          0x5f387usize,
          0x9f387usize,
          0xdf387usize,
          0x11f387usize,
          0x15f387usize,
          0x19f387usize,
          0x1df387usize,
          0x1fb87usize,
          0x5fb87usize,
          0x9fb87usize,
          0xdfb87usize,
          0x11fb87usize,
          0x15fb87usize,
          0x19fb87usize,
          0x1dfb87usize,
          0x1c397usize,
          0x5c397usize,
          0x9c397usize,
          0xdc397usize,
          0x11c397usize,
          0x15c397usize,
          0x19c397usize,
          0x1dc397usize,
          0x1cb97usize,
          0x5cb97usize,
          0x9cb97usize,
          0xdcb97usize,
          0x11cb97usize,
          0x15cb97usize,
          0x19cb97usize,
          0x1dcb97usize,
          0x1d397usize,
          0x5d397usize,
          0x9d397usize,
          0xdd397usize,
          0x11d397usize,
          0x15d397usize,
          0x19d397usize,
          0x1dd397usize,
          0x1db97usize,
          0x5db97usize,
          0x9db97usize,
          0xddb97usize,
          0x11db97usize,
          0x15db97usize,
          0x19db97usize,
          0x1ddb97usize,
          0x1e397usize,
          0x5e397usize,
          0x9e397usize,
          0xde397usize,
          0x11e397usize,
          0x15e397usize,
          0x19e397usize,
          0x1de397usize,
          0x1eb97usize,
          0x5eb97usize,
          0x9eb97usize,
          0xdeb97usize,
          0x11eb97usize,
          0x15eb97usize,
          0x19eb97usize,
          0x1deb97usize,
          0x1f397usize,
          0x5f397usize,
          0x9f397usize,
          0xdf397usize,
          0x11f397usize,
          0x15f397usize,
          0x19f397usize,
          0x1df397usize,
          0x1fb97usize,
          0x5fb97usize,
          0x9fb97usize,
          0xdfb97usize,
          0x11fb97usize,
          0x15fb97usize,
          0x19fb97usize,
          0x1dfb97usize,
          0x1c3a7usize,
          0x5c3a7usize,
          0x9c3a7usize,
          0xdc3a7usize,
          0x11c3a7usize,
          0x15c3a7usize,
          0x19c3a7usize,
          0x1dc3a7usize,
          0x1cba7usize,
          0x5cba7usize,
          0x9cba7usize,
          0xdcba7usize,
          0x11cba7usize,
          0x15cba7usize,
          0x19cba7usize,
          0x1dcba7usize,
          0x1d3a7usize,
          0x5d3a7usize,
          0x9d3a7usize,
          0xdd3a7usize,
          0x11d3a7usize,
          0x15d3a7usize,
          0x19d3a7usize,
          0x1dd3a7usize,
          0x1dba7usize,
          0x5dba7usize,
          0x9dba7usize,
          0xddba7usize,
          0x11dba7usize,
          0x15dba7usize,
          0x19dba7usize,
          0x1ddba7usize,
          0x1e3a7usize,
          0x5e3a7usize,
          0x9e3a7usize,
          0xde3a7usize,
          0x11e3a7usize,
          0x15e3a7usize,
          0x19e3a7usize,
          0x1de3a7usize,
          0x1eba7usize,
          0x5eba7usize,
          0x9eba7usize,
          0xdeba7usize,
          0x11eba7usize,
          0x15eba7usize,
          0x19eba7usize,
          0x1deba7usize,
          0x1f3a7usize,
          0x5f3a7usize,
          0x9f3a7usize,
          0xdf3a7usize,
          0x11f3a7usize,
          0x15f3a7usize,
          0x19f3a7usize,
          0x1df3a7usize,
          0x1fba7usize,
          0x5fba7usize,
          0x9fba7usize,
          0xdfba7usize,
          0x11fba7usize,
          0x15fba7usize,
          0x19fba7usize,
          0x1dfba7usize,
          0x1c3b7usize,
          0x5c3b7usize,
          0x9c3b7usize,
          0xdc3b7usize,
          0x11c3b7usize,
          0x15c3b7usize,
          0x19c3b7usize,
          0x1dc3b7usize,
          0x1cbb7usize,
          0x5cbb7usize,
          0x9cbb7usize,
          0xdcbb7usize,
          0x11cbb7usize,
          0x15cbb7usize,
          0x19cbb7usize,
          0x1dcbb7usize,
          0x1d3b7usize,
          0x5d3b7usize,
          0x9d3b7usize,
          0xdd3b7usize,
          0x11d3b7usize,
          0x15d3b7usize,
          0x19d3b7usize,
          0x1dd3b7usize,
          0x1dbb7usize,
          0x5dbb7usize,
          0x9dbb7usize,
          0xddbb7usize,
          0x11dbb7usize,
          0x15dbb7usize,
          0x19dbb7usize,
          0x1ddbb7usize,
          0x1e3b7usize,
          0x5e3b7usize,
          0x9e3b7usize,
          0xde3b7usize,
          0x11e3b7usize,
          0x15e3b7usize,
          0x19e3b7usize,
          0x1de3b7usize,
          0x1ebb7usize,
          0x5ebb7usize,
          0x9ebb7usize,
          0xdebb7usize,
          0x11ebb7usize,
          0x15ebb7usize,
          0x19ebb7usize,
          0x1debb7usize,
          0x1f3b7usize,
          0x5f3b7usize,
          0x9f3b7usize,
          0xdf3b7usize,
          0x11f3b7usize,
          0x15f3b7usize,
          0x19f3b7usize,
          0x1df3b7usize,
          0x1fbb7usize,
          0x5fbb7usize,
          0x9fbb7usize,
          0xdfbb7usize,
          0x11fbb7usize,
          0x15fbb7usize,
          0x19fbb7usize,
          0x1dfbb7usize,
          0x1c3c7usize,
          0x5c3c7usize,
          0x9c3c7usize,
          0xdc3c7usize,
          0x11c3c7usize,
          0x15c3c7usize,
          0x19c3c7usize,
          0x1dc3c7usize,
          0x1cbc7usize,
          0x5cbc7usize,
          0x9cbc7usize,
          0xdcbc7usize,
          0x11cbc7usize,
          0x15cbc7usize,
          0x19cbc7usize,
          0x1dcbc7usize,
          0x1d3c7usize,
          0x5d3c7usize,
          0x9d3c7usize,
          0xdd3c7usize,
          0x11d3c7usize,
          0x15d3c7usize,
          0x19d3c7usize,
          0x1dd3c7usize,
          0x1dbc7usize,
          0x5dbc7usize,
          0x9dbc7usize,
          0xddbc7usize,
          0x11dbc7usize,
          0x15dbc7usize,
          0x19dbc7usize,
          0x1ddbc7usize,
          0x1e3c7usize,
          0x5e3c7usize,
          0x9e3c7usize,
          0xde3c7usize,
          0x11e3c7usize,
          0x15e3c7usize,
          0x19e3c7usize,
          0x1de3c7usize,
          0x1ebc7usize,
          0x5ebc7usize,
          0x9ebc7usize,
          0xdebc7usize,
          0x11ebc7usize,
          0x15ebc7usize,
          0x19ebc7usize,
          0x1debc7usize,
          0x1f3c7usize,
          0x5f3c7usize,
          0x9f3c7usize,
          0xdf3c7usize,
          0x11f3c7usize,
          0x15f3c7usize,
          0x19f3c7usize,
          0x1df3c7usize,
          0x1fbc7usize,
          0x5fbc7usize,
          0x9fbc7usize,
          0xdfbc7usize,
          0x11fbc7usize,
          0x15fbc7usize,
          0x19fbc7usize,
          0x1dfbc7usize,
          0x1c3d7usize,
          0x5c3d7usize,
          0x9c3d7usize,
          0xdc3d7usize,
          0x11c3d7usize,
          0x15c3d7usize,
          0x19c3d7usize,
          0x1dc3d7usize,
          0x1cbd7usize,
          0x5cbd7usize,
          0x9cbd7usize,
          0xdcbd7usize,
          0x11cbd7usize,
          0x15cbd7usize,
          0x19cbd7usize,
          0x1dcbd7usize,
          0x1d3d7usize,
          0x5d3d7usize,
          0x9d3d7usize,
          0xdd3d7usize,
          0x11d3d7usize,
          0x15d3d7usize,
          0x19d3d7usize,
          0x1dd3d7usize,
          0x1dbd7usize,
          0x5dbd7usize,
          0x9dbd7usize,
          0xddbd7usize,
          0x11dbd7usize,
          0x15dbd7usize,
          0x19dbd7usize,
          0x1ddbd7usize,
          0x1e3d7usize,
          0x5e3d7usize,
          0x9e3d7usize,
          0xde3d7usize,
          0x11e3d7usize,
          0x15e3d7usize,
          0x19e3d7usize,
          0x1de3d7usize,
          0x1ebd7usize,
          0x5ebd7usize,
          0x9ebd7usize,
          0xdebd7usize,
          0x11ebd7usize,
          0x15ebd7usize,
          0x19ebd7usize,
          0x1debd7usize,
          0x1f3d7usize,
          0x5f3d7usize,
          0x9f3d7usize,
          0xdf3d7usize,
          0x11f3d7usize,
          0x15f3d7usize,
          0x19f3d7usize,
          0x1df3d7usize,
          0x1fbd7usize,
          0x5fbd7usize,
          0x9fbd7usize,
          0xdfbd7usize,
          0x11fbd7usize,
          0x15fbd7usize,
          0x19fbd7usize,
          0x1dfbd7usize,
          0x1c3e7usize,
          0x5c3e7usize,
          0x9c3e7usize,
          0xdc3e7usize,
          0x11c3e7usize,
          0x15c3e7usize,
          0x19c3e7usize,
          0x1dc3e7usize,
          0x1cbe7usize,
          0x5cbe7usize,
          0x9cbe7usize,
          0xdcbe7usize,
          0x11cbe7usize,
          0x15cbe7usize,
          0x19cbe7usize,
          0x1dcbe7usize,
          0x1d3e7usize,
          0x5d3e7usize,
          0x9d3e7usize,
          0xdd3e7usize,
          0x11d3e7usize,
          0x15d3e7usize,
          0x19d3e7usize,
          0x1dd3e7usize,
          0x1dbe7usize,
          0x5dbe7usize,
          0x9dbe7usize,
          0xddbe7usize,
          0x11dbe7usize,
          0x15dbe7usize,
          0x19dbe7usize,
          0x1ddbe7usize,
          0x1e3e7usize,
          0x5e3e7usize,
          0x9e3e7usize,
          0xde3e7usize,
          0x11e3e7usize,
          0x15e3e7usize,
          0x19e3e7usize,
          0x1de3e7usize,
          0x1ebe7usize,
          0x5ebe7usize,
          0x9ebe7usize,
          0xdebe7usize,
          0x11ebe7usize,
          0x15ebe7usize,
          0x19ebe7usize,
          0x1debe7usize,
          0x1f3e7usize,
          0x5f3e7usize,
          0x9f3e7usize,
          0xdf3e7usize,
          0x11f3e7usize,
          0x15f3e7usize,
          0x19f3e7usize,
          0x1df3e7usize,
          0x1fbe7usize,
          0x5fbe7usize,
          0x9fbe7usize,
          0xdfbe7usize,
          0x11fbe7usize,
          0x15fbe7usize,
          0x19fbe7usize,
          0x1dfbe7usize,
          0x1c3f7usize,
          0x5c3f7usize,
          0x9c3f7usize,
          0xdc3f7usize,
          0x11c3f7usize,
          0x15c3f7usize,
          0x19c3f7usize,
          0x1dc3f7usize,
          0x1cbf7usize,
          0x5cbf7usize,
          0x9cbf7usize,
          0xdcbf7usize,
          0x11cbf7usize,
          0x15cbf7usize,
          0x19cbf7usize,
          0x1dcbf7usize,
          0x1d3f7usize,
          0x5d3f7usize,
          0x9d3f7usize,
          0xdd3f7usize,
          0x11d3f7usize,
          0x15d3f7usize,
          0x19d3f7usize,
          0x1dd3f7usize,
          0x1dbf7usize,
          0x5dbf7usize,
          0x9dbf7usize,
          0xddbf7usize,
          0x11dbf7usize,
          0x15dbf7usize,
          0x19dbf7usize,
          0x1ddbf7usize,
          0x1e3f7usize,
          0x5e3f7usize,
          0x9e3f7usize,
          0xde3f7usize,
          0x11e3f7usize,
          0x15e3f7usize,
          0x19e3f7usize,
          0x1de3f7usize,
          0x1ebf7usize,
          0x5ebf7usize,
          0x9ebf7usize,
          0xdebf7usize,
          0x11ebf7usize,
          0x15ebf7usize,
          0x19ebf7usize,
          0x1debf7usize,
          0x1f3f7usize,
          0x5f3f7usize,
          0x9f3f7usize,
          0xdf3f7usize,
          0x11f3f7usize,
          0x15f3f7usize,
          0x19f3f7usize,
          0x1df3f7usize,
          0x1fbf7usize,
          0x5fbf7usize,
          0x9fbf7usize,
          0xdfbf7usize,
          0x11fbf7usize,
          0x15fbf7usize,
          0x19fbf7usize,
          0x1dfbf7usize,
          0xe1c387usize,
          0x2e1c387usize,
          0x4e1c387usize,
          0x6e1c387usize,
          0x8e1c387usize,
          0xae1c387usize,
          0xce1c387usize,
          0xee1c387usize,
          0xe5c387usize,
          0x2e5c387usize,
          0x4e5c387usize,
          0x6e5c387usize,
          0x8e5c387usize,
          0xae5c387usize,
          0xce5c387usize,
          0xee5c387usize,
          0xe9c387usize,
          0x2e9c387usize,
          0x4e9c387usize,
          0x6e9c387usize,
          0x8e9c387usize,
          0xae9c387usize,
          0xce9c387usize,
          0xee9c387usize,
          0xedc387usize,
          0x2edc387usize,
          0x4edc387usize,
          0x6edc387usize,
          0x8edc387usize,
          0xaedc387usize,
          0xcedc387usize,
          0xeedc387usize,
          0xf1c387usize,
          0x2f1c387usize,
          0x4f1c387usize,
          0x6f1c387usize,
          0x8f1c387usize,
          0xaf1c387usize,
          0xcf1c387usize,
          0xef1c387usize,
          0xf5c387usize,
          0x2f5c387usize,
          0x4f5c387usize,
          0x6f5c387usize,
          0x8f5c387usize,
          0xaf5c387usize,
          0xcf5c387usize,
          0xef5c387usize,
          0xf9c387usize,
          0x2f9c387usize,
          0x4f9c387usize,
          0x6f9c387usize,
          0x8f9c387usize,
          0xaf9c387usize,
          0xcf9c387usize,
          0xef9c387usize,
          0xfdc387usize,
          0x2fdc387usize,
          0x4fdc387usize,
          0x6fdc387usize,
          0x8fdc387usize,
          0xafdc387usize,
          0xcfdc387usize,
          0xefdc387usize,
          0xe1cb87usize,
          0x2e1cb87usize,
          0x4e1cb87usize,
          0x6e1cb87usize,
          0x8e1cb87usize,
          0xae1cb87usize,
          0xce1cb87usize,
          0xee1cb87usize,
          0xe5cb87usize,
          0x2e5cb87usize,
          0x4e5cb87usize,
          0x6e5cb87usize,
          0x8e5cb87usize,
          0xae5cb87usize,
          0xce5cb87usize,
          0xee5cb87usize,
          0xe9cb87usize,
          0x2e9cb87usize,
          0x4e9cb87usize,
          0x6e9cb87usize,
          0x8e9cb87usize,
          0xae9cb87usize,
          0xce9cb87usize,
          0xee9cb87usize,
          0xedcb87usize,
          0x2edcb87usize,
          0x4edcb87usize,
          0x6edcb87usize,
          0x8edcb87usize,
          0xaedcb87usize,
          0xcedcb87usize,
          0xeedcb87usize,
          0xf1cb87usize,
          0x2f1cb87usize,
          0x4f1cb87usize,
          0x6f1cb87usize,
          0x8f1cb87usize,
          0xaf1cb87usize,
          0xcf1cb87usize,
          0xef1cb87usize,
          0xf5cb87usize,
          0x2f5cb87usize,
          0x4f5cb87usize,
          0x6f5cb87usize,
          0x8f5cb87usize,
          0xaf5cb87usize,
          0xcf5cb87usize,
          0xef5cb87usize,
          0xf9cb87usize,
          0x2f9cb87usize,
          0x4f9cb87usize,
          0x6f9cb87usize,
          0x8f9cb87usize
      ];

static mut kZeroRepsDepth
    : [u32; 704]
    = [   0u32,
          4u32,
          8u32,
          7u32,
          7u32,
          7u32,
          7u32,
          7u32,
          7u32,
          7u32,
          7u32,
          11u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          14u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          21u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32,
          28u32
      ];

static mut kNonZeroRepsBits
    : [usize; 704]
    = [   0xbusize,
          0x1busize,
          0x2busize,
          0x3busize,
          0x2cbusize,
          0x6cbusize,
          0xacbusize,
          0xecbusize,
          0x2dbusize,
          0x6dbusize,
          0xadbusize,
          0xedbusize,
          0x2ebusize,
          0x6ebusize,
          0xaebusize,
          0xeebusize,
          0x2fbusize,
          0x6fbusize,
          0xafbusize,
          0xefbusize,
          0xb2cbusize,
          0x1b2cbusize,
          0x2b2cbusize,
          0x3b2cbusize,
          0xb6cbusize,
          0x1b6cbusize,
          0x2b6cbusize,
          0x3b6cbusize,
          0xbacbusize,
          0x1bacbusize,
          0x2bacbusize,
          0x3bacbusize,
          0xbecbusize,
          0x1becbusize,
          0x2becbusize,
          0x3becbusize,
          0xb2dbusize,
          0x1b2dbusize,
          0x2b2dbusize,
          0x3b2dbusize,
          0xb6dbusize,
          0x1b6dbusize,
          0x2b6dbusize,
          0x3b6dbusize,
          0xbadbusize,
          0x1badbusize,
          0x2badbusize,
          0x3badbusize,
          0xbedbusize,
          0x1bedbusize,
          0x2bedbusize,
          0x3bedbusize,
          0xb2ebusize,
          0x1b2ebusize,
          0x2b2ebusize,
          0x3b2ebusize,
          0xb6ebusize,
          0x1b6ebusize,
          0x2b6ebusize,
          0x3b6ebusize,
          0xbaebusize,
          0x1baebusize,
          0x2baebusize,
          0x3baebusize,
          0xbeebusize,
          0x1beebusize,
          0x2beebusize,
          0x3beebusize,
          0xb2fbusize,
          0x1b2fbusize,
          0x2b2fbusize,
          0x3b2fbusize,
          0xb6fbusize,
          0x1b6fbusize,
          0x2b6fbusize,
          0x3b6fbusize,
          0xbafbusize,
          0x1bafbusize,
          0x2bafbusize,
          0x3bafbusize,
          0xbefbusize,
          0x1befbusize,
          0x2befbusize,
          0x3befbusize,
          0x2cb2cbusize,
          0x6cb2cbusize,
          0xacb2cbusize,
          0xecb2cbusize,
          0x2db2cbusize,
          0x6db2cbusize,
          0xadb2cbusize,
          0xedb2cbusize,
          0x2eb2cbusize,
          0x6eb2cbusize,
          0xaeb2cbusize,
          0xeeb2cbusize,
          0x2fb2cbusize,
          0x6fb2cbusize,
          0xafb2cbusize,
          0xefb2cbusize,
          0x2cb6cbusize,
          0x6cb6cbusize,
          0xacb6cbusize,
          0xecb6cbusize,
          0x2db6cbusize,
          0x6db6cbusize,
          0xadb6cbusize,
          0xedb6cbusize,
          0x2eb6cbusize,
          0x6eb6cbusize,
          0xaeb6cbusize,
          0xeeb6cbusize,
          0x2fb6cbusize,
          0x6fb6cbusize,
          0xafb6cbusize,
          0xefb6cbusize,
          0x2cbacbusize,
          0x6cbacbusize,
          0xacbacbusize,
          0xecbacbusize,
          0x2dbacbusize,
          0x6dbacbusize,
          0xadbacbusize,
          0xedbacbusize,
          0x2ebacbusize,
          0x6ebacbusize,
          0xaebacbusize,
          0xeebacbusize,
          0x2fbacbusize,
          0x6fbacbusize,
          0xafbacbusize,
          0xefbacbusize,
          0x2cbecbusize,
          0x6cbecbusize,
          0xacbecbusize,
          0xecbecbusize,
          0x2dbecbusize,
          0x6dbecbusize,
          0xadbecbusize,
          0xedbecbusize,
          0x2ebecbusize,
          0x6ebecbusize,
          0xaebecbusize,
          0xeebecbusize,
          0x2fbecbusize,
          0x6fbecbusize,
          0xafbecbusize,
          0xefbecbusize,
          0x2cb2dbusize,
          0x6cb2dbusize,
          0xacb2dbusize,
          0xecb2dbusize,
          0x2db2dbusize,
          0x6db2dbusize,
          0xadb2dbusize,
          0xedb2dbusize,
          0x2eb2dbusize,
          0x6eb2dbusize,
          0xaeb2dbusize,
          0xeeb2dbusize,
          0x2fb2dbusize,
          0x6fb2dbusize,
          0xafb2dbusize,
          0xefb2dbusize,
          0x2cb6dbusize,
          0x6cb6dbusize,
          0xacb6dbusize,
          0xecb6dbusize,
          0x2db6dbusize,
          0x6db6dbusize,
          0xadb6dbusize,
          0xedb6dbusize,
          0x2eb6dbusize,
          0x6eb6dbusize,
          0xaeb6dbusize,
          0xeeb6dbusize,
          0x2fb6dbusize,
          0x6fb6dbusize,
          0xafb6dbusize,
          0xefb6dbusize,
          0x2cbadbusize,
          0x6cbadbusize,
          0xacbadbusize,
          0xecbadbusize,
          0x2dbadbusize,
          0x6dbadbusize,
          0xadbadbusize,
          0xedbadbusize,
          0x2ebadbusize,
          0x6ebadbusize,
          0xaebadbusize,
          0xeebadbusize,
          0x2fbadbusize,
          0x6fbadbusize,
          0xafbadbusize,
          0xefbadbusize,
          0x2cbedbusize,
          0x6cbedbusize,
          0xacbedbusize,
          0xecbedbusize,
          0x2dbedbusize,
          0x6dbedbusize,
          0xadbedbusize,
          0xedbedbusize,
          0x2ebedbusize,
          0x6ebedbusize,
          0xaebedbusize,
          0xeebedbusize,
          0x2fbedbusize,
          0x6fbedbusize,
          0xafbedbusize,
          0xefbedbusize,
          0x2cb2ebusize,
          0x6cb2ebusize,
          0xacb2ebusize,
          0xecb2ebusize,
          0x2db2ebusize,
          0x6db2ebusize,
          0xadb2ebusize,
          0xedb2ebusize,
          0x2eb2ebusize,
          0x6eb2ebusize,
          0xaeb2ebusize,
          0xeeb2ebusize,
          0x2fb2ebusize,
          0x6fb2ebusize,
          0xafb2ebusize,
          0xefb2ebusize,
          0x2cb6ebusize,
          0x6cb6ebusize,
          0xacb6ebusize,
          0xecb6ebusize,
          0x2db6ebusize,
          0x6db6ebusize,
          0xadb6ebusize,
          0xedb6ebusize,
          0x2eb6ebusize,
          0x6eb6ebusize,
          0xaeb6ebusize,
          0xeeb6ebusize,
          0x2fb6ebusize,
          0x6fb6ebusize,
          0xafb6ebusize,
          0xefb6ebusize,
          0x2cbaebusize,
          0x6cbaebusize,
          0xacbaebusize,
          0xecbaebusize,
          0x2dbaebusize,
          0x6dbaebusize,
          0xadbaebusize,
          0xedbaebusize,
          0x2ebaebusize,
          0x6ebaebusize,
          0xaebaebusize,
          0xeebaebusize,
          0x2fbaebusize,
          0x6fbaebusize,
          0xafbaebusize,
          0xefbaebusize,
          0x2cbeebusize,
          0x6cbeebusize,
          0xacbeebusize,
          0xecbeebusize,
          0x2dbeebusize,
          0x6dbeebusize,
          0xadbeebusize,
          0xedbeebusize,
          0x2ebeebusize,
          0x6ebeebusize,
          0xaebeebusize,
          0xeebeebusize,
          0x2fbeebusize,
          0x6fbeebusize,
          0xafbeebusize,
          0xefbeebusize,
          0x2cb2fbusize,
          0x6cb2fbusize,
          0xacb2fbusize,
          0xecb2fbusize,
          0x2db2fbusize,
          0x6db2fbusize,
          0xadb2fbusize,
          0xedb2fbusize,
          0x2eb2fbusize,
          0x6eb2fbusize,
          0xaeb2fbusize,
          0xeeb2fbusize,
          0x2fb2fbusize,
          0x6fb2fbusize,
          0xafb2fbusize,
          0xefb2fbusize,
          0x2cb6fbusize,
          0x6cb6fbusize,
          0xacb6fbusize,
          0xecb6fbusize,
          0x2db6fbusize,
          0x6db6fbusize,
          0xadb6fbusize,
          0xedb6fbusize,
          0x2eb6fbusize,
          0x6eb6fbusize,
          0xaeb6fbusize,
          0xeeb6fbusize,
          0x2fb6fbusize,
          0x6fb6fbusize,
          0xafb6fbusize,
          0xefb6fbusize,
          0x2cbafbusize,
          0x6cbafbusize,
          0xacbafbusize,
          0xecbafbusize,
          0x2dbafbusize,
          0x6dbafbusize,
          0xadbafbusize,
          0xedbafbusize,
          0x2ebafbusize,
          0x6ebafbusize,
          0xaebafbusize,
          0xeebafbusize,
          0x2fbafbusize,
          0x6fbafbusize,
          0xafbafbusize,
          0xefbafbusize,
          0x2cbefbusize,
          0x6cbefbusize,
          0xacbefbusize,
          0xecbefbusize,
          0x2dbefbusize,
          0x6dbefbusize,
          0xadbefbusize,
          0xedbefbusize,
          0x2ebefbusize,
          0x6ebefbusize,
          0xaebefbusize,
          0xeebefbusize,
          0x2fbefbusize,
          0x6fbefbusize,
          0xafbefbusize,
          0xefbefbusize,
          0xb2cb2cbusize,
          0x1b2cb2cbusize,
          0x2b2cb2cbusize,
          0x3b2cb2cbusize,
          0xb6cb2cbusize,
          0x1b6cb2cbusize,
          0x2b6cb2cbusize,
          0x3b6cb2cbusize,
          0xbacb2cbusize,
          0x1bacb2cbusize,
          0x2bacb2cbusize,
          0x3bacb2cbusize,
          0xbecb2cbusize,
          0x1becb2cbusize,
          0x2becb2cbusize,
          0x3becb2cbusize,
          0xb2db2cbusize,
          0x1b2db2cbusize,
          0x2b2db2cbusize,
          0x3b2db2cbusize,
          0xb6db2cbusize,
          0x1b6db2cbusize,
          0x2b6db2cbusize,
          0x3b6db2cbusize,
          0xbadb2cbusize,
          0x1badb2cbusize,
          0x2badb2cbusize,
          0x3badb2cbusize,
          0xbedb2cbusize,
          0x1bedb2cbusize,
          0x2bedb2cbusize,
          0x3bedb2cbusize,
          0xb2eb2cbusize,
          0x1b2eb2cbusize,
          0x2b2eb2cbusize,
          0x3b2eb2cbusize,
          0xb6eb2cbusize,
          0x1b6eb2cbusize,
          0x2b6eb2cbusize,
          0x3b6eb2cbusize,
          0xbaeb2cbusize,
          0x1baeb2cbusize,
          0x2baeb2cbusize,
          0x3baeb2cbusize,
          0xbeeb2cbusize,
          0x1beeb2cbusize,
          0x2beeb2cbusize,
          0x3beeb2cbusize,
          0xb2fb2cbusize,
          0x1b2fb2cbusize,
          0x2b2fb2cbusize,
          0x3b2fb2cbusize,
          0xb6fb2cbusize,
          0x1b6fb2cbusize,
          0x2b6fb2cbusize,
          0x3b6fb2cbusize,
          0xbafb2cbusize,
          0x1bafb2cbusize,
          0x2bafb2cbusize,
          0x3bafb2cbusize,
          0xbefb2cbusize,
          0x1befb2cbusize,
          0x2befb2cbusize,
          0x3befb2cbusize,
          0xb2cb6cbusize,
          0x1b2cb6cbusize,
          0x2b2cb6cbusize,
          0x3b2cb6cbusize,
          0xb6cb6cbusize,
          0x1b6cb6cbusize,
          0x2b6cb6cbusize,
          0x3b6cb6cbusize,
          0xbacb6cbusize,
          0x1bacb6cbusize,
          0x2bacb6cbusize,
          0x3bacb6cbusize,
          0xbecb6cbusize,
          0x1becb6cbusize,
          0x2becb6cbusize,
          0x3becb6cbusize,
          0xb2db6cbusize,
          0x1b2db6cbusize,
          0x2b2db6cbusize,
          0x3b2db6cbusize,
          0xb6db6cbusize,
          0x1b6db6cbusize,
          0x2b6db6cbusize,
          0x3b6db6cbusize,
          0xbadb6cbusize,
          0x1badb6cbusize,
          0x2badb6cbusize,
          0x3badb6cbusize,
          0xbedb6cbusize,
          0x1bedb6cbusize,
          0x2bedb6cbusize,
          0x3bedb6cbusize,
          0xb2eb6cbusize,
          0x1b2eb6cbusize,
          0x2b2eb6cbusize,
          0x3b2eb6cbusize,
          0xb6eb6cbusize,
          0x1b6eb6cbusize,
          0x2b6eb6cbusize,
          0x3b6eb6cbusize,
          0xbaeb6cbusize,
          0x1baeb6cbusize,
          0x2baeb6cbusize,
          0x3baeb6cbusize,
          0xbeeb6cbusize,
          0x1beeb6cbusize,
          0x2beeb6cbusize,
          0x3beeb6cbusize,
          0xb2fb6cbusize,
          0x1b2fb6cbusize,
          0x2b2fb6cbusize,
          0x3b2fb6cbusize,
          0xb6fb6cbusize,
          0x1b6fb6cbusize,
          0x2b6fb6cbusize,
          0x3b6fb6cbusize,
          0xbafb6cbusize,
          0x1bafb6cbusize,
          0x2bafb6cbusize,
          0x3bafb6cbusize,
          0xbefb6cbusize,
          0x1befb6cbusize,
          0x2befb6cbusize,
          0x3befb6cbusize,
          0xb2cbacbusize,
          0x1b2cbacbusize,
          0x2b2cbacbusize,
          0x3b2cbacbusize,
          0xb6cbacbusize,
          0x1b6cbacbusize,
          0x2b6cbacbusize,
          0x3b6cbacbusize,
          0xbacbacbusize,
          0x1bacbacbusize,
          0x2bacbacbusize,
          0x3bacbacbusize,
          0xbecbacbusize,
          0x1becbacbusize,
          0x2becbacbusize,
          0x3becbacbusize,
          0xb2dbacbusize,
          0x1b2dbacbusize,
          0x2b2dbacbusize,
          0x3b2dbacbusize,
          0xb6dbacbusize,
          0x1b6dbacbusize,
          0x2b6dbacbusize,
          0x3b6dbacbusize,
          0xbadbacbusize,
          0x1badbacbusize,
          0x2badbacbusize,
          0x3badbacbusize,
          0xbedbacbusize,
          0x1bedbacbusize,
          0x2bedbacbusize,
          0x3bedbacbusize,
          0xb2ebacbusize,
          0x1b2ebacbusize,
          0x2b2ebacbusize,
          0x3b2ebacbusize,
          0xb6ebacbusize,
          0x1b6ebacbusize,
          0x2b6ebacbusize,
          0x3b6ebacbusize,
          0xbaebacbusize,
          0x1baebacbusize,
          0x2baebacbusize,
          0x3baebacbusize,
          0xbeebacbusize,
          0x1beebacbusize,
          0x2beebacbusize,
          0x3beebacbusize,
          0xb2fbacbusize,
          0x1b2fbacbusize,
          0x2b2fbacbusize,
          0x3b2fbacbusize,
          0xb6fbacbusize,
          0x1b6fbacbusize,
          0x2b6fbacbusize,
          0x3b6fbacbusize,
          0xbafbacbusize,
          0x1bafbacbusize,
          0x2bafbacbusize,
          0x3bafbacbusize,
          0xbefbacbusize,
          0x1befbacbusize,
          0x2befbacbusize,
          0x3befbacbusize,
          0xb2cbecbusize,
          0x1b2cbecbusize,
          0x2b2cbecbusize,
          0x3b2cbecbusize,
          0xb6cbecbusize,
          0x1b6cbecbusize,
          0x2b6cbecbusize,
          0x3b6cbecbusize,
          0xbacbecbusize,
          0x1bacbecbusize,
          0x2bacbecbusize,
          0x3bacbecbusize,
          0xbecbecbusize,
          0x1becbecbusize,
          0x2becbecbusize,
          0x3becbecbusize,
          0xb2dbecbusize,
          0x1b2dbecbusize,
          0x2b2dbecbusize,
          0x3b2dbecbusize,
          0xb6dbecbusize,
          0x1b6dbecbusize,
          0x2b6dbecbusize,
          0x3b6dbecbusize,
          0xbadbecbusize,
          0x1badbecbusize,
          0x2badbecbusize,
          0x3badbecbusize,
          0xbedbecbusize,
          0x1bedbecbusize,
          0x2bedbecbusize,
          0x3bedbecbusize,
          0xb2ebecbusize,
          0x1b2ebecbusize,
          0x2b2ebecbusize,
          0x3b2ebecbusize,
          0xb6ebecbusize,
          0x1b6ebecbusize,
          0x2b6ebecbusize,
          0x3b6ebecbusize,
          0xbaebecbusize,
          0x1baebecbusize,
          0x2baebecbusize,
          0x3baebecbusize,
          0xbeebecbusize,
          0x1beebecbusize,
          0x2beebecbusize,
          0x3beebecbusize,
          0xb2fbecbusize,
          0x1b2fbecbusize,
          0x2b2fbecbusize,
          0x3b2fbecbusize,
          0xb6fbecbusize,
          0x1b6fbecbusize,
          0x2b6fbecbusize,
          0x3b6fbecbusize,
          0xbafbecbusize,
          0x1bafbecbusize,
          0x2bafbecbusize,
          0x3bafbecbusize,
          0xbefbecbusize,
          0x1befbecbusize,
          0x2befbecbusize,
          0x3befbecbusize,
          0xb2cb2dbusize,
          0x1b2cb2dbusize,
          0x2b2cb2dbusize,
          0x3b2cb2dbusize,
          0xb6cb2dbusize,
          0x1b6cb2dbusize,
          0x2b6cb2dbusize,
          0x3b6cb2dbusize,
          0xbacb2dbusize,
          0x1bacb2dbusize,
          0x2bacb2dbusize,
          0x3bacb2dbusize,
          0xbecb2dbusize,
          0x1becb2dbusize,
          0x2becb2dbusize,
          0x3becb2dbusize,
          0xb2db2dbusize,
          0x1b2db2dbusize,
          0x2b2db2dbusize,
          0x3b2db2dbusize,
          0xb6db2dbusize,
          0x1b6db2dbusize,
          0x2b6db2dbusize,
          0x3b6db2dbusize,
          0xbadb2dbusize,
          0x1badb2dbusize,
          0x2badb2dbusize,
          0x3badb2dbusize,
          0xbedb2dbusize,
          0x1bedb2dbusize,
          0x2bedb2dbusize,
          0x3bedb2dbusize,
          0xb2eb2dbusize,
          0x1b2eb2dbusize,
          0x2b2eb2dbusize,
          0x3b2eb2dbusize,
          0xb6eb2dbusize,
          0x1b6eb2dbusize,
          0x2b6eb2dbusize,
          0x3b6eb2dbusize,
          0xbaeb2dbusize,
          0x1baeb2dbusize,
          0x2baeb2dbusize,
          0x3baeb2dbusize,
          0xbeeb2dbusize,
          0x1beeb2dbusize,
          0x2beeb2dbusize,
          0x3beeb2dbusize,
          0xb2fb2dbusize,
          0x1b2fb2dbusize,
          0x2b2fb2dbusize,
          0x3b2fb2dbusize,
          0xb6fb2dbusize,
          0x1b6fb2dbusize,
          0x2b6fb2dbusize,
          0x3b6fb2dbusize,
          0xbafb2dbusize,
          0x1bafb2dbusize,
          0x2bafb2dbusize,
          0x3bafb2dbusize,
          0xbefb2dbusize,
          0x1befb2dbusize,
          0x2befb2dbusize,
          0x3befb2dbusize,
          0xb2cb6dbusize,
          0x1b2cb6dbusize,
          0x2b2cb6dbusize,
          0x3b2cb6dbusize,
          0xb6cb6dbusize,
          0x1b6cb6dbusize,
          0x2b6cb6dbusize,
          0x3b6cb6dbusize,
          0xbacb6dbusize,
          0x1bacb6dbusize,
          0x2bacb6dbusize,
          0x3bacb6dbusize,
          0xbecb6dbusize,
          0x1becb6dbusize,
          0x2becb6dbusize,
          0x3becb6dbusize,
          0xb2db6dbusize,
          0x1b2db6dbusize,
          0x2b2db6dbusize,
          0x3b2db6dbusize,
          0xb6db6dbusize,
          0x1b6db6dbusize,
          0x2b6db6dbusize,
          0x3b6db6dbusize,
          0xbadb6dbusize,
          0x1badb6dbusize,
          0x2badb6dbusize,
          0x3badb6dbusize,
          0xbedb6dbusize,
          0x1bedb6dbusize,
          0x2bedb6dbusize,
          0x3bedb6dbusize,
          0xb2eb6dbusize,
          0x1b2eb6dbusize,
          0x2b2eb6dbusize,
          0x3b2eb6dbusize,
          0xb6eb6dbusize,
          0x1b6eb6dbusize,
          0x2b6eb6dbusize,
          0x3b6eb6dbusize,
          0xbaeb6dbusize,
          0x1baeb6dbusize,
          0x2baeb6dbusize,
          0x3baeb6dbusize
      ];

static mut kNonZeroRepsDepth
    : [u32; 704]
    = [   6u32,
          6u32,
          6u32,
          6u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          12u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          18u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          24u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32,
          30u32
      ];

static mut kStaticCommandCodeBits
    : [u16; 704]
    = [   0i32 as (u16),
          256i32 as (u16),
          128i32 as (u16),
          384i32 as (u16),
          64i32 as (u16),
          320i32 as (u16),
          192i32 as (u16),
          448i32 as (u16),
          32i32 as (u16),
          288i32 as (u16),
          160i32 as (u16),
          416i32 as (u16),
          96i32 as (u16),
          352i32 as (u16),
          224i32 as (u16),
          480i32 as (u16),
          16i32 as (u16),
          272i32 as (u16),
          144i32 as (u16),
          400i32 as (u16),
          80i32 as (u16),
          336i32 as (u16),
          208i32 as (u16),
          464i32 as (u16),
          48i32 as (u16),
          304i32 as (u16),
          176i32 as (u16),
          432i32 as (u16),
          112i32 as (u16),
          368i32 as (u16),
          240i32 as (u16),
          496i32 as (u16),
          8i32 as (u16),
          264i32 as (u16),
          136i32 as (u16),
          392i32 as (u16),
          72i32 as (u16),
          328i32 as (u16),
          200i32 as (u16),
          456i32 as (u16),
          40i32 as (u16),
          296i32 as (u16),
          168i32 as (u16),
          424i32 as (u16),
          104i32 as (u16),
          360i32 as (u16),
          232i32 as (u16),
          488i32 as (u16),
          24i32 as (u16),
          280i32 as (u16),
          152i32 as (u16),
          408i32 as (u16),
          88i32 as (u16),
          344i32 as (u16),
          216i32 as (u16),
          472i32 as (u16),
          56i32 as (u16),
          312i32 as (u16),
          184i32 as (u16),
          440i32 as (u16),
          120i32 as (u16),
          376i32 as (u16),
          248i32 as (u16),
          504i32 as (u16),
          4i32 as (u16),
          260i32 as (u16),
          132i32 as (u16),
          388i32 as (u16),
          68i32 as (u16),
          324i32 as (u16),
          196i32 as (u16),
          452i32 as (u16),
          36i32 as (u16),
          292i32 as (u16),
          164i32 as (u16),
          420i32 as (u16),
          100i32 as (u16),
          356i32 as (u16),
          228i32 as (u16),
          484i32 as (u16),
          20i32 as (u16),
          276i32 as (u16),
          148i32 as (u16),
          404i32 as (u16),
          84i32 as (u16),
          340i32 as (u16),
          212i32 as (u16),
          468i32 as (u16),
          52i32 as (u16),
          308i32 as (u16),
          180i32 as (u16),
          436i32 as (u16),
          116i32 as (u16),
          372i32 as (u16),
          244i32 as (u16),
          500i32 as (u16),
          12i32 as (u16),
          268i32 as (u16),
          140i32 as (u16),
          396i32 as (u16),
          76i32 as (u16),
          332i32 as (u16),
          204i32 as (u16),
          460i32 as (u16),
          44i32 as (u16),
          300i32 as (u16),
          172i32 as (u16),
          428i32 as (u16),
          108i32 as (u16),
          364i32 as (u16),
          236i32 as (u16),
          492i32 as (u16),
          28i32 as (u16),
          284i32 as (u16),
          156i32 as (u16),
          412i32 as (u16),
          92i32 as (u16),
          348i32 as (u16),
          220i32 as (u16),
          476i32 as (u16),
          60i32 as (u16),
          316i32 as (u16),
          188i32 as (u16),
          444i32 as (u16),
          124i32 as (u16),
          380i32 as (u16),
          252i32 as (u16),
          508i32 as (u16),
          2i32 as (u16),
          258i32 as (u16),
          130i32 as (u16),
          386i32 as (u16),
          66i32 as (u16),
          322i32 as (u16),
          194i32 as (u16),
          450i32 as (u16),
          34i32 as (u16),
          290i32 as (u16),
          162i32 as (u16),
          418i32 as (u16),
          98i32 as (u16),
          354i32 as (u16),
          226i32 as (u16),
          482i32 as (u16),
          18i32 as (u16),
          274i32 as (u16),
          146i32 as (u16),
          402i32 as (u16),
          82i32 as (u16),
          338i32 as (u16),
          210i32 as (u16),
          466i32 as (u16),
          50i32 as (u16),
          306i32 as (u16),
          178i32 as (u16),
          434i32 as (u16),
          114i32 as (u16),
          370i32 as (u16),
          242i32 as (u16),
          498i32 as (u16),
          10i32 as (u16),
          266i32 as (u16),
          138i32 as (u16),
          394i32 as (u16),
          74i32 as (u16),
          330i32 as (u16),
          202i32 as (u16),
          458i32 as (u16),
          42i32 as (u16),
          298i32 as (u16),
          170i32 as (u16),
          426i32 as (u16),
          106i32 as (u16),
          362i32 as (u16),
          234i32 as (u16),
          490i32 as (u16),
          26i32 as (u16),
          282i32 as (u16),
          154i32 as (u16),
          410i32 as (u16),
          90i32 as (u16),
          346i32 as (u16),
          218i32 as (u16),
          474i32 as (u16),
          58i32 as (u16),
          314i32 as (u16),
          186i32 as (u16),
          442i32 as (u16),
          122i32 as (u16),
          378i32 as (u16),
          250i32 as (u16),
          506i32 as (u16),
          6i32 as (u16),
          262i32 as (u16),
          134i32 as (u16),
          390i32 as (u16),
          70i32 as (u16),
          326i32 as (u16),
          198i32 as (u16),
          454i32 as (u16),
          38i32 as (u16),
          294i32 as (u16),
          166i32 as (u16),
          422i32 as (u16),
          102i32 as (u16),
          358i32 as (u16),
          230i32 as (u16),
          486i32 as (u16),
          22i32 as (u16),
          278i32 as (u16),
          150i32 as (u16),
          406i32 as (u16),
          86i32 as (u16),
          342i32 as (u16),
          214i32 as (u16),
          470i32 as (u16),
          54i32 as (u16),
          310i32 as (u16),
          182i32 as (u16),
          438i32 as (u16),
          118i32 as (u16),
          374i32 as (u16),
          246i32 as (u16),
          502i32 as (u16),
          14i32 as (u16),
          270i32 as (u16),
          142i32 as (u16),
          398i32 as (u16),
          78i32 as (u16),
          334i32 as (u16),
          206i32 as (u16),
          462i32 as (u16),
          46i32 as (u16),
          302i32 as (u16),
          174i32 as (u16),
          430i32 as (u16),
          110i32 as (u16),
          366i32 as (u16),
          238i32 as (u16),
          494i32 as (u16),
          30i32 as (u16),
          286i32 as (u16),
          158i32 as (u16),
          414i32 as (u16),
          94i32 as (u16),
          350i32 as (u16),
          222i32 as (u16),
          478i32 as (u16),
          62i32 as (u16),
          318i32 as (u16),
          190i32 as (u16),
          446i32 as (u16),
          126i32 as (u16),
          382i32 as (u16),
          254i32 as (u16),
          510i32 as (u16),
          1i32 as (u16),
          257i32 as (u16),
          129i32 as (u16),
          385i32 as (u16),
          65i32 as (u16),
          321i32 as (u16),
          193i32 as (u16),
          449i32 as (u16),
          33i32 as (u16),
          289i32 as (u16),
          161i32 as (u16),
          417i32 as (u16),
          97i32 as (u16),
          353i32 as (u16),
          225i32 as (u16),
          481i32 as (u16),
          17i32 as (u16),
          273i32 as (u16),
          145i32 as (u16),
          401i32 as (u16),
          81i32 as (u16),
          337i32 as (u16),
          209i32 as (u16),
          465i32 as (u16),
          49i32 as (u16),
          305i32 as (u16),
          177i32 as (u16),
          433i32 as (u16),
          113i32 as (u16),
          369i32 as (u16),
          241i32 as (u16),
          497i32 as (u16),
          9i32 as (u16),
          265i32 as (u16),
          137i32 as (u16),
          393i32 as (u16),
          73i32 as (u16),
          329i32 as (u16),
          201i32 as (u16),
          457i32 as (u16),
          41i32 as (u16),
          297i32 as (u16),
          169i32 as (u16),
          425i32 as (u16),
          105i32 as (u16),
          361i32 as (u16),
          233i32 as (u16),
          489i32 as (u16),
          25i32 as (u16),
          281i32 as (u16),
          153i32 as (u16),
          409i32 as (u16),
          89i32 as (u16),
          345i32 as (u16),
          217i32 as (u16),
          473i32 as (u16),
          57i32 as (u16),
          313i32 as (u16),
          185i32 as (u16),
          441i32 as (u16),
          121i32 as (u16),
          377i32 as (u16),
          249i32 as (u16),
          505i32 as (u16),
          5i32 as (u16),
          261i32 as (u16),
          133i32 as (u16),
          389i32 as (u16),
          69i32 as (u16),
          325i32 as (u16),
          197i32 as (u16),
          453i32 as (u16),
          37i32 as (u16),
          293i32 as (u16),
          165i32 as (u16),
          421i32 as (u16),
          101i32 as (u16),
          357i32 as (u16),
          229i32 as (u16),
          485i32 as (u16),
          21i32 as (u16),
          277i32 as (u16),
          149i32 as (u16),
          405i32 as (u16),
          85i32 as (u16),
          341i32 as (u16),
          213i32 as (u16),
          469i32 as (u16),
          53i32 as (u16),
          309i32 as (u16),
          181i32 as (u16),
          437i32 as (u16),
          117i32 as (u16),
          373i32 as (u16),
          245i32 as (u16),
          501i32 as (u16),
          13i32 as (u16),
          269i32 as (u16),
          141i32 as (u16),
          397i32 as (u16),
          77i32 as (u16),
          333i32 as (u16),
          205i32 as (u16),
          461i32 as (u16),
          45i32 as (u16),
          301i32 as (u16),
          173i32 as (u16),
          429i32 as (u16),
          109i32 as (u16),
          365i32 as (u16),
          237i32 as (u16),
          493i32 as (u16),
          29i32 as (u16),
          285i32 as (u16),
          157i32 as (u16),
          413i32 as (u16),
          93i32 as (u16),
          349i32 as (u16),
          221i32 as (u16),
          477i32 as (u16),
          61i32 as (u16),
          317i32 as (u16),
          189i32 as (u16),
          445i32 as (u16),
          125i32 as (u16),
          381i32 as (u16),
          253i32 as (u16),
          509i32 as (u16),
          3i32 as (u16),
          259i32 as (u16),
          131i32 as (u16),
          387i32 as (u16),
          67i32 as (u16),
          323i32 as (u16),
          195i32 as (u16),
          451i32 as (u16),
          35i32 as (u16),
          291i32 as (u16),
          163i32 as (u16),
          419i32 as (u16),
          99i32 as (u16),
          355i32 as (u16),
          227i32 as (u16),
          483i32 as (u16),
          19i32 as (u16),
          275i32 as (u16),
          147i32 as (u16),
          403i32 as (u16),
          83i32 as (u16),
          339i32 as (u16),
          211i32 as (u16),
          467i32 as (u16),
          51i32 as (u16),
          307i32 as (u16),
          179i32 as (u16),
          435i32 as (u16),
          115i32 as (u16),
          371i32 as (u16),
          243i32 as (u16),
          499i32 as (u16),
          11i32 as (u16),
          267i32 as (u16),
          139i32 as (u16),
          395i32 as (u16),
          75i32 as (u16),
          331i32 as (u16),
          203i32 as (u16),
          459i32 as (u16),
          43i32 as (u16),
          299i32 as (u16),
          171i32 as (u16),
          427i32 as (u16),
          107i32 as (u16),
          363i32 as (u16),
          235i32 as (u16),
          491i32 as (u16),
          27i32 as (u16),
          283i32 as (u16),
          155i32 as (u16),
          411i32 as (u16),
          91i32 as (u16),
          347i32 as (u16),
          219i32 as (u16),
          475i32 as (u16),
          59i32 as (u16),
          315i32 as (u16),
          187i32 as (u16),
          443i32 as (u16),
          123i32 as (u16),
          379i32 as (u16),
          251i32 as (u16),
          507i32 as (u16),
          7i32 as (u16),
          1031i32 as (u16),
          519i32 as (u16),
          1543i32 as (u16),
          263i32 as (u16),
          1287i32 as (u16),
          775i32 as (u16),
          1799i32 as (u16),
          135i32 as (u16),
          1159i32 as (u16),
          647i32 as (u16),
          1671i32 as (u16),
          391i32 as (u16),
          1415i32 as (u16),
          903i32 as (u16),
          1927i32 as (u16),
          71i32 as (u16),
          1095i32 as (u16),
          583i32 as (u16),
          1607i32 as (u16),
          327i32 as (u16),
          1351i32 as (u16),
          839i32 as (u16),
          1863i32 as (u16),
          199i32 as (u16),
          1223i32 as (u16),
          711i32 as (u16),
          1735i32 as (u16),
          455i32 as (u16),
          1479i32 as (u16),
          967i32 as (u16),
          1991i32 as (u16),
          39i32 as (u16),
          1063i32 as (u16),
          551i32 as (u16),
          1575i32 as (u16),
          295i32 as (u16),
          1319i32 as (u16),
          807i32 as (u16),
          1831i32 as (u16),
          167i32 as (u16),
          1191i32 as (u16),
          679i32 as (u16),
          1703i32 as (u16),
          423i32 as (u16),
          1447i32 as (u16),
          935i32 as (u16),
          1959i32 as (u16),
          103i32 as (u16),
          1127i32 as (u16),
          615i32 as (u16),
          1639i32 as (u16),
          359i32 as (u16),
          1383i32 as (u16),
          871i32 as (u16),
          1895i32 as (u16),
          231i32 as (u16),
          1255i32 as (u16),
          743i32 as (u16),
          1767i32 as (u16),
          487i32 as (u16),
          1511i32 as (u16),
          999i32 as (u16),
          2023i32 as (u16),
          23i32 as (u16),
          1047i32 as (u16),
          535i32 as (u16),
          1559i32 as (u16),
          279i32 as (u16),
          1303i32 as (u16),
          791i32 as (u16),
          1815i32 as (u16),
          151i32 as (u16),
          1175i32 as (u16),
          663i32 as (u16),
          1687i32 as (u16),
          407i32 as (u16),
          1431i32 as (u16),
          919i32 as (u16),
          1943i32 as (u16),
          87i32 as (u16),
          1111i32 as (u16),
          599i32 as (u16),
          1623i32 as (u16),
          343i32 as (u16),
          1367i32 as (u16),
          855i32 as (u16),
          1879i32 as (u16),
          215i32 as (u16),
          1239i32 as (u16),
          727i32 as (u16),
          1751i32 as (u16),
          471i32 as (u16),
          1495i32 as (u16),
          983i32 as (u16),
          2007i32 as (u16),
          55i32 as (u16),
          1079i32 as (u16),
          567i32 as (u16),
          1591i32 as (u16),
          311i32 as (u16),
          1335i32 as (u16),
          823i32 as (u16),
          1847i32 as (u16),
          183i32 as (u16),
          1207i32 as (u16),
          695i32 as (u16),
          1719i32 as (u16),
          439i32 as (u16),
          1463i32 as (u16),
          951i32 as (u16),
          1975i32 as (u16),
          119i32 as (u16),
          1143i32 as (u16),
          631i32 as (u16),
          1655i32 as (u16),
          375i32 as (u16),
          1399i32 as (u16),
          887i32 as (u16),
          1911i32 as (u16),
          247i32 as (u16),
          1271i32 as (u16),
          759i32 as (u16),
          1783i32 as (u16),
          503i32 as (u16),
          1527i32 as (u16),
          1015i32 as (u16),
          2039i32 as (u16),
          15i32 as (u16),
          1039i32 as (u16),
          527i32 as (u16),
          1551i32 as (u16),
          271i32 as (u16),
          1295i32 as (u16),
          783i32 as (u16),
          1807i32 as (u16),
          143i32 as (u16),
          1167i32 as (u16),
          655i32 as (u16),
          1679i32 as (u16),
          399i32 as (u16),
          1423i32 as (u16),
          911i32 as (u16),
          1935i32 as (u16),
          79i32 as (u16),
          1103i32 as (u16),
          591i32 as (u16),
          1615i32 as (u16),
          335i32 as (u16),
          1359i32 as (u16),
          847i32 as (u16),
          1871i32 as (u16),
          207i32 as (u16),
          1231i32 as (u16),
          719i32 as (u16),
          1743i32 as (u16),
          463i32 as (u16),
          1487i32 as (u16),
          975i32 as (u16),
          1999i32 as (u16),
          47i32 as (u16),
          1071i32 as (u16),
          559i32 as (u16),
          1583i32 as (u16),
          303i32 as (u16),
          1327i32 as (u16),
          815i32 as (u16),
          1839i32 as (u16),
          175i32 as (u16),
          1199i32 as (u16),
          687i32 as (u16),
          1711i32 as (u16),
          431i32 as (u16),
          1455i32 as (u16),
          943i32 as (u16),
          1967i32 as (u16),
          111i32 as (u16),
          1135i32 as (u16),
          623i32 as (u16),
          1647i32 as (u16),
          367i32 as (u16),
          1391i32 as (u16),
          879i32 as (u16),
          1903i32 as (u16),
          239i32 as (u16),
          1263i32 as (u16),
          751i32 as (u16),
          1775i32 as (u16),
          495i32 as (u16),
          1519i32 as (u16),
          1007i32 as (u16),
          2031i32 as (u16),
          31i32 as (u16),
          1055i32 as (u16),
          543i32 as (u16),
          1567i32 as (u16),
          287i32 as (u16),
          1311i32 as (u16),
          799i32 as (u16),
          1823i32 as (u16),
          159i32 as (u16),
          1183i32 as (u16),
          671i32 as (u16),
          1695i32 as (u16),
          415i32 as (u16),
          1439i32 as (u16),
          927i32 as (u16),
          1951i32 as (u16),
          95i32 as (u16),
          1119i32 as (u16),
          607i32 as (u16),
          1631i32 as (u16),
          351i32 as (u16),
          1375i32 as (u16),
          863i32 as (u16),
          1887i32 as (u16),
          223i32 as (u16),
          1247i32 as (u16),
          735i32 as (u16),
          1759i32 as (u16),
          479i32 as (u16),
          1503i32 as (u16),
          991i32 as (u16),
          2015i32 as (u16),
          63i32 as (u16),
          1087i32 as (u16),
          575i32 as (u16),
          1599i32 as (u16),
          319i32 as (u16),
          1343i32 as (u16),
          831i32 as (u16),
          1855i32 as (u16),
          191i32 as (u16),
          1215i32 as (u16),
          703i32 as (u16),
          1727i32 as (u16),
          447i32 as (u16),
          1471i32 as (u16),
          959i32 as (u16),
          1983i32 as (u16),
          127i32 as (u16),
          1151i32 as (u16),
          639i32 as (u16),
          1663i32 as (u16),
          383i32 as (u16),
          1407i32 as (u16),
          895i32 as (u16),
          1919i32 as (u16),
          255i32 as (u16),
          1279i32 as (u16),
          767i32 as (u16),
          1791i32 as (u16),
          511i32 as (u16),
          1535i32 as (u16),
          1023i32 as (u16),
          2047i32 as (u16)
      ];

static mut kStaticDistanceCodeBits
    : [u16; 64]
    = [   0i32 as (u16),
          32i32 as (u16),
          16i32 as (u16),
          48i32 as (u16),
          8i32 as (u16),
          40i32 as (u16),
          24i32 as (u16),
          56i32 as (u16),
          4i32 as (u16),
          36i32 as (u16),
          20i32 as (u16),
          52i32 as (u16),
          12i32 as (u16),
          44i32 as (u16),
          28i32 as (u16),
          60i32 as (u16),
          2i32 as (u16),
          34i32 as (u16),
          18i32 as (u16),
          50i32 as (u16),
          10i32 as (u16),
          42i32 as (u16),
          26i32 as (u16),
          58i32 as (u16),
          6i32 as (u16),
          38i32 as (u16),
          22i32 as (u16),
          54i32 as (u16),
          14i32 as (u16),
          46i32 as (u16),
          30i32 as (u16),
          62i32 as (u16),
          1i32 as (u16),
          33i32 as (u16),
          17i32 as (u16),
          49i32 as (u16),
          9i32 as (u16),
          41i32 as (u16),
          25i32 as (u16),
          57i32 as (u16),
          5i32 as (u16),
          37i32 as (u16),
          21i32 as (u16),
          53i32 as (u16),
          13i32 as (u16),
          45i32 as (u16),
          29i32 as (u16),
          61i32 as (u16),
          3i32 as (u16),
          35i32 as (u16),
          19i32 as (u16),
          51i32 as (u16),
          11i32 as (u16),
          43i32 as (u16),
          27i32 as (u16),
          59i32 as (u16),
          7i32 as (u16),
          39i32 as (u16),
          23i32 as (u16),
          55i32 as (u16),
          15i32 as (u16),
          47i32 as (u16),
          31i32 as (u16),
          63i32 as (u16)
      ];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PrefixCodeRange {
    pub offset : u32,
    pub nbits : u32,
}

static mut kBlockLengthPrefixCode
    : [PrefixCodeRange; 26]
    = [   PrefixCodeRange {
              offset: 1u32,
              nbits: 2u32
          },
          PrefixCodeRange { offset: 5u32, nbits: 2u32 },
          PrefixCodeRange { offset: 9u32, nbits: 2u32 },
          PrefixCodeRange { offset: 13u32, nbits: 2u32 },
          PrefixCodeRange { offset: 17u32, nbits: 3u32 },
          PrefixCodeRange { offset: 25u32, nbits: 3u32 },
          PrefixCodeRange { offset: 33u32, nbits: 3u32 },
          PrefixCodeRange { offset: 41u32, nbits: 3u32 },
          PrefixCodeRange { offset: 49u32, nbits: 4u32 },
          PrefixCodeRange { offset: 65u32, nbits: 4u32 },
          PrefixCodeRange { offset: 81u32, nbits: 4u32 },
          PrefixCodeRange { offset: 97u32, nbits: 4u32 },
          PrefixCodeRange { offset: 113u32, nbits: 5u32 },
          PrefixCodeRange { offset: 145u32, nbits: 5u32 },
          PrefixCodeRange { offset: 177u32, nbits: 5u32 },
          PrefixCodeRange { offset: 209u32, nbits: 5u32 },
          PrefixCodeRange { offset: 241u32, nbits: 6u32 },
          PrefixCodeRange { offset: 305u32, nbits: 6u32 },
          PrefixCodeRange { offset: 369u32, nbits: 7u32 },
          PrefixCodeRange { offset: 497u32, nbits: 8u32 },
          PrefixCodeRange { offset: 753u32, nbits: 9u32 },
          PrefixCodeRange {
              offset: 1265u32,
              nbits: 10u32
          },
          PrefixCodeRange {
              offset: 2289u32,
              nbits: 11u32
          },
          PrefixCodeRange {
              offset: 4337u32,
              nbits: 12u32
          },
          PrefixCodeRange {
              offset: 8433u32,
              nbits: 13u32
          },
          PrefixCodeRange {
              offset: 16625u32,
              nbits: 24u32
          }
      ];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HuffmanTree {
    pub total_count_ : u32,
    pub index_left_ : i16,
    pub index_right_or_value_ : i16,
}

unsafe extern fn BROTLI_UNALIGNED_STORE64(
    mut p : *mut ::std::os::raw::c_void, mut v : usize
) {
    memcpy(
        p,
        &mut v as (*mut usize) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<usize>()
    );
}

unsafe extern fn BrotliWriteBits(
    mut n_bits : usize,
    mut bits : usize,
    mut pos : *mut usize,
    mut array : *mut u8
) {
    let mut p
        : *mut u8
        = &mut *array.offset((*pos >> 3i32) as (isize)) as (*mut u8);
    let mut v : usize = *p as (usize);
    if bits >> n_bits == 0usize {
        0i32;
    } else {
        __assert_fail(
            (*b"(bits >> n_bits) == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliWriteBits\0").as_ptr()
        );
    }
    if n_bits <= 56usize {
        0i32;
    } else {
        __assert_fail(
            (*b"n_bits <= 56\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliWriteBits\0").as_ptr()
        );
    }
    v = v | bits << (*pos & 7usize);
    BROTLI_UNALIGNED_STORE64(p as (*mut ::std::os::raw::c_void),v);
    *pos = (*pos).wrapping_add(n_bits);
}

unsafe extern fn BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
    num_codes : i32,
    mut code_length_bitdepth : *const u8,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    static mut kStorageOrder
        : [u8; 18]
        = [   1i32 as (u8),
              2i32 as (u8),
              3i32 as (u8),
              4i32 as (u8),
              0i32 as (u8),
              5i32 as (u8),
              17i32 as (u8),
              6i32 as (u8),
              16i32 as (u8),
              7i32 as (u8),
              8i32 as (u8),
              9i32 as (u8),
              10i32 as (u8),
              11i32 as (u8),
              12i32 as (u8),
              13i32 as (u8),
              14i32 as (u8),
              15i32 as (u8)
          ];
    static mut kHuffmanBitLengthHuffmanCodeSymbols
        : [u8; 6]
        = [   0i32 as (u8),
              7i32 as (u8),
              3i32 as (u8),
              2i32 as (u8),
              1i32 as (u8),
              15i32 as (u8)
          ];
    static mut kHuffmanBitLengthHuffmanCodeBitLengths
        : [u8; 6]
        = [   2i32 as (u8),
              4i32 as (u8),
              3i32 as (u8),
              2i32 as (u8),
              2i32 as (u8),
              4i32 as (u8)
          ];
    let mut skip_some : usize = 0usize;
    let mut codes_to_store : usize = 18usize;
    if num_codes > 1i32 {
        'loop1: loop {
            if codes_to_store > 0usize {
                if *code_length_bitdepth.offset(
                        kStorageOrder[
                            codes_to_store.wrapping_sub(1usize)
                        ] as (isize)
                    ) as (i32) != 0i32 {
                    break 'loop1;
                } else {
                    codes_to_store = codes_to_store.wrapping_sub(1 as (usize));
                    continue 'loop1;
                }
            } else {
                break 'loop1;
            }
        }
    }
    if *code_length_bitdepth.offset(
            kStorageOrder[0usize] as (isize)
        ) as (i32) == 0i32 && (*code_length_bitdepth.offset(
                                    kStorageOrder[1usize] as (isize)
                                ) as (i32) == 0i32) {
        skip_some = 2usize;
        if *code_length_bitdepth.offset(
                kStorageOrder[2usize] as (isize)
            ) as (i32) == 0i32 {
            skip_some = 3usize;
        }
    }
    BrotliWriteBits(2usize,skip_some,storage_ix,storage);
    let mut i : usize;
    i = skip_some;
    'loop8: loop {
        if i < codes_to_store {
            let mut l
                : usize
                = *code_length_bitdepth.offset(
                       kStorageOrder[i] as (isize)
                   ) as (usize);
            BrotliWriteBits(
                kHuffmanBitLengthHuffmanCodeBitLengths[l] as (usize),
                kHuffmanBitLengthHuffmanCodeSymbols[l] as (usize),
                storage_ix,
                storage
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop8;
        } else {
            break 'loop8;
        }
    }
}

unsafe extern fn BrotliStoreHuffmanTreeToBitMask(
    huffman_tree_size : usize,
    mut huffman_tree : *const u8,
    mut huffman_tree_extra_bits : *const u8,
    mut code_length_bitdepth : *const u8,
    mut code_length_bitdepth_symbols : *const u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < huffman_tree_size {
            let mut ix : usize = *huffman_tree.offset(i as (isize)) as (usize);
            BrotliWriteBits(
                *code_length_bitdepth.offset(ix as (isize)) as (usize),
                *code_length_bitdepth_symbols.offset(ix as (isize)) as (usize),
                storage_ix,
                storage
            );
            if ix == 17usize {
                BrotliWriteBits(
                    3usize,
                    *huffman_tree_extra_bits.offset(i as (isize)) as (usize),
                    storage_ix,
                    storage
                );
            } else if ix == 16usize {
                BrotliWriteBits(
                    2usize,
                    *huffman_tree_extra_bits.offset(i as (isize)) as (usize),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}
// TODO
#[no_mangle]
pub unsafe extern fn BrotliStoreHuffmanTree(
    mut depths : *const u8,
    mut num : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut huffman_tree : [u8; 704];
    let mut huffman_tree_extra_bits : [u8; 704];
    let mut huffman_tree_size : usize = 0usize;
    let mut code_length_bitdepth
        : [u8; 18]
        = [   0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8),
              0i32 as (u8)
          ];
    let mut code_length_bitdepth_symbols : [u16; 18];
    let mut huffman_tree_histogram
        : [u32; 18]
        = [   0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32,
              0u32
          ];
    let mut i : usize;
    let mut num_codes : i32 = 0i32;
    let mut code : usize = 0usize;
    if num <= 704usize {
        0i32;
    } else {
        __assert_fail(
            (*b"num <= BROTLI_NUM_COMMAND_SYMBOLS\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliStoreHuffmanTree\0").as_ptr()
        );
    }
    BrotliWriteHuffmanTree(
        depths,
        num,
        &mut huffman_tree_size as (*mut usize),
        huffman_tree.as_mut_ptr(),
        huffman_tree_extra_bits.as_mut_ptr()
    );
    i = 0usize;
    'loop1: loop {
        if i < huffman_tree_size {
            {
                let _rhs = 1;
                let _lhs = &mut huffman_tree_histogram[huffman_tree[i] as (usize)];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0usize;
    'loop3: loop {
        if i < 18usize {
            if huffman_tree_histogram[i] != 0 {
                if num_codes == 0i32 {
                    code = i;
                    num_codes = 1i32;
                } else if num_codes == 1i32 {
                    num_codes = 2i32;
                }
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    BrotliCreateHuffmanTree(
        huffman_tree_histogram.as_mut_ptr() as (*const u32),
        18usize,
        5i32,
        tree,
        code_length_bitdepth.as_mut_ptr()
    );
    BrotliConvertBitDepthsToSymbols(
        code_length_bitdepth.as_mut_ptr() as (*const u8),
        18usize,
        code_length_bitdepth_symbols.as_mut_ptr()
    );
    BrotliStoreHuffmanTreeOfHuffmanTreeToBitMask(
        num_codes,
        code_length_bitdepth.as_mut_ptr() as (*const u8),
        storage_ix,
        storage
    );
    if num_codes == 1i32 {
        code_length_bitdepth[code] = 0i32 as (u8);
    }
    BrotliStoreHuffmanTreeToBitMask(
        huffman_tree_size,
        huffman_tree.as_mut_ptr() as (*const u8),
        huffman_tree_extra_bits.as_mut_ptr() as (*const u8),
        code_length_bitdepth.as_mut_ptr() as (*const u8),
        code_length_bitdepth_symbols.as_mut_ptr() as (*const u16),
        storage_ix,
        storage
    );
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

unsafe extern fn InitHuffmanTree(
    mut xself : *mut HuffmanTree,
    mut count : u32,
    mut left : i16,
    mut right : i16
) {
    (*xself).total_count_ = count;
    (*xself).index_left_ = left;
    (*xself).index_right_or_value_ = right;
}

unsafe extern fn SortHuffmanTreeItems(
    mut items : *mut HuffmanTree,
    n : usize,
    mut
    comparator
    :
    unsafe extern fn(*const HuffmanTree, *const HuffmanTree) -> i32
) {
    static mut gaps
        : [usize; 6]
        = [   132usize,
              57usize,
              23usize,
              10usize,
              4usize,
              1usize
          ];
    if n < 13usize {
        let mut i : usize;
        i = 1usize;
        'loop14: loop {
            if i < n {
                let mut tmp : HuffmanTree = *items.offset(i as (isize));
                let mut k : usize = i;
                let mut j : usize = i.wrapping_sub(1usize);
                'loop17: loop {
                    if comparator(
                           &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                           &mut *items.offset(
                                     j as (isize)
                                 ) as (*mut HuffmanTree) as (*const HuffmanTree)
                       ) != 0 {
                        *items.offset(k as (isize)) = *items.offset(j as (isize));
                        k = j;
                        if {
                               let _old = j;
                               j = j.wrapping_sub(1 as (usize));
                               _old
                           } == 0 {
                            break 'loop17;
                        } else {
                            continue 'loop17;
                        }
                    } else {
                        break 'loop17;
                    }
                }
                *items.offset(k as (isize)) = tmp;
                i = i.wrapping_add(1 as (usize));
                continue 'loop14;
            } else {
                break 'loop14;
            }
        }
    } else {
        let mut g : i32 = if n < 57usize { 2i32 } else { 0i32 };
        'loop2: loop {
            if g < 6i32 {
                let mut gap : usize = gaps[g as (usize)];
                let mut i : usize;
                i = gap;
                'loop5: loop {
                    if i < n {
                        let mut j : usize = i;
                        let mut tmp : HuffmanTree = *items.offset(i as (isize));
                        'loop8: loop {
                            if j >= gap && (comparator(
                                                &mut tmp as (*mut HuffmanTree) as (*const HuffmanTree),
                                                &mut *items.offset(
                                                          j.wrapping_sub(gap) as (isize)
                                                      ) as (*mut HuffmanTree) as (*const HuffmanTree)
                                            ) != 0) {
                                *items.offset(j as (isize)) = *items.offset(
                                                                   j.wrapping_sub(gap) as (isize)
                                                               );
                                j = j.wrapping_sub(gap);
                                continue 'loop8;
                            } else {
                                break 'loop8;
                            }
                        }
                        *items.offset(j as (isize)) = tmp;
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop5;
                    } else {
                        break 'loop5;
                    }
                }
                g = g + 1;
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}

unsafe extern fn SortHuffmanTree(
    mut v0 : *const HuffmanTree, mut v1 : *const HuffmanTree
) -> i32 {
    if !!((*v0).total_count_ < (*v1).total_count_) {
        1i32
    } else {
        0i32
    }
}

unsafe extern fn StoreStaticCodeLengthCode(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        40usize,
        0xffu32 as (usize) << 32i32 | 0x55555554u32 as (usize),
        storage_ix,
        storage
    );
}

#[no_mangle]
pub unsafe extern fn BrotliBuildAndStoreHuffmanTreeFast(
    mut m : *mut MemoryManager,
    mut histogram : *const u32,
    histogram_total : usize,
    max_bits : usize,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut count : usize = 0usize;
    let mut symbols
        : [usize; 4]
        = [   0usize,
              0usize,
              0usize,
              0usize
          ];
    let mut length : usize = 0usize;
    let mut total : usize = histogram_total;
    'loop1: loop {
        if total != 0usize {
            if *histogram.offset(length as (isize)) != 0 {
                if count < 4usize {
                    symbols[count] = length;
                }
                count = count.wrapping_add(1 as (usize));
                total = total.wrapping_sub(
                            *histogram.offset(length as (isize)) as (usize)
                        );
            }
            length = length.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    if count <= 1usize {
        BrotliWriteBits(
            4usize,
            1usize,
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            symbols[0usize],
            storage_ix,
            storage
        );
        *depth.offset(symbols[0usize] as (isize)) = 0i32 as (u8);
        *bits.offset(symbols[0usize] as (isize)) = 0i32 as (u16);
    } else {
        memset(
            depth as (*mut ::std::os::raw::c_void),
            0i32,
            length.wrapping_mul(::std::mem::size_of::<u8>())
        );
        let max_tree_size
            : usize
            = (2usize).wrapping_mul(length).wrapping_add(
                  1usize
              );
        let mut tree
            : *mut HuffmanTree
            = if max_tree_size != 0 {
                  BrotliAllocate(
                      m,
                      max_tree_size.wrapping_mul(::std::mem::size_of::<HuffmanTree>())
                  ) as (*mut HuffmanTree)
              } else {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree)
              };
        let mut count_limit : u32;
        if !(0i32 == 0) {
        } else {
            count_limit = 1u32;
            'loop5: loop {
                let mut node : *mut HuffmanTree = tree;
                let mut l : usize;
                l = length;
                'loop6: loop {
                    if l != 0usize {
                        l = l.wrapping_sub(1 as (usize));
                        if *histogram.offset(l as (isize)) != 0 {
                            if *histogram.offset(l as (isize)) >= count_limit {
                                InitHuffmanTree(
                                    node,
                                    *histogram.offset(l as (isize)),
                                    -1i32 as (i16),
                                    l as (i16)
                                );
                            } else {
                                InitHuffmanTree(node,count_limit,-1i32 as (i16),l as (i16));
                            }
                            node = node.offset(1 as (isize));
                            continue 'loop6;
                        } else {
                            continue 'loop6;
                        }
                    } else {
                        break 'loop6;
                    }
                }
                let n
                    : i32
                    = ((node as (isize)).wrapping_sub(
                           tree as (isize)
                       ) / ::std::mem::size_of::<HuffmanTree>() as (isize)) as (i32);
                let mut sentinel : HuffmanTree;
                let mut i : i32 = 0i32;
                let mut j : i32 = n + 1i32;
                let mut k : i32;
                SortHuffmanTreeItems(tree,n as (usize),SortHuffmanTree);
                InitHuffmanTree(
                    &mut sentinel as (*mut HuffmanTree),
                    !(0u32),
                    -1i32 as (i16),
                    -1i32 as (i16)
                );
                *{
                     let _old = node;
                     node = node.offset(1 as (isize));
                     _old
                 } = sentinel;
                *{
                     let _old = node;
                     node = node.offset(1 as (isize));
                     _old
                 } = sentinel;
                k = n - 1i32;
                'loop8: loop {
                    if k > 0i32 {
                        let mut left : i32;
                        let mut right : i32;
                        if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                              j as (isize)
                                                                          )).total_count_ {
                            left = i;
                            i = i + 1;
                        } else {
                            left = j;
                            j = j + 1;
                        }
                        if (*tree.offset(i as (isize))).total_count_ <= (*tree.offset(
                                                                              j as (isize)
                                                                          )).total_count_ {
                            right = i;
                            i = i + 1;
                        } else {
                            right = j;
                            j = j + 1;
                        }
                        (*node.offset(-1i32 as (isize))).total_count_ = (*tree.offset(
                                                                              left as (isize)
                                                                          )).total_count_.wrapping_add(
                                                                            (*tree.offset(
                                                                                  right as (isize)
                                                                              )).total_count_
                                                                        );
                        (*node.offset(-1i32 as (isize))).index_left_ = left as (i16);
                        (*node.offset(
                              -1i32 as (isize)
                          )).index_right_or_value_ = right as (i16);
                        *{
                             let _old = node;
                             node = node.offset(1 as (isize));
                             _old
                         } = sentinel;
                        k = k - 1;
                        continue 'loop8;
                    } else {
                        break 'loop8;
                    }
                }
                if BrotliSetDepth(2i32 * n - 1i32,tree,depth,14i32) != 0 {
                    break 'loop5;
                } else {
                    count_limit = count_limit.wrapping_mul(2u32);
                    continue 'loop5;
                }
            }
            BrotliFree(m,tree as (*mut ::std::os::raw::c_void));
            tree = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree);
            BrotliConvertBitDepthsToSymbols(depth as (*const u8),length,bits);
            if count <= 4usize {
                let mut i : usize;
                BrotliWriteBits(
                    2usize,
                    1usize,
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    2usize,
                    count.wrapping_sub(1usize),
                    storage_ix,
                    storage
                );
                i = 0usize;
                'loop28: loop {
                    if i < count {
                        let mut j : usize;
                        j = i.wrapping_add(1usize);
                        'loop36: loop {
                            if j < count {
                                if *depth.offset(symbols[j] as (isize)) as (i32) < *depth.offset(
                                                                                        symbols[
                                                                                            i
                                                                                        ] as (isize)
                                                                                    ) as (i32) {
                                    let mut __brotli_swap_tmp : usize = symbols[j];
                                    symbols[j] = symbols[i];
                                    symbols[i] = __brotli_swap_tmp;
                                }
                                j = j.wrapping_add(1 as (usize));
                                continue 'loop36;
                            } else {
                                break 'loop36;
                            }
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop28;
                    } else {
                        break 'loop28;
                    }
                }
                if count == 2usize {
                    BrotliWriteBits(
                        max_bits,
                        symbols[0usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[1usize],
                        storage_ix,
                        storage
                    );
                } else if count == 3usize {
                    BrotliWriteBits(
                        max_bits,
                        symbols[0usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[1usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[2usize],
                        storage_ix,
                        storage
                    );
                } else {
                    BrotliWriteBits(
                        max_bits,
                        symbols[0usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[1usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[2usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        max_bits,
                        symbols[3usize],
                        storage_ix,
                        storage
                    );
                    BrotliWriteBits(
                        1usize,
                        if *depth.offset(
                                symbols[0usize] as (isize)
                            ) as (i32) == 1i32 {
                            1i32
                        } else {
                            0i32
                        } as (usize),
                        storage_ix,
                        storage
                    );
                }
            } else {
                let mut previous_value : u8 = 8i32 as (u8);
                let mut i : usize;
                StoreStaticCodeLengthCode(storage_ix,storage);
                i = 0usize;
                'loop13: loop {
                    if i < length {
                        let value : u8 = *depth.offset(i as (isize));
                        let mut reps : usize = 1usize;
                        let mut k : usize;
                        k = i.wrapping_add(1usize);
                        'loop15: loop {
                            if k < length && (*depth.offset(
                                                   k as (isize)
                                               ) as (i32) == value as (i32)) {
                                reps = reps.wrapping_add(1 as (usize));
                                k = k.wrapping_add(1 as (usize));
                                continue 'loop15;
                            } else {
                                break 'loop15;
                            }
                        }
                        i = i.wrapping_add(reps);
                        if value as (i32) == 0i32 {
                            BrotliWriteBits(
                                kZeroRepsDepth[reps] as (usize),
                                kZeroRepsBits[reps],
                                storage_ix,
                                storage
                            );
                            continue 'loop13;
                        } else {
                            if previous_value as (i32) != value as (i32) {
                                BrotliWriteBits(
                                    kCodeLengthDepth[value as (usize)] as (usize),
                                    kCodeLengthBits[value as (usize)] as (usize),
                                    storage_ix,
                                    storage
                                );
                                reps = reps.wrapping_sub(1 as (usize));
                            }
                            if reps < 3usize {
                                'loop21: loop {
                                    if reps != 0usize {
                                        reps = reps.wrapping_sub(1 as (usize));
                                        BrotliWriteBits(
                                            kCodeLengthDepth[value as (usize)] as (usize),
                                            kCodeLengthBits[value as (usize)] as (usize),
                                            storage_ix,
                                            storage
                                        );
                                        continue 'loop21;
                                    } else {
                                        break 'loop21;
                                    }
                                }
                            } else {
                                reps = reps.wrapping_sub(3usize);
                                BrotliWriteBits(
                                    kNonZeroRepsDepth[reps] as (usize),
                                    kNonZeroRepsBits[reps],
                                    storage_ix,
                                    storage
                                );
                            }
                            previous_value = value;
                            continue 'loop13;
                        }
                    } else {
                        break 'loop13;
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ContextType {
    CONTEXT_LSB6 = 0i32,
    CONTEXT_MSB6 = 1i32,
    CONTEXT_UTF8 = 2i32,
    CONTEXT_SIGNED = 3i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Command {
    pub insert_len_ : u32,
    pub copy_len_ : u32,
    pub dist_extra_ : u32,
    pub cmd_prefix_ : u16,
    pub dist_prefix_ : u16,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockSplit {
    pub num_types : usize,
    pub num_blocks : usize,
    pub types : *mut u8,
    pub lengths : *mut u32,
    pub types_alloc_size : usize,
    pub lengths_alloc_size : usize,
}

#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : [u32; 256],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[repr(C)]
pub struct HistogramCommand {
    pub data_ : [u32; 704],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[repr(C)]
pub struct HistogramDistance {
    pub data_ : [u32; 520],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MetaBlockSplit {
    pub literal_split : BlockSplit,
    pub command_split : BlockSplit,
    pub distance_split : BlockSplit,
    pub literal_context_map : *mut u32,
    pub literal_context_map_size : usize,
    pub distance_context_map : *mut u32,
    pub distance_context_map_size : usize,
    pub literal_histograms : *mut HistogramLiteral,
    pub literal_histograms_size : usize,
    pub command_histograms : *mut HistogramCommand,
    pub command_histograms_size : usize,
    pub distance_histograms : *mut HistogramDistance,
    pub distance_histograms_size : usize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BlockTypeCodeCalculator {
    pub last_type : usize,
    pub second_last_type : usize,
}

#[repr(C)]
pub struct BlockSplitCode {
    pub type_code_calculator : BlockTypeCodeCalculator,
    pub type_depths : [u8; 258],
    pub type_bits : [u16; 258],
    pub length_depths : [u8; 26],
    pub length_bits : [u16; 26],
}

#[repr(C)]
pub struct BlockEncoder {
    pub alphabet_size_ : usize,
    pub num_block_types_ : usize,
    pub block_types_ : *const u8,
    pub block_lengths_ : *const u32,
    pub num_blocks_ : usize,
    pub block_split_code_ : BlockSplitCode,
    pub block_ix_ : usize,
    pub block_len_ : usize,
    pub entropy_ix_ : usize,
    pub depths_ : *mut u8,
    pub bits_ : *mut u16,
}

unsafe extern fn Log2FloorNonZero(mut n : usize) -> u32 {
    let mut result : u32 = 0u32;
    'loop1: loop {
        if {
               n = n >> 1i32;
               n
           } != 0 {
            result = result.wrapping_add(1 as (u32));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    result
}

unsafe extern fn BrotliEncodeMlen(
    mut length : usize,
    mut bits : *mut usize,
    mut numbits : *mut usize,
    mut nibblesbits : *mut usize
) {
    let mut lg
        : usize
        = (if length == 1usize {
               1u32
           } else {
               Log2FloorNonZero(
                   length.wrapping_sub(1usize) as (u32) as (usize)
               ).wrapping_add(
                   1u32
               )
           }) as (usize);
    let mut mnibbles
        : usize
        = (if lg < 16usize {
               16usize
           } else {
               lg.wrapping_add(3usize)
           }).wrapping_div(
              4usize
          );
    if length > 0usize {
        0i32;
    } else {
        __assert_fail(
            (*b"length > 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliEncodeMlen\0").as_ptr()
        );
    }
    if length <= (1i32 << 24i32) as (usize) {
        0i32;
    } else {
        __assert_fail(
            (*b"length <= (1 << 24)\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliEncodeMlen\0").as_ptr()
        );
    }
    if lg <= 24usize {
        0i32;
    } else {
        __assert_fail(
            (*b"lg <= 24\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliEncodeMlen\0").as_ptr()
        );
    }
    *nibblesbits = mnibbles.wrapping_sub(4usize);
    *numbits = mnibbles.wrapping_mul(4usize);
    *bits = length.wrapping_sub(1usize);
}

unsafe extern fn StoreCompressedMetaBlockHeader(
    mut is_final_block : i32,
    mut length : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lenbits : usize;
    let mut nlenbits : usize;
    let mut nibblesbits : usize;
    BrotliWriteBits(
        1usize,
        is_final_block as (usize),
        storage_ix,
        storage
    );
    if is_final_block != 0 {
        BrotliWriteBits(
            1usize,
            0usize,
            storage_ix,
            storage
        );
    }
    BrotliEncodeMlen(
        length,
        &mut lenbits as (*mut usize),
        &mut nlenbits as (*mut usize),
        &mut nibblesbits as (*mut usize)
    );
    BrotliWriteBits(2usize,nibblesbits,storage_ix,storage);
    BrotliWriteBits(nlenbits,lenbits,storage_ix,storage);
    if is_final_block == 0 {
        BrotliWriteBits(
            1usize,
            0usize,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn InitBlockTypeCodeCalculator(
    mut xself : *mut BlockTypeCodeCalculator
) {
    (*xself).last_type = 1usize;
    (*xself).second_last_type = 0usize;
}

unsafe extern fn InitBlockEncoder(
    mut xself : *mut BlockEncoder,
    mut alphabet_size : usize,
    mut num_block_types : usize,
    mut block_types : *const u8,
    mut block_lengths : *const u32,
    num_blocks : usize
) {
    (*xself).alphabet_size_ = alphabet_size;
    (*xself).num_block_types_ = num_block_types;
    (*xself).block_types_ = block_types;
    (*xself).block_lengths_ = block_lengths;
    (*xself).num_blocks_ = num_blocks;
    InitBlockTypeCodeCalculator(
        &mut (*xself).block_split_code_.type_code_calculator as (*mut BlockTypeCodeCalculator)
    );
    (*xself).block_ix_ = 0usize;
    (*xself).block_len_ = if num_blocks == 0usize {
                             0u32
                         } else {
                             *block_lengths.offset(0i32 as (isize))
                         } as (usize);
    (*xself).entropy_ix_ = 0usize;
    (*xself).depths_ = 0i32 as (*mut u8);
    (*xself).bits_ = 0i32 as (*mut u16);
}

unsafe extern fn NextBlockTypeCode(
    mut calculator : *mut BlockTypeCodeCalculator, mut type_ : u8
) -> usize {
    let mut type_code
        : usize
        = (if type_ as (usize) == (*calculator).last_type.wrapping_add(
                                      1usize
                                  ) {
               1u32
           } else if type_ as (usize) == (*calculator).second_last_type {
               0u32
           } else {
               (type_ as (u32)).wrapping_add(2u32)
           }) as (usize);
    (*calculator).second_last_type = (*calculator).last_type;
    (*calculator).last_type = type_ as (usize);
    type_code
}

unsafe extern fn BlockLengthPrefixCode(mut len : u32) -> u32 {
    let mut code
        : u32
        = (if len >= 177u32 {
               if len >= 753u32 { 20i32 } else { 14i32 }
           } else if len >= 41u32 {
               7i32
           } else {
               0i32
           }) as (u32);
    'loop1: loop {
        if code < (26i32 - 1i32) as (u32) && (len >= kBlockLengthPrefixCode[
                                                         code.wrapping_add(1u32) as (usize)
                                                     ].offset) {
            code = code.wrapping_add(1 as (u32));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    code
}

unsafe extern fn StoreVarLenUint8(
    mut n : usize, mut storage_ix : *mut usize, mut storage : *mut u8
) { if n == 0usize {
        BrotliWriteBits(
            1usize,
            0usize,
            storage_ix,
            storage
        );
    } else {
        let mut nbits : usize = Log2FloorNonZero(n) as (usize);
        BrotliWriteBits(
            1usize,
            1usize,
            storage_ix,
            storage
        );
        BrotliWriteBits(3usize,nbits,storage_ix,storage);
        BrotliWriteBits(
            nbits,
            n.wrapping_sub(1usize << nbits),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn StoreSimpleHuffmanTree(
    mut depths : *const u8,
    mut symbols : *mut usize,
    mut num_symbols : usize,
    mut max_bits : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BrotliWriteBits(
        2usize,
        1usize,
        storage_ix,
        storage
    );
    BrotliWriteBits(
        2usize,
        num_symbols.wrapping_sub(1usize),
        storage_ix,
        storage
    );
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < num_symbols {
            let mut j : usize;
            j = i.wrapping_add(1usize);
            'loop9: loop {
                if j < num_symbols {
                    if *depths.offset(
                            *symbols.offset(j as (isize)) as (isize)
                        ) as (i32) < *depths.offset(
                                          *symbols.offset(i as (isize)) as (isize)
                                      ) as (i32) {
                        let mut __brotli_swap_tmp : usize = *symbols.offset(j as (isize));
                        *symbols.offset(j as (isize)) = *symbols.offset(i as (isize));
                        *symbols.offset(i as (isize)) = __brotli_swap_tmp;
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    if num_symbols == 2usize {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
    } else if num_symbols == 3usize {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2i32 as (isize)),
            storage_ix,
            storage
        );
    } else {
        BrotliWriteBits(
            max_bits,
            *symbols.offset(0i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(1i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(2i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            max_bits,
            *symbols.offset(3i32 as (isize)),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            1usize,
            if *depths.offset(
                    *symbols.offset(0i32 as (isize)) as (isize)
                ) as (i32) == 1i32 {
                1i32
            } else {
                0i32
            } as (usize),
            storage_ix,
            storage
        );
    }
}

unsafe extern fn BuildAndStoreHuffmanTree(
    mut histogram : *const u32,
    length : usize,
    mut tree : *mut HuffmanTree,
    mut depth : *mut u8,
    mut bits : *mut u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut count : usize = 0usize;
    let mut s4
        : [usize; 4]
        = [   0usize,
              0usize,
              0usize,
              0usize
          ];
    let mut i : usize;
    let mut max_bits : usize = 0usize;
    i = 0usize;
    'loop1: loop {
        if i < length {
            if *histogram.offset(i as (isize)) != 0 {
                if count < 4usize {
                    s4[count] = i;
                } else if count > 4usize {
                    break 'loop1;
                }
                count = count.wrapping_add(1 as (usize));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    let mut max_bits_counter
        : usize
        = length.wrapping_sub(1usize);
    'loop6: loop {
        if max_bits_counter != 0 {
            max_bits_counter = max_bits_counter >> 1i32;
            max_bits = max_bits.wrapping_add(1 as (usize));
            continue 'loop6;
        } else {
            break 'loop6;
        }
    }
    if count <= 1usize {
        BrotliWriteBits(
            4usize,
            1usize,
            storage_ix,
            storage
        );
        BrotliWriteBits(max_bits,s4[0usize],storage_ix,storage);
        *depth.offset(s4[0usize] as (isize)) = 0i32 as (u8);
        *bits.offset(s4[0usize] as (isize)) = 0i32 as (u16);
    } else {
        memset(
            depth as (*mut ::std::os::raw::c_void),
            0i32,
            length.wrapping_mul(::std::mem::size_of::<u8>())
        );
        BrotliCreateHuffmanTree(histogram,length,15i32,tree,depth);
        BrotliConvertBitDepthsToSymbols(depth as (*const u8),length,bits);
        if count <= 4usize {
            StoreSimpleHuffmanTree(
                depth as (*const u8),
                s4.as_mut_ptr(),
                count,
                max_bits,
                storage_ix,
                storage
            );
        } else {
            BrotliStoreHuffmanTree(
                depth as (*const u8),
                length,
                tree,
                storage_ix,
                storage
            );
        }
    }
}

unsafe extern fn GetBlockLengthPrefixCode(
    mut len : u32,
    mut code : *mut usize,
    mut n_extra : *mut u32,
    mut extra : *mut u32
) {
    *code = BlockLengthPrefixCode(len) as (usize);
    *n_extra = kBlockLengthPrefixCode[*code].nbits;
    *extra = len.wrapping_sub(kBlockLengthPrefixCode[*code].offset);
}

unsafe extern fn StoreBlockSwitch(
    mut code : *mut BlockSplitCode,
    block_len : u32,
    block_type : u8,
    mut is_first_block : i32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut typecode
        : usize
        = NextBlockTypeCode(
              &mut (*code).type_code_calculator as (*mut BlockTypeCodeCalculator),
              block_type
          );
    let mut lencode : usize;
    let mut len_nextra : u32;
    let mut len_extra : u32;
    if is_first_block == 0 {
        BrotliWriteBits(
            (*code).type_depths[typecode] as (usize),
            (*code).type_bits[typecode] as (usize),
            storage_ix,
            storage
        );
    }
    GetBlockLengthPrefixCode(
        block_len,
        &mut lencode as (*mut usize),
        &mut len_nextra as (*mut u32),
        &mut len_extra as (*mut u32)
    );
    BrotliWriteBits(
        (*code).length_depths[lencode] as (usize),
        (*code).length_bits[lencode] as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        len_nextra as (usize),
        len_extra as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn BuildAndStoreBlockSplitCode(
    mut types : *const u8,
    mut lengths : *const u32,
    num_blocks : usize,
    num_types : usize,
    mut tree : *mut HuffmanTree,
    mut code : *mut BlockSplitCode,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut type_histo : [u32; 258];
    let mut length_histo : [u32; 26];
    let mut i : usize;
    let mut type_code_calculator : BlockTypeCodeCalculator;
    memset(
        type_histo.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        num_types.wrapping_add(2usize).wrapping_mul(
            ::std::mem::size_of::<u32>()
        )
    );
    memset(
        length_histo.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 26]>()
    );
    InitBlockTypeCodeCalculator(
        &mut type_code_calculator as (*mut BlockTypeCodeCalculator)
    );
    i = 0usize;
    'loop1: loop {
        if i < num_blocks {
            let mut type_code
                : usize
                = NextBlockTypeCode(
                      &mut type_code_calculator as (*mut BlockTypeCodeCalculator),
                      *types.offset(i as (isize))
                  );
            if i != 0usize {
                let _rhs = 1;
                let _lhs = &mut type_histo[type_code];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            {
                let _rhs = 1;
                let _lhs
                    = &mut length_histo[
                               BlockLengthPrefixCode(*lengths.offset(i as (isize))) as (usize)
                           ];
                *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    StoreVarLenUint8(
        num_types.wrapping_sub(1usize),
        storage_ix,
        storage
    );
    if num_types > 1usize {
        BuildAndStoreHuffmanTree(
            &mut type_histo[0usize] as (*mut u32) as (*const u32),
            num_types.wrapping_add(2usize),
            tree,
            &mut (*code).type_depths[0usize] as (*mut u8),
            &mut (*code).type_bits[0usize] as (*mut u16),
            storage_ix,
            storage
        );
        BuildAndStoreHuffmanTree(
            &mut length_histo[0usize] as (*mut u32) as (*const u32),
            26usize,
            tree,
            &mut (*code).length_depths[0usize] as (*mut u8),
            &mut (*code).length_bits[0usize] as (*mut u16),
            storage_ix,
            storage
        );
        StoreBlockSwitch(
            code,
            *lengths.offset(0i32 as (isize)),
            *types.offset(0i32 as (isize)),
            1i32,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn BuildAndStoreBlockSwitchEntropyCodes(
    mut xself : *mut BlockEncoder,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    BuildAndStoreBlockSplitCode(
        (*xself).block_types_,
        (*xself).block_lengths_,
        (*xself).num_blocks_,
        (*xself).num_block_types_,
        tree,
        &mut (*xself).block_split_code_ as (*mut BlockSplitCode),
        storage_ix,
        storage
    );
}

unsafe extern fn IndexOf(
    mut v : *const u8, mut v_size : usize, mut value : u8
) -> usize {
    let mut i : usize = 0usize;
    'loop1: loop {
        if i < v_size {
            if *v.offset(i as (isize)) as (i32) == value as (i32) {
                break 'loop1;
            } else {
                i = i.wrapping_add(1 as (usize));
                continue 'loop1;
            }
        } else {
            return i;
        }
    }
    i
}

unsafe extern fn MoveToFront(mut v : *mut u8, mut index : usize) {
    let mut value : u8 = *v.offset(index as (isize));
    let mut i : usize;
    i = index;
    'loop1: loop {
        if i != 0usize {
            *v.offset(i as (isize)) = *v.offset(
                                           i.wrapping_sub(1usize) as (isize)
                                       );
            i = i.wrapping_sub(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    *v.offset(0i32 as (isize)) = value;
}

unsafe extern fn MoveToFrontTransform(
    mut v_in : *const u32, v_size : usize, mut v_out : *mut u32
) {
    let mut i : usize;
    let mut mtf : [u8; 256];
    let mut max_value : u32;
    if v_size == 0usize {
    } else {
        max_value = *v_in.offset(0i32 as (isize));
        i = 1usize;
        'loop2: loop {
            if i < v_size {
                if *v_in.offset(i as (isize)) > max_value {
                    max_value = *v_in.offset(i as (isize));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if max_value < 256u32 {
            0i32;
        } else {
            __assert_fail(
                (*b"max_value < 256u\0").as_ptr(),
                file!().as_ptr(),
                line!(),
                (*b"MoveToFrontTransform\0").as_ptr()
            );
        }
        i = 0usize;
        'loop4: loop {
            if i <= max_value as (usize) {
                mtf[i] = i as (u8);
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        let mut mtf_size
            : usize
            = max_value.wrapping_add(1u32) as (usize);
        i = 0usize;
        'loop6: loop {
            if i < v_size {
                let mut index
                    : usize
                    = IndexOf(
                          mtf.as_mut_ptr() as (*const u8),
                          mtf_size,
                          *v_in.offset(i as (isize)) as (u8)
                      );
                if index < mtf_size {
                    0i32;
                } else {
                    __assert_fail(
                        (*b"index < mtf_size\0").as_ptr(),
                        file!().as_ptr(),
                        line!(),
                        (*b"MoveToFrontTransform\0").as_ptr()
                    );
                }
                *v_out.offset(i as (isize)) = index as (u32);
                MoveToFront(mtf.as_mut_ptr(),index);
                i = i.wrapping_add(1 as (usize));
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
    }
}

unsafe extern fn brotli_max_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a > b { a } else { b }
}

unsafe extern fn brotli_min_uint32_t(
    mut a : u32, mut b : u32
) -> u32 {
    if a < b { a } else { b }
}

unsafe extern fn RunLengthCodeZeros(
    in_size : usize,
    mut v : *mut u32,
    mut out_size : *mut usize,
    mut max_run_length_prefix : *mut u32
) {
    let mut max_reps : u32 = 0u32;
    let mut i : usize;
    let mut max_prefix : u32;
    i = 0usize;
    'loop1: loop {
        if i < in_size {
            let mut reps : u32 = 0u32;
            'loop17: loop {
                if i < in_size && (*v.offset(i as (isize)) != 0u32) {
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop17;
                } else {
                    break 'loop17;
                }
            }
            'loop18: loop {
                if i < in_size && (*v.offset(i as (isize)) == 0u32) {
                    reps = reps.wrapping_add(1 as (u32));
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop18;
                } else {
                    break 'loop18;
                }
            }
            max_reps = brotli_max_uint32_t(reps,max_reps);
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    max_prefix = if max_reps > 0u32 {
                     Log2FloorNonZero(max_reps as (usize))
                 } else {
                     0u32
                 };
    max_prefix = brotli_min_uint32_t(
                     max_prefix,
                     *max_run_length_prefix
                 );
    *max_run_length_prefix = max_prefix;
    *out_size = 0usize;
    i = 0usize;
    'loop3: loop {
        if i < in_size {
            if *out_size <= i {
                0i32;
            } else {
                __assert_fail(
                    (*b"*out_size <= i\0").as_ptr(),
                    file!().as_ptr(),
                    line!(),
                    (*b"RunLengthCodeZeros\0").as_ptr()
                );
            }
            if *v.offset(i as (isize)) != 0u32 {
                *v.offset(*out_size as (isize)) = (*v.offset(
                                                        i as (isize)
                                                    )).wrapping_add(
                                                      *max_run_length_prefix
                                                  );
                i = i.wrapping_add(1 as (usize));
                *out_size = (*out_size).wrapping_add(1 as (usize));
                continue 'loop3;
            } else {
                let mut reps : u32 = 1u32;
                let mut k : usize;
                k = i.wrapping_add(1usize);
                'loop7: loop {
                    if k < in_size && (*v.offset(k as (isize)) == 0u32) {
                        reps = reps.wrapping_add(1 as (u32));
                        k = k.wrapping_add(1 as (usize));
                        continue 'loop7;
                    } else {
                        break 'loop7;
                    }
                }
                i = i.wrapping_add(reps as (usize));
                'loop9: loop {
                    if reps != 0u32 {
                        if reps < 2u32 << max_prefix {
                            break 'loop9;
                        } else {
                            let extra_bits : u32 = (1u32 << max_prefix).wrapping_sub(1u32);
                            *v.offset(*out_size as (isize)) = max_prefix.wrapping_add(
                                                                  extra_bits << 9i32
                                                              );
                            reps = reps.wrapping_sub((2u32 << max_prefix).wrapping_sub(1u32));
                            *out_size = (*out_size).wrapping_add(1 as (usize));
                            continue 'loop9;
                        }
                    } else {
                        continue 'loop3;
                    }
                }
                let mut run_length_prefix
                    : u32
                    = Log2FloorNonZero(reps as (usize));
                let extra_bits
                    : u32
                    = reps.wrapping_sub(1u32 << run_length_prefix);
                *v.offset(*out_size as (isize)) = run_length_prefix.wrapping_add(
                                                      extra_bits << 9i32
                                                  );
                *out_size = (*out_size).wrapping_add(1 as (usize));
                continue 'loop3;
            }
        } else {
            break 'loop3;
        }
    }
}

unsafe extern fn EncodeContextMap(
    mut m : *mut MemoryManager,
    mut context_map : *const u32,
    mut context_map_size : usize,
    mut num_clusters : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut i : usize;
    let mut rle_symbols : *mut u32;
    let mut max_run_length_prefix : u32 = 6u32;
    let mut num_rle_symbols : usize = 0usize;
    let mut histogram : [u32; 272];
    static kSymbolMask : u32 = (1u32 << 9i32) - 1u32;
    let mut depths : [u8; 272];
    let mut bits : [u16; 272];
    StoreVarLenUint8(
        num_clusters.wrapping_sub(1usize),
        storage_ix,
        storage
    );
    if num_clusters == 1usize {
    } else {
        rle_symbols = if context_map_size != 0 {
                          BrotliAllocate(
                              m,
                              context_map_size.wrapping_mul(::std::mem::size_of::<u32>())
                          ) as (*mut u32)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
                      };
        if !(0i32 == 0) {
        } else {
            MoveToFrontTransform(context_map,context_map_size,rle_symbols);
            RunLengthCodeZeros(
                context_map_size,
                rle_symbols,
                &mut num_rle_symbols as (*mut usize),
                &mut max_run_length_prefix as (*mut u32)
            );
            memset(
                histogram.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                0i32,
                ::std::mem::size_of::<[u32; 272]>()
            );
            i = 0usize;
            'loop3: loop {
                if i < num_rle_symbols {
                    {
                        let _rhs = 1;
                        let _lhs
                            = &mut histogram[
                                       (*rle_symbols.offset(i as (isize)) & kSymbolMask) as (usize)
                                   ];
                        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                    }
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop3;
                } else {
                    break 'loop3;
                }
            }
            let mut use_rle
                : i32
                = if !!(max_run_length_prefix > 0u32) {
                      1i32
                  } else {
                      0i32
                  };
            BrotliWriteBits(
                1usize,
                use_rle as (usize),
                storage_ix,
                storage
            );
            if use_rle != 0 {
                BrotliWriteBits(
                    4usize,
                    max_run_length_prefix.wrapping_sub(1u32) as (usize),
                    storage_ix,
                    storage
                );
            }
            BuildAndStoreHuffmanTree(
                histogram.as_mut_ptr() as (*const u32),
                num_clusters.wrapping_add(max_run_length_prefix as (usize)),
                tree,
                depths.as_mut_ptr(),
                bits.as_mut_ptr(),
                storage_ix,
                storage
            );
            i = 0usize;
            'loop7: loop {
                if i < num_rle_symbols {
                    let rle_symbol
                        : u32
                        = *rle_symbols.offset(i as (isize)) & kSymbolMask;
                    let extra_bits_val
                        : u32
                        = *rle_symbols.offset(i as (isize)) >> 9i32;
                    BrotliWriteBits(
                        depths[rle_symbol as (usize)] as (usize),
                        bits[rle_symbol as (usize)] as (usize),
                        storage_ix,
                        storage
                    );
                    if rle_symbol > 0u32 && (rle_symbol <= max_run_length_prefix) {
                        BrotliWriteBits(
                            rle_symbol as (usize),
                            extra_bits_val as (usize),
                            storage_ix,
                            storage
                        );
                    }
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop7;
                } else {
                    break 'loop7;
                }
            }
            BrotliWriteBits(
                1usize,
                1usize,
                storage_ix,
                storage
            );
            BrotliFree(m,rle_symbols as (*mut ::std::os::raw::c_void));
            rle_symbols = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
        }
    }
}

unsafe extern fn StoreTrivialContextMap(
    mut num_types : usize,
    mut context_bits : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    StoreVarLenUint8(
        num_types.wrapping_sub(1usize),
        storage_ix,
        storage
    );
    if num_types > 1usize {
        let mut repeat_code
            : usize
            = context_bits.wrapping_sub(1u32 as (usize));
        let mut repeat_bits
            : usize
            = (1u32 << repeat_code).wrapping_sub(1u32) as (usize);
        let mut alphabet_size
            : usize
            = num_types.wrapping_add(repeat_code);
        let mut histogram : [u32; 272];
        let mut depths : [u8; 272];
        let mut bits : [u16; 272];
        let mut i : usize;
        memset(
            histogram.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            0i32,
            alphabet_size.wrapping_mul(::std::mem::size_of::<u32>())
        );
        BrotliWriteBits(
            1usize,
            1usize,
            storage_ix,
            storage
        );
        BrotliWriteBits(
            4usize,
            repeat_code.wrapping_sub(1usize),
            storage_ix,
            storage
        );
        histogram[repeat_code] = num_types as (u32);
        histogram[0usize] = 1u32;
        i = context_bits;
        'loop2: loop {
            if i < alphabet_size {
                histogram[i] = 1u32;
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        BuildAndStoreHuffmanTree(
            histogram.as_mut_ptr() as (*const u32),
            alphabet_size,
            tree,
            depths.as_mut_ptr(),
            bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        i = 0usize;
        'loop4: loop {
            if i < num_types {
                let mut code
                    : usize
                    = if i == 0usize {
                          0usize
                      } else {
                          i.wrapping_add(context_bits).wrapping_sub(1usize)
                      };
                BrotliWriteBits(
                    depths[code] as (usize),
                    bits[code] as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    depths[repeat_code] as (usize),
                    bits[repeat_code] as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(repeat_code,repeat_bits,storage_ix,storage);
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        BrotliWriteBits(
            1usize,
            1usize,
            storage_ix,
            storage
        );
    }
}

unsafe extern fn BuildAndStoreEntropyCodesLiteral(
    mut m : *mut MemoryManager,
    mut xself : *mut BlockEncoder,
    mut histograms : *const HistogramLiteral,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*xself).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*xself).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(::std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
    (*xself).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(::std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
    } else {
        let mut i : usize;
        i = 0usize;
        'loop2: loop {
            if i < histograms_size {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut (*histograms.offset(i as (isize))).data_[
                             0usize
                         ] as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*xself).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*xself).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}

unsafe extern fn BuildAndStoreEntropyCodesCommand(
    mut m : *mut MemoryManager,
    mut xself : *mut BlockEncoder,
    mut histograms : *const HistogramCommand,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*xself).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*xself).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(::std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
    (*xself).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(::std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
    } else {
        let mut i : usize;
        i = 0usize;
        'loop2: loop {
            if i < histograms_size {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut (*histograms.offset(i as (isize))).data_[
                             0usize
                         ] as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*xself).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*xself).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}

unsafe extern fn BuildAndStoreEntropyCodesDistance(
    mut m : *mut MemoryManager,
    mut xself : *mut BlockEncoder,
    mut histograms : *const HistogramDistance,
    histograms_size : usize,
    mut tree : *mut HuffmanTree,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let alphabet_size : usize = (*xself).alphabet_size_;
    let table_size
        : usize
        = histograms_size.wrapping_mul(alphabet_size);
    (*xself).depths_ = if table_size != 0 {
                          BrotliAllocate(
                              m,
                              table_size.wrapping_mul(::std::mem::size_of::<u8>())
                          ) as (*mut u8)
                      } else {
                          0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
                      };
    (*xself).bits_ = if table_size != 0 {
                        BrotliAllocate(
                            m,
                            table_size.wrapping_mul(::std::mem::size_of::<u16>())
                        ) as (*mut u16)
                    } else {
                        0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
                    };
    if !(0i32 == 0) {
    } else {
        let mut i : usize;
        i = 0usize;
        'loop2: loop {
            if i < histograms_size {
                let mut ix : usize = i.wrapping_mul(alphabet_size);
                BuildAndStoreHuffmanTree(
                    &mut (*histograms.offset(i as (isize))).data_[
                             0usize
                         ] as (*mut u32) as (*const u32),
                    alphabet_size,
                    tree,
                    &mut *(*xself).depths_.offset(ix as (isize)) as (*mut u8),
                    &mut *(*xself).bits_.offset(ix as (isize)) as (*mut u16),
                    storage_ix,
                    storage
                );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
    }
}

unsafe extern fn StoreSymbol(
    mut xself : *mut BlockEncoder,
    mut symbol : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    if (*xself).block_len_ == 0usize {
        let mut block_ix
            : usize
            = {
                  (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
                  (*xself).block_ix_
              };
        let mut block_len
            : u32
            = *(*xself).block_lengths_.offset(block_ix as (isize));
        let mut block_type
            : u8
            = *(*xself).block_types_.offset(block_ix as (isize));
        (*xself).block_len_ = block_len as (usize);
        (*xself).entropy_ix_ = (block_type as (usize)).wrapping_mul(
                                  (*xself).alphabet_size_
                              );
        StoreBlockSwitch(
            &mut (*xself).block_split_code_ as (*mut BlockSplitCode),
            block_len,
            block_type,
            0i32,
            storage_ix,
            storage
        );
    }
    (*xself).block_len_ = (*xself).block_len_.wrapping_sub(1 as (usize));
    let mut ix : usize = (*xself).entropy_ix_.wrapping_add(symbol);
    BrotliWriteBits(
        *(*xself).depths_.offset(ix as (isize)) as (usize),
        *(*xself).bits_.offset(ix as (isize)) as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn CommandCopyLenCode(
    mut xself : *const Command
) -> u32 {
    (*xself).copy_len_ & 0xffffffu32 ^ (*xself).copy_len_ >> 24i32
}

unsafe extern fn GetInsertLengthCode(
    mut insertlen : usize
) -> u16 {
    if insertlen < 6usize {
        insertlen as (u16)
    } else if insertlen < 130usize {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  insertlen.wrapping_sub(2usize)
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            insertlen.wrapping_sub(2usize) >> nbits
        ).wrapping_add(
            2usize
        ) as (u16)
    } else if insertlen < 2114usize {
        Log2FloorNonZero(
            insertlen.wrapping_sub(66usize)
        ).wrapping_add(
            10u32
        ) as (u16)
    } else if insertlen < 6210usize {
        21u32 as (u16)
    } else if insertlen < 22594usize {
        22u32 as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetCopyLengthCode(mut copylen : usize) -> u16 {
    if copylen < 10usize {
        copylen.wrapping_sub(2usize) as (u16)
    } else if copylen < 134usize {
        let mut nbits
            : u32
            = Log2FloorNonZero(
                  copylen.wrapping_sub(6usize)
              ).wrapping_sub(
                  1u32
              );
        ((nbits << 1i32) as (usize)).wrapping_add(
            copylen.wrapping_sub(6usize) >> nbits
        ).wrapping_add(
            4usize
        ) as (u16)
    } else if copylen < 2118usize {
        Log2FloorNonZero(
            copylen.wrapping_sub(70usize)
        ).wrapping_add(
            12u32
        ) as (u16)
    } else {
        23u32 as (u16)
    }
}

unsafe extern fn GetInsertExtra(mut inscode : u16) -> u32 {
    kInsExtra[inscode as (usize)]
}

unsafe extern fn GetInsertBase(mut inscode : u16) -> u32 {
    kInsBase[inscode as (usize)]
}

unsafe extern fn GetCopyBase(mut copycode : u16) -> u32 {
    kCopyBase[copycode as (usize)]
}

unsafe extern fn GetCopyExtra(mut copycode : u16) -> u32 {
    kCopyExtra[copycode as (usize)]
}

unsafe extern fn StoreCommandExtra(
    mut cmd : *const Command,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut copylen_code : u32 = CommandCopyLenCode(cmd);
    let mut inscode
        : u16
        = GetInsertLengthCode((*cmd).insert_len_ as (usize));
    let mut copycode
        : u16
        = GetCopyLengthCode(copylen_code as (usize));
    let mut insnumextra : u32 = GetInsertExtra(inscode);
    let mut insextraval
        : usize
        = (*cmd).insert_len_.wrapping_sub(
              GetInsertBase(inscode)
          ) as (usize);
    let mut copyextraval
        : usize
        = copylen_code.wrapping_sub(GetCopyBase(copycode)) as (usize);
    let mut bits : usize = copyextraval << insnumextra | insextraval;
    BrotliWriteBits(
        insnumextra.wrapping_add(GetCopyExtra(copycode)) as (usize),
        bits,
        storage_ix,
        storage
    );
}

unsafe extern fn Context(
    mut p1 : u8, mut p2 : u8, mut mode : ContextType
) -> u8 {
    if mode as (i32) == ContextType::CONTEXT_SIGNED as (i32) {
        ((kSigned3BitContextLookup[
              p1 as (usize)
          ] as (i32) << 3i32) + kSigned3BitContextLookup[
                                    p2 as (usize)
                                ] as (i32)) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_UTF8 as (i32) {
        (kUTF8ContextLookup[p1 as (usize)] as (i32) | kUTF8ContextLookup[
                                                          (p2 as (i32) + 256i32) as (usize)
                                                      ] as (i32)) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_MSB6 as (i32) {
        (p1 as (i32) >> 2i32) as (u8)
    } else if mode as (i32) == ContextType::CONTEXT_LSB6 as (i32) {
        (p1 as (i32) & 0x3fi32) as (u8)
    } else {
        0i32 as (u8)
    }
}

unsafe extern fn StoreSymbolWithContext(
    mut xself : *mut BlockEncoder,
    mut symbol : usize,
    mut context : usize,
    mut context_map : *const u32,
    mut storage_ix : *mut usize,
    mut storage : *mut u8,
    context_bits : usize
) {
    if (*xself).block_len_ == 0usize {
        let mut block_ix
            : usize
            = {
                  (*xself).block_ix_ = (*xself).block_ix_.wrapping_add(1 as (usize));
                  (*xself).block_ix_
              };
        let mut block_len
            : u32
            = *(*xself).block_lengths_.offset(block_ix as (isize));
        let mut block_type
            : u8
            = *(*xself).block_types_.offset(block_ix as (isize));
        (*xself).block_len_ = block_len as (usize);
        (*xself).entropy_ix_ = block_type as (usize) << context_bits;
        StoreBlockSwitch(
            &mut (*xself).block_split_code_ as (*mut BlockSplitCode),
            block_len,
            block_type,
            0i32,
            storage_ix,
            storage
        );
    }
    (*xself).block_len_ = (*xself).block_len_.wrapping_sub(1 as (usize));
    let mut histo_ix
        : usize
        = *context_map.offset(
               (*xself).entropy_ix_.wrapping_add(context) as (isize)
           ) as (usize);
    let mut ix
        : usize
        = histo_ix.wrapping_mul((*xself).alphabet_size_).wrapping_add(
              symbol
          );
    BrotliWriteBits(
        *(*xself).depths_.offset(ix as (isize)) as (usize),
        *(*xself).bits_.offset(ix as (isize)) as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn CommandCopyLen(mut xself : *const Command) -> u32 {
    (*xself).copy_len_ & 0xffffffu32
}

unsafe extern fn CommandDistanceContext(
    mut xself : *const Command
) -> u32 {
    let mut r : u32 = ((*xself).cmd_prefix_ as (i32) >> 6i32) as (u32);
    let mut c : u32 = ((*xself).cmd_prefix_ as (i32) & 7i32) as (u32);
    if (r == 0u32 || r == 2u32 || r == 4u32 || r == 7u32) && (c <= 2u32) {
        c
    } else {
        3u32
    }
}

unsafe extern fn CleanupBlockEncoder(
    mut m : *mut MemoryManager, mut xself : *mut BlockEncoder
) {
    BrotliFree(m,(*xself).depths_ as (*mut ::std::os::raw::c_void));
    (*xself).depths_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    BrotliFree(m,(*xself).bits_ as (*mut ::std::os::raw::c_void));
    (*xself).bits_ = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16);
}

unsafe extern fn JumpToByteBoundary(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    *storage_ix = (*storage_ix).wrapping_add(
                      7u32 as (usize)
                  ) & !7u32 as (usize);
    *storage.offset((*storage_ix >> 3i32) as (isize)) = 0i32 as (u8);
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlock(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut prev_byte : u8,
    mut prev_byte2 : u8,
    mut is_last : i32,
    mut num_direct_distance_codes : u32,
    mut distance_postfix_bits : u32,
    mut literal_context_mode : ContextType,
    mut commands : *const Command,
    mut n_commands : usize,
    mut mb : *const MetaBlockSplit,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    let mut num_distance_codes
        : usize
        = (16u32).wrapping_add(
              num_direct_distance_codes
          ).wrapping_add(
              48u32 << distance_postfix_bits
          ) as (usize);
    let mut tree : *mut HuffmanTree;
    let mut literal_enc : BlockEncoder;
    let mut command_enc : BlockEncoder;
    let mut distance_enc : BlockEncoder;
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    tree = if 2i32 * 704i32 + 1i32 != 0 {
               BrotliAllocate(
                   m,
                   ((2i32 * 704i32 + 1i32) as (usize)).wrapping_mul(
                       ::std::mem::size_of::<HuffmanTree>()
                   )
               ) as (*mut HuffmanTree)
           } else {
               0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree)
           };
    if !(0i32 == 0) {
    } else {
        InitBlockEncoder(
            &mut literal_enc as (*mut BlockEncoder),
            256usize,
            (*mb).literal_split.num_types,
            (*mb).literal_split.types as (*const u8),
            (*mb).literal_split.lengths as (*const u32),
            (*mb).literal_split.num_blocks
        );
        InitBlockEncoder(
            &mut command_enc as (*mut BlockEncoder),
            704usize,
            (*mb).command_split.num_types,
            (*mb).command_split.types as (*const u8),
            (*mb).command_split.lengths as (*const u32),
            (*mb).command_split.num_blocks
        );
        InitBlockEncoder(
            &mut distance_enc as (*mut BlockEncoder),
            num_distance_codes,
            (*mb).distance_split.num_types,
            (*mb).distance_split.types as (*const u8),
            (*mb).distance_split.lengths as (*const u32),
            (*mb).distance_split.num_blocks
        );
        BuildAndStoreBlockSwitchEntropyCodes(
            &mut literal_enc as (*mut BlockEncoder),
            tree,
            storage_ix,
            storage
        );
        BuildAndStoreBlockSwitchEntropyCodes(
            &mut command_enc as (*mut BlockEncoder),
            tree,
            storage_ix,
            storage
        );
        BuildAndStoreBlockSwitchEntropyCodes(
            &mut distance_enc as (*mut BlockEncoder),
            tree,
            storage_ix,
            storage
        );
        BrotliWriteBits(
            2usize,
            distance_postfix_bits as (usize),
            storage_ix,
            storage
        );
        BrotliWriteBits(
            4usize,
            (num_direct_distance_codes >> distance_postfix_bits) as (usize),
            storage_ix,
            storage
        );
        i = 0usize;
        'loop2: loop {
            if i < (*mb).literal_split.num_types {
                BrotliWriteBits(
                    2usize,
                    literal_context_mode as (usize),
                    storage_ix,
                    storage
                );
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        if (*mb).literal_context_map_size == 0usize {
            StoreTrivialContextMap(
                (*mb).literal_histograms_size,
                6usize,
                tree,
                storage_ix,
                storage
            );
        } else {
            EncodeContextMap(
                m,
                (*mb).literal_context_map as (*const u32),
                (*mb).literal_context_map_size,
                (*mb).literal_histograms_size,
                tree,
                storage_ix,
                storage
            );
            if !(0i32 == 0) {
                return;
            }
        }
        if (*mb).distance_context_map_size == 0usize {
            StoreTrivialContextMap(
                (*mb).distance_histograms_size,
                2usize,
                tree,
                storage_ix,
                storage
            );
        } else {
            EncodeContextMap(
                m,
                (*mb).distance_context_map as (*const u32),
                (*mb).distance_context_map_size,
                (*mb).distance_histograms_size,
                tree,
                storage_ix,
                storage
            );
            if !(0i32 == 0) {
                return;
            }
        }
        BuildAndStoreEntropyCodesLiteral(
            m,
            &mut literal_enc as (*mut BlockEncoder),
            (*mb).literal_histograms as (*const HistogramLiteral),
            (*mb).literal_histograms_size,
            tree,
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
        } else {
            BuildAndStoreEntropyCodesCommand(
                m,
                &mut command_enc as (*mut BlockEncoder),
                (*mb).command_histograms as (*const HistogramCommand),
                (*mb).command_histograms_size,
                tree,
                storage_ix,
                storage
            );
            if !(0i32 == 0) {
            } else {
                BuildAndStoreEntropyCodesDistance(
                    m,
                    &mut distance_enc as (*mut BlockEncoder),
                    (*mb).distance_histograms as (*const HistogramDistance),
                    (*mb).distance_histograms_size,
                    tree,
                    storage_ix,
                    storage
                );
                if !(0i32 == 0) {
                } else {
                    BrotliFree(m,tree as (*mut ::std::os::raw::c_void));
                    tree = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree);
                    i = 0usize;
                    'loop15: loop {
                        if i < n_commands {
                            let cmd : Command = *commands.offset(i as (isize));
                            let mut cmd_code : usize = cmd.cmd_prefix_ as (usize);
                            StoreSymbol(
                                &mut command_enc as (*mut BlockEncoder),
                                cmd_code,
                                storage_ix,
                                storage
                            );
                            StoreCommandExtra(&cmd as (*const Command),storage_ix,storage);
                            if (*mb).literal_context_map_size == 0usize {
                                let mut j : usize;
                                j = cmd.insert_len_ as (usize);
                                'loop25: loop {
                                    if j != 0usize {
                                        StoreSymbol(
                                            &mut literal_enc as (*mut BlockEncoder),
                                            *input.offset((pos & mask) as (isize)) as (usize),
                                            storage_ix,
                                            storage
                                        );
                                        pos = pos.wrapping_add(1 as (usize));
                                        j = j.wrapping_sub(1 as (usize));
                                        continue 'loop25;
                                    } else {
                                        break 'loop25;
                                    }
                                }
                            } else {
                                let mut j : usize;
                                j = cmd.insert_len_ as (usize);
                                'loop21: loop {
                                    if j != 0usize {
                                        let mut context
                                            : usize
                                            = Context(
                                                  prev_byte,
                                                  prev_byte2,
                                                  literal_context_mode
                                              ) as (usize);
                                        let mut literal
                                            : u8
                                            = *input.offset((pos & mask) as (isize));
                                        StoreSymbolWithContext(
                                            &mut literal_enc as (*mut BlockEncoder),
                                            literal as (usize),
                                            context,
                                            (*mb).literal_context_map as (*const u32),
                                            storage_ix,
                                            storage,
                                            6usize
                                        );
                                        prev_byte2 = prev_byte;
                                        prev_byte = literal;
                                        pos = pos.wrapping_add(1 as (usize));
                                        j = j.wrapping_sub(1 as (usize));
                                        continue 'loop21;
                                    } else {
                                        break 'loop21;
                                    }
                                }
                            }
                            pos = pos.wrapping_add(
                                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                                  );
                            if CommandCopyLen(&cmd as (*const Command)) != 0 {
                                prev_byte2 = *input.offset(
                                                  (pos.wrapping_sub(
                                                       2usize
                                                   ) & mask) as (isize)
                                              );
                                prev_byte = *input.offset(
                                                 (pos.wrapping_sub(
                                                      1usize
                                                  ) & mask) as (isize)
                                             );
                                if cmd.cmd_prefix_ as (i32) >= 128i32 {
                                    let mut dist_code : usize = cmd.dist_prefix_ as (usize);
                                    let mut distnumextra : u32 = cmd.dist_extra_ >> 24i32;
                                    let mut distextra
                                        : usize
                                        = (cmd.dist_extra_ & 0xffffffu32) as (usize);
                                    if (*mb).distance_context_map_size == 0usize {
                                        StoreSymbol(
                                            &mut distance_enc as (*mut BlockEncoder),
                                            dist_code,
                                            storage_ix,
                                            storage
                                        );
                                    } else {
                                        let mut context
                                            : usize
                                            = CommandDistanceContext(
                                                  &cmd as (*const Command)
                                              ) as (usize);
                                        StoreSymbolWithContext(
                                            &mut distance_enc as (*mut BlockEncoder),
                                            dist_code,
                                            context,
                                            (*mb).distance_context_map as (*const u32),
                                            storage_ix,
                                            storage,
                                            2usize
                                        );
                                    }
                                    BrotliWriteBits(
                                        distnumextra as (usize),
                                        distextra,
                                        storage_ix,
                                        storage
                                    );
                                }
                            }
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop15;
                        } else {
                            break 'loop15;
                        }
                    }
                    CleanupBlockEncoder(m,&mut distance_enc as (*mut BlockEncoder));
                    CleanupBlockEncoder(m,&mut command_enc as (*mut BlockEncoder));
                    CleanupBlockEncoder(m,&mut literal_enc as (*mut BlockEncoder));
                    if is_last != 0 {
                        JumpToByteBoundary(storage_ix,storage);
                    }
                }
            }
        }
    }
}

unsafe extern fn HistogramClearLiteral(
    mut xself : *mut HistogramLiteral
) {
    memset(
        (*xself).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 256]>()
    );
    (*xself).total_count_ = 0usize;
    (*xself).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramClearCommand(
    mut xself : *mut HistogramCommand
) {
    memset(
        (*xself).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 704]>()
    );
    (*xself).total_count_ = 0usize;
    (*xself).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramClearDistance(
    mut xself : *mut HistogramDistance
) {
    memset(
        (*xself).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 520]>()
    );
    (*xself).total_count_ = 0usize;
    (*xself).bit_cost_ = 3.402e+38f64;
}

unsafe extern fn HistogramAddCommand(
    mut xself : *mut HistogramCommand, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*xself).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*xself).total_count_ = (*xself).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddLiteral(
    mut xself : *mut HistogramLiteral, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*xself).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*xself).total_count_ = (*xself).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn HistogramAddDistance(
    mut xself : *mut HistogramDistance, mut val : usize
) {
    {
        let _rhs = 1;
        let _lhs = &mut (*xself).data_[val];
        *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
    }
    (*xself).total_count_ = (*xself).total_count_.wrapping_add(
                               1 as (usize)
                           );
}

unsafe extern fn BuildHistograms(
    mut input : *const u8,
    mut start_pos : usize,
    mut mask : usize,
    mut commands : *const Command,
    mut n_commands : usize,
    mut lit_histo : *mut HistogramLiteral,
    mut cmd_histo : *mut HistogramCommand,
    mut dist_histo : *mut HistogramDistance
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < n_commands {
            let cmd : Command = *commands.offset(i as (isize));
            let mut j : usize;
            HistogramAddCommand(cmd_histo,cmd.cmd_prefix_ as (usize));
            j = cmd.insert_len_ as (usize);
            'loop4: loop {
                if j != 0usize {
                    HistogramAddLiteral(
                        lit_histo,
                        *input.offset((pos & mask) as (isize)) as (usize)
                    );
                    pos = pos.wrapping_add(1 as (usize));
                    j = j.wrapping_sub(1 as (usize));
                    continue 'loop4;
                } else {
                    break 'loop4;
                }
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(
                   &cmd as (*const Command)
               ) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
                HistogramAddDistance(dist_histo,cmd.dist_prefix_ as (usize));
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn StoreDataWithHuffmanCodes(
    mut input : *const u8,
    mut start_pos : usize,
    mut mask : usize,
    mut commands : *const Command,
    mut n_commands : usize,
    mut lit_depth : *const u8,
    mut lit_bits : *const u16,
    mut cmd_depth : *const u8,
    mut cmd_bits : *const u16,
    mut dist_depth : *const u8,
    mut dist_bits : *const u16,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut pos : usize = start_pos;
    let mut i : usize;
    i = 0usize;
    'loop1: loop {
        if i < n_commands {
            let cmd : Command = *commands.offset(i as (isize));
            let cmd_code : usize = cmd.cmd_prefix_ as (usize);
            let mut j : usize;
            BrotliWriteBits(
                *cmd_depth.offset(cmd_code as (isize)) as (usize),
                *cmd_bits.offset(cmd_code as (isize)) as (usize),
                storage_ix,
                storage
            );
            StoreCommandExtra(&cmd as (*const Command),storage_ix,storage);
            j = cmd.insert_len_ as (usize);
            'loop4: loop {
                if j != 0usize {
                    let literal : u8 = *input.offset((pos & mask) as (isize));
                    BrotliWriteBits(
                        *lit_depth.offset(literal as (isize)) as (usize),
                        *lit_bits.offset(literal as (isize)) as (usize),
                        storage_ix,
                        storage
                    );
                    pos = pos.wrapping_add(1 as (usize));
                    j = j.wrapping_sub(1 as (usize));
                    continue 'loop4;
                } else {
                    break 'loop4;
                }
            }
            pos = pos.wrapping_add(
                      CommandCopyLen(&cmd as (*const Command)) as (usize)
                  );
            if CommandCopyLen(
                   &cmd as (*const Command)
               ) != 0 && (cmd.cmd_prefix_ as (i32) >= 128i32) {
                let dist_code : usize = cmd.dist_prefix_ as (usize);
                let distnumextra : u32 = cmd.dist_extra_ >> 24i32;
                let distextra : u32 = cmd.dist_extra_ & 0xffffffu32;
                BrotliWriteBits(
                    *dist_depth.offset(dist_code as (isize)) as (usize),
                    *dist_bits.offset(dist_code as (isize)) as (usize),
                    storage_ix,
                    storage
                );
                BrotliWriteBits(
                    distnumextra as (usize),
                    distextra as (usize),
                    storage_ix,
                    storage
                );
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlockTrivial(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut is_last : i32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lit_histo : HistogramLiteral;
    let mut cmd_histo : HistogramCommand;
    let mut dist_histo : HistogramDistance;
    let mut lit_depth : [u8; 256];
    let mut lit_bits : [u16; 256];
    let mut cmd_depth : [u8; 704];
    let mut cmd_bits : [u16; 704];
    let mut dist_depth : [u8; 64];
    let mut dist_bits : [u16; 64];
    let mut tree : *mut HuffmanTree;
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    HistogramClearLiteral(&mut lit_histo as (*mut HistogramLiteral));
    HistogramClearCommand(&mut cmd_histo as (*mut HistogramCommand));
    HistogramClearDistance(
        &mut dist_histo as (*mut HistogramDistance)
    );
    BuildHistograms(
        input,
        start_pos,
        mask,
        commands,
        n_commands,
        &mut lit_histo as (*mut HistogramLiteral),
        &mut cmd_histo as (*mut HistogramCommand),
        &mut dist_histo as (*mut HistogramDistance)
    );
    BrotliWriteBits(
        13usize,
        0usize,
        storage_ix,
        storage
    );
    tree = if 2i32 * 704i32 + 1i32 != 0 {
               BrotliAllocate(
                   m,
                   ((2i32 * 704i32 + 1i32) as (usize)).wrapping_mul(
                       ::std::mem::size_of::<HuffmanTree>()
                   )
               ) as (*mut HuffmanTree)
           } else {
               0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree)
           };
    if !(0i32 == 0) {
    } else {
        BuildAndStoreHuffmanTree(
            lit_histo.data_.as_mut_ptr() as (*const u32),
            256usize,
            tree,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        BuildAndStoreHuffmanTree(
            cmd_histo.data_.as_mut_ptr() as (*const u32),
            704usize,
            tree,
            cmd_depth.as_mut_ptr(),
            cmd_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        BuildAndStoreHuffmanTree(
            dist_histo.data_.as_mut_ptr() as (*const u32),
            64usize,
            tree,
            dist_depth.as_mut_ptr(),
            dist_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        BrotliFree(m,tree as (*mut ::std::os::raw::c_void));
        tree = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HuffmanTree);
        StoreDataWithHuffmanCodes(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            lit_depth.as_mut_ptr() as (*const u8),
            lit_bits.as_mut_ptr() as (*const u16),
            cmd_depth.as_mut_ptr() as (*const u8),
            cmd_bits.as_mut_ptr() as (*const u16),
            dist_depth.as_mut_ptr() as (*const u8),
            dist_bits.as_mut_ptr() as (*const u16),
            storage_ix,
            storage
        );
        if is_last != 0 {
            JumpToByteBoundary(storage_ix,storage);
        }
    }
}

unsafe extern fn StoreStaticCommandHuffmanTree(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        56usize,
        0x926244u32 as (usize) << 32i32 | 0x16307003u32 as (usize),
        storage_ix,
        storage
    );
    BrotliWriteBits(
        3usize,
        0x0u32 as (usize),
        storage_ix,
        storage
    );
}

unsafe extern fn StoreStaticDistanceHuffmanTree(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        28usize,
        0x369dc03u32 as (usize),
        storage_ix,
        storage
    );
}

#[no_mangle]
pub unsafe extern fn BrotliStoreMetaBlockFast(
    mut m : *mut MemoryManager,
    mut input : *const u8,
    mut start_pos : usize,
    mut length : usize,
    mut mask : usize,
    mut is_last : i32,
    mut commands : *const Command,
    mut n_commands : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    StoreCompressedMetaBlockHeader(is_last,length,storage_ix,storage);
    BrotliWriteBits(
        13usize,
        0usize,
        storage_ix,
        storage
    );
    if n_commands <= 128usize {
        let mut histogram
            : [u32; 256]
            = [0u32; 256];
        let mut pos : usize = start_pos;
        let mut num_literals : usize = 0usize;
        let mut i : usize;
        let mut lit_depth : [u8; 256];
        let mut lit_bits : [u16; 256];
        i = 0usize;
        'loop9: loop {
            if i < n_commands {
                let cmd : Command = *commands.offset(i as (isize));
                let mut j : usize;
                j = cmd.insert_len_ as (usize);
                'loop17: loop {
                    if j != 0usize {
                        {
                            let _rhs = 1;
                            let _lhs
                                = &mut histogram[
                                           *input.offset((pos & mask) as (isize)) as (usize)
                                       ];
                            *_lhs = (*_lhs).wrapping_add(_rhs as (u32));
                        }
                        pos = pos.wrapping_add(1 as (usize));
                        j = j.wrapping_sub(1 as (usize));
                        continue 'loop17;
                    } else {
                        break 'loop17;
                    }
                }
                num_literals = num_literals.wrapping_add(
                                   cmd.insert_len_ as (usize)
                               );
                pos = pos.wrapping_add(
                          CommandCopyLen(&cmd as (*const Command)) as (usize)
                      );
                i = i.wrapping_add(1 as (usize));
                continue 'loop9;
            } else {
                break 'loop9;
            }
        }
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            histogram.as_mut_ptr() as (*const u32),
            num_literals,
            8usize,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        } else {
            StoreStaticCommandHuffmanTree(storage_ix,storage);
            StoreStaticDistanceHuffmanTree(storage_ix,storage);
            StoreDataWithHuffmanCodes(
                input,
                start_pos,
                mask,
                commands,
                n_commands,
                lit_depth.as_mut_ptr() as (*const u8),
                lit_bits.as_mut_ptr() as (*const u16),
                kStaticCommandCodeDepth.as_ptr(),
                kStaticCommandCodeBits.as_ptr(),
                kStaticDistanceCodeDepth.as_ptr(),
                kStaticDistanceCodeBits.as_ptr(),
                storage_ix,
                storage
            );
        }
    } else {
        let mut lit_histo : HistogramLiteral;
        let mut cmd_histo : HistogramCommand;
        let mut dist_histo : HistogramDistance;
        let mut lit_depth : [u8; 256];
        let mut lit_bits : [u16; 256];
        let mut cmd_depth : [u8; 704];
        let mut cmd_bits : [u16; 704];
        let mut dist_depth : [u8; 64];
        let mut dist_bits : [u16; 64];
        HistogramClearLiteral(&mut lit_histo as (*mut HistogramLiteral));
        HistogramClearCommand(&mut cmd_histo as (*mut HistogramCommand));
        HistogramClearDistance(
            &mut dist_histo as (*mut HistogramDistance)
        );
        BuildHistograms(
            input,
            start_pos,
            mask,
            commands,
            n_commands,
            &mut lit_histo as (*mut HistogramLiteral),
            &mut cmd_histo as (*mut HistogramCommand),
            &mut dist_histo as (*mut HistogramDistance)
        );
        BrotliBuildAndStoreHuffmanTreeFast(
            m,
            lit_histo.data_.as_mut_ptr() as (*const u32),
            lit_histo.total_count_,
            8usize,
            lit_depth.as_mut_ptr(),
            lit_bits.as_mut_ptr(),
            storage_ix,
            storage
        );
        if !(0i32 == 0) {
            return;
        } else {
            BrotliBuildAndStoreHuffmanTreeFast(
                m,
                cmd_histo.data_.as_mut_ptr() as (*const u32),
                cmd_histo.total_count_,
                10usize,
                cmd_depth.as_mut_ptr(),
                cmd_bits.as_mut_ptr(),
                storage_ix,
                storage
            );
            if !(0i32 == 0) {
                return;
            } else {
                BrotliBuildAndStoreHuffmanTreeFast(
                    m,
                    dist_histo.data_.as_mut_ptr() as (*const u32),
                    dist_histo.total_count_,
                    6usize,
                    dist_depth.as_mut_ptr(),
                    dist_bits.as_mut_ptr(),
                    storage_ix,
                    storage
                );
                if !(0i32 == 0) {
                    return;
                } else {
                    StoreDataWithHuffmanCodes(
                        input,
                        start_pos,
                        mask,
                        commands,
                        n_commands,
                        lit_depth.as_mut_ptr() as (*const u8),
                        lit_bits.as_mut_ptr() as (*const u16),
                        cmd_depth.as_mut_ptr() as (*const u8),
                        cmd_bits.as_mut_ptr() as (*const u16),
                        dist_depth.as_mut_ptr() as (*const u8),
                        dist_bits.as_mut_ptr() as (*const u16),
                        storage_ix,
                        storage
                    );
                }
            }
        }
    }
    if is_last != 0 {
        JumpToByteBoundary(storage_ix,storage);
    }
}

unsafe extern fn BrotliStoreUncompressedMetaBlockHeader(
    mut length : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut lenbits : usize;
    let mut nlenbits : usize;
    let mut nibblesbits : usize;
    BrotliWriteBits(
        1usize,
        0usize,
        storage_ix,
        storage
    );
    BrotliEncodeMlen(
        length,
        &mut lenbits as (*mut usize),
        &mut nlenbits as (*mut usize),
        &mut nibblesbits as (*mut usize)
    );
    BrotliWriteBits(2usize,nibblesbits,storage_ix,storage);
    BrotliWriteBits(nlenbits,lenbits,storage_ix,storage);
    BrotliWriteBits(
        1usize,
        1usize,
        storage_ix,
        storage
    );
}

unsafe extern fn BrotliWriteBitsPrepareStorage(
    mut pos : usize, mut array : *mut u8
) {
    if pos & 7usize == 0usize {
        0i32;
    } else {
        __assert_fail(
            (*b"(pos & 7) == 0\0").as_ptr(),
            file!().as_ptr(),
            line!(),
            (*b"BrotliWriteBitsPrepareStorage\0").as_ptr()
        );
    }
    *array.offset((pos >> 3i32) as (isize)) = 0i32 as (u8);
}

#[no_mangle]
pub unsafe extern fn BrotliStoreUncompressedMetaBlock(
    mut is_final_block : i32,
    mut input : *const u8,
    mut position : usize,
    mut mask : usize,
    mut len : usize,
    mut storage_ix : *mut usize,
    mut storage : *mut u8
) {
    let mut masked_pos : usize = position & mask;
    BrotliStoreUncompressedMetaBlockHeader(len,storage_ix,storage);
    JumpToByteBoundary(storage_ix,storage);
    if masked_pos.wrapping_add(len) > mask.wrapping_add(
                                          1usize
                                      ) {
        let mut len1
            : usize
            = mask.wrapping_add(1usize).wrapping_sub(masked_pos);
        memcpy(
            &mut *storage.offset(
                      (*storage_ix >> 3i32) as (isize)
                  ) as (*mut u8) as (*mut ::std::os::raw::c_void),
            &*input.offset(
                  masked_pos as (isize)
              ) as (*const u8) as (*const ::std::os::raw::c_void),
            len1
        );
        *storage_ix = (*storage_ix).wrapping_add(len1 << 3i32);
        len = len.wrapping_sub(len1);
        masked_pos = 0usize;
    }
    memcpy(
        &mut *storage.offset(
                  (*storage_ix >> 3i32) as (isize)
              ) as (*mut u8) as (*mut ::std::os::raw::c_void),
        &*input.offset(
              masked_pos as (isize)
          ) as (*const u8) as (*const ::std::os::raw::c_void),
        len
    );
    *storage_ix = (*storage_ix).wrapping_add(len << 3i32);
    BrotliWriteBitsPrepareStorage(*storage_ix,storage);
    if is_final_block != 0 {
        BrotliWriteBits(
            1usize,
            1usize,
            storage_ix,
            storage
        );
        BrotliWriteBits(
            1usize,
            1usize,
            storage_ix,
            storage
        );
        JumpToByteBoundary(storage_ix,storage);
    }
}

#[no_mangle]
pub unsafe extern fn BrotliStoreSyncMetaBlock(
    mut storage_ix : *mut usize, mut storage : *mut u8
) {
    BrotliWriteBits(
        6usize,
        6usize,
        storage_ix,
        storage
    );
    JumpToByteBoundary(storage_ix,storage);
}
