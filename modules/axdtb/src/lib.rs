// Copyright (c) 2018 Colin Finck, RWTH Aachen University
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]
#![no_std]

// MACROS

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use core::result::Result;
use core::str::from_utf8;
use axlog::ax_println;
// 参考类型定义
pub struct DtbInfo {
    pub memory_addr: usize,
    pub memory_size: usize,
    pub mmio_regions: Vec<(usize, usize)>,
}
use crate::alloc::string::ToString;
// 定义解析 DTB 并提取相关信息的函数
pub fn parse_dtb(dtb_pa: usize) -> Result<DtbInfo,String> {

  
    // 从给定的物理地址解析 DTB
    let dtb_address: *const u8 = dtb_pa as *const u8;
    unsafe {
        if let Some(dtb) = Dtb::from_raw(dtb_address) {
            // for node in dtb.enum_properties("/soc/virtio_mmio") {
            //     ax_println!("{}", node);
            // }
            let mut memory_addr =  0 ;
            let mut memory_size =  0 ;
            let mut mmio_regions = Vec::new();
            //ax_println!("{}",)
            // if let Ok(path) = str::from_utf8(dtb.struct_slice) {
            //      // 假设 memory_addr 存储在名为 "memory" 的属性中
                 if let Some(memory_addr_property) = dtb.get_property("/memory", "reg") {
                    // 将属性的内容（假设是一个包含地址的字符串）转换为 usize 类型
                    let mut memory_addr1:Vec<u8> = Vec::new();
                    let mut memory_size1:Vec<u8> = Vec::new();
                    let mut cnt = 0;
                    for byte in memory_addr_property
                    {
                       // ax_println!("{}",*byte);
                        if cnt <=7 
                        {
                        memory_addr1.push(*byte);
                        }
                        else 
                        {
                        memory_size1.push(*byte);
                        }
                        cnt+=1;
                    }
                    //ax_println!("{:?}",memory_size1);
                    let mut memory_addr2: [u8; 8] = [0; 8]; // 创建一个固定长度的数组并初始化为0
                    memory_addr2.copy_from_slice(&memory_addr1[..8]); // 从 Vec<u8> 中复制前8个字节到数组中
                    memory_addr = usize::from_be_bytes(memory_addr2);
                    //memory_addr = usize::from_be_bytes(memory_addr1.as_slice());
                   // ax_println!("{}",memory_addr);
                   let mut memory_size2: [u8; 8] = [0; 8]; // 创建一个固定长度的数组并初始化为0
                   memory_size2.copy_from_slice(&memory_size1[..8]); // 从 Vec<u8> 中复制前8个字节到数组中
                   memory_size = usize::from_be_bytes(memory_size2);
                }
                let mut addv = 8000;
                loop 
                { 
                    let mut path = "/soc/virtio_mmio@1000".to_string();
                    path.push_str(&(addv.to_string()));
                    if addv == 0  {break;}
                    addv -= 1000;
                if let Some(mmio_property) = dtb.get_property(&path, "reg") {
                    // 将属性内容解析为 mmio 地址和大小的元组列表
                    //ax_println!("{:?}",mmio_property);
                    let mut addr1:Vec<u8> = Vec::new();
                    let mut size1:Vec<u8> = Vec::new();
                    let mut addr:usize = 0;
                    let mut size:usize = 0;
                    let mut cnt = 0;
                    for byte in mmio_property
                    {
                        //ax_println!("{}",*byte);
                        if cnt <=7 
                        {
                        addr1.push(*byte);
                        }
                        else 
                        {
                        size1.push(*byte);
                        }
                        cnt+=1;
                    }
                    //ax_println!("{:?}",memory_size1);
                    let mut addr2: [u8; 8] = [0; 8]; // 创建一个固定长度的数组并初始化为0
                    addr2.copy_from_slice(&addr1[..8]); // 从 Vec<u8> 中复制前8个字节到数组中
                    addr = usize::from_be_bytes(addr2);
                    //memory_addr = usize::from_be_bytes(memory_addr1.as_slice());
                   // ax_println!("{}",memory_addr);
                   let mut size2: [u8; 8] = [0; 8]; // 创建一个固定长度的数组并初始化为0
                   size2.copy_from_slice(&size1[..8]); // 从 Vec<u8> 中复制前8个字节到数组中
                   size = usize::from_be_bytes(size2);
                   mmio_regions.push((addr,size));
                }  
                else 
                {
                    mmio_regions.push((1,2));
                }
            }
              
            

            

            // 返回用提取的信息填充的 DtbInfo
            Ok(DtbInfo {
                memory_addr,
                memory_size,
                mmio_regions,
            })
        } else {
            Err("dtb error".to_string())
        }
    }
}
macro_rules! align_down {
    ($value:expr, $alignment:expr) => {
        $value & !($alignment - 1)
    };
}

macro_rules! align_up {
    ($value:expr, $alignment:expr) => {
        align_down!($value + ($alignment - 1), $alignment)
    };
}

// IMPORTS
use core::{cmp, mem, slice, str};

// FUNCTIONS
/// Get the length of a C-style null-terminated byte string, which is part of a larger slice.
/// The latter requirement makes this function safe to use.
fn c_strlen_on_slice(slice: &[u8]) -> usize {
    let mut end = slice;
    while !end.is_empty() && *end.first().unwrap_or(&0) != 0 {
        end = &end[1..];
    }

    end.as_ptr() as usize - slice.as_ptr() as usize
}

/// Get the token and advance the struct_slice to the next token.
fn parse_token(struct_slice: &mut &[u8]) -> u32 {
    let (token_slice, remaining_slice) = struct_slice.split_at(mem::size_of::<u32>());
    *struct_slice = remaining_slice;

    u32::from_be_bytes(token_slice.try_into().unwrap())
}

/// Get the node name of a FDT_BEGIN_NODE token and advance the struct_slice to the next token.
fn parse_begin_node<'a>(struct_slice: &mut &'a [u8]) -> &'a str {
    let node_name_length = c_strlen_on_slice(struct_slice);
    let node_name = str::from_utf8(&struct_slice[..node_name_length]).unwrap();
    let aligned_length = align_up!(node_name_length + 1, mem::size_of::<u32>());
    *struct_slice = &struct_slice[aligned_length..];

    node_name
}

/// Get the property data length of a FDT_PROP token and advance the struct_slice to the property name offset.
fn parse_prop_data_length(struct_slice: &mut &[u8]) -> usize {
    let (property_length_slice, remaining_slice) = struct_slice.split_at(mem::size_of::<u32>());
    *struct_slice = remaining_slice;

    u32::from_be_bytes(property_length_slice.try_into().unwrap()) as usize
}

/// Get the property name of a FDT_PROP token and advance the struct_slice to the next token.
fn parse_prop_name<'a>(struct_slice: &mut &[u8], strings_slice: &'a [u8]) -> &'a str {
    // Get the offset of the property name string inside strings_slice.
    let (property_name_offset_slice, remaining_slice) =
        struct_slice.split_at(mem::size_of::<u32>());
    *struct_slice = remaining_slice;
    let property_name_offset =
        u32::from_be_bytes(property_name_offset_slice.try_into().unwrap()) as usize;

    // Determine the length of that null-terminated string and return it.
    let property_name_slice = &strings_slice[property_name_offset..];
    let property_name_length = c_strlen_on_slice(property_name_slice);
    let property_name = str::from_utf8(&property_name_slice[..property_name_length]).unwrap();

    property_name
}

// CONSTANTS
const DTB_MAGIC: u32 = 0xD00DFEED;
const DTB_VERSION: u32 = 17;

const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;
const FDT_NOP: u32 = 0x00000004;
const FDT_END: u32 = 0x00000009;

// STRUCTURES
#[repr(C)]
struct DtbHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

pub struct Dtb<'a> {
    pub header: &'a DtbHeader,
    pub struct_slice: &'a [u8],
    pub strings_slice: &'a [u8],
}

impl<'a> Dtb<'a> {
// 这个函数 check_header 的作用是检查给定的 DtbHeader 结构是否符合特定的规范。
    fn check_header(header: &DtbHeader) -> bool {
        u32::from_be(header.magic) == DTB_MAGIC && u32::from_be(header.version) == DTB_VERSION
    }

//这段代码实现了一个 from_raw 方法，它是一个不安全函数，用于从原始内存地址构造一个可能的Dtb类型实例
    pub unsafe fn from_raw(address: *const u8) -> Option<Self> {
        let header = &*(address as *const DtbHeader);
        if !Self::check_header(header) {
            return None;
        }

        let address = header as *const _ as usize + u32::from_be(header.off_dt_struct) as usize;
        let length = u32::from_be(header.size_dt_struct) as usize;
        let struct_slice = slice::from_raw_parts(address as *const u8, length);

        let address = header as *const _ as usize + u32::from_be(header.off_dt_strings) as usize;
        let length = u32::from_be(header.size_dt_strings) as usize;
        let strings_slice = slice::from_raw_parts(address as *const u8, length);

        Some(Self {
            header,
            struct_slice,
            strings_slice,
        })
    }
    //enum_subnodes 的方法，用于枚举给定路径下的子节点。
    pub fn enum_subnodes<'b>(&self, path: &'b str) -> EnumSubnodesIter<'a, 'b> {
        assert!(!path.is_empty());

        EnumSubnodesIter {
            struct_slice: self.struct_slice,
            path,
            nesting_level: 0,
            looking_on_level: 1,
        }
    }
//这段代码定义了一个名为 enum_properties 的方法，用于枚举给定路径下的属性。
    pub fn enum_properties<'b>(&self, path: &'b str) -> EnumPropertiesIter<'a, 'b> {
        assert!(!path.is_empty());

        EnumPropertiesIter {
            struct_slice: self.struct_slice,
            strings_slice: self.strings_slice,
            path,
            nesting_level: 0,
            looking_on_level: 1,
        }
    }
//这段代码的作用是在设备树的结构体切片中按照指定路径和属性名查找属性，并返回其数据，如果找不到则返回 None。
    pub fn get_property(&self, path: &str, property: &str) -> Option<&'a [u8]> {
        let mut struct_slice = self.struct_slice;
        let mut path = path;
        let mut nesting_level = 0;
        let mut looking_on_level = 1;
        while !struct_slice.is_empty() {
            let token = parse_token(&mut struct_slice);
            match token {
                FDT_BEGIN_NODE => {
                    if path.is_empty() {
                        // This is a subnode of the node we have been looking for.
                        // The Flattened Device Tree Specification states that properties always precede subnodes, so we can stop.
                        struct_slice = &[];
                    } else {
                        // The beginning of a node starts a new nesting level.
                        nesting_level += 1;

                        // Get the node information and advance the cursor to the next token.
                        let node_name = parse_begin_node(&mut struct_slice);

                        // We're only interested in this node if it is on the nesting level we are looking for.
                        if looking_on_level == nesting_level {
                            // path is advanced with every path component that matches, so we can compare it against
                            // node_name using starts_with().
                            // But path can either contain a full node name (like "uart@fe001000") or leave out the
                            // unit address (like "uart@") to find the first UART device.
                            // Therefore, get the minimum of both lengths and only call starts_with() on that length.
                            let length_to_check = cmp::min(path.len(), node_name.len());
                            let name_to_check = &node_name[..length_to_check];

                            if node_name.is_empty() || path.starts_with(name_to_check) {
                                // The current node is either the root node (node_name.is_empty()) or a matching path
                                // component.
                                // Advance path and the nesting level we are looking for.
                                path = &path[length_to_check..];
                                if path.starts_with('/') {
                                    // Skip the slash.
                                    path = &path[1..];
                                }

                                looking_on_level += 1;
                            }
                        }
                    }
                }

                FDT_END_NODE => {
                    // Finish this nesting level.
                    nesting_level -= 1;

                    if path.is_empty() {
                        // If path is empty and we encounter the end of a nesting level, we have iterated over
                        // all properties of the node we were looking for and can stop.
                        struct_slice = &[];
                    }
                }

                FDT_PROP => {
                    // Get the property data length.
                    let property_length = parse_prop_data_length(&mut struct_slice);
                    let aligned_length = align_up!(property_length, mem::size_of::<u32>());

                    if path.is_empty() {
                        // We have reached the node we are looking for.
                        // Now get the property_name to also check if this is the property we are looking for.
                        let property_name = parse_prop_name(&mut struct_slice, self.strings_slice);

                        if property_name == property {
                            // It is, so get the data and return it.
                            let property_data = &struct_slice[..property_length];
                            return Some(property_data);
                        } else {
                            // It is not, so just advance the cursor.
                            struct_slice = &struct_slice[aligned_length..];
                        }
                    } else {
                        // Skip over the property name offset and data.
                        struct_slice = &struct_slice[mem::size_of::<u32>()..];
                        struct_slice = &struct_slice[aligned_length..];
                    }
                }

                FDT_NOP => {
                    // Nothing to be done for NOPs.
                }

                FDT_END => {
                    // This marks the end of the device tree.
                    struct_slice = &[];
                }

                _ => {
                    panic!("get_property encountered an invalid token {:#010X} {} bytes before the end", token, struct_slice.len());
                }
            }
        }

        None
    }
}

pub struct EnumSubnodesIter<'a, 'b> {
    struct_slice: &'a [u8],
    path: &'b str,
    nesting_level: usize,
    looking_on_level: usize,
}

impl<'a, 'b> Iterator for EnumSubnodesIter<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        while !self.struct_slice.is_empty() {
            let token = parse_token(&mut self.struct_slice);
            match token {
                FDT_BEGIN_NODE => {
                    // The beginning of a node starts a new nesting level.
                    self.nesting_level += 1;

                    // Get the node information and advance the cursor to the next token.
                    let node_name = parse_begin_node(&mut self.struct_slice);

                    // We're only interested in this node if it is on the nesting level we are looking for.
                    if self.looking_on_level == self.nesting_level {
                        if self.path.is_empty() {
                            // self.path is empty and we are on the right nesting level, so this is a subnode
                            // we are looking for.
                            return Some(node_name);
                        } else {
                            // self.path is advanced with every path component that matches, so we can compare it against
                            // node_name using starts_with().
                            // But self.path can either contain a full node name (like "uart@fe001000") or leave out the
                            // unit address (like "uart@") to find the first UART device.
                            // Therefore, get the minimum of both lengths and only call starts_with() on that length.
                            let length_to_check = cmp::min(self.path.len(), node_name.len());
                            let name_to_check = &node_name[..length_to_check];

                            if node_name.is_empty() || self.path.starts_with(name_to_check) {
                                // The current node is either the root node (node_name.is_empty()) or a matching path
                                // component.
                                // Advance self.path and the nesting level we are looking for.
                                self.path = &self.path[length_to_check..];
                                if self.path.starts_with('/') {
                                    // Skip the slash.
                                    self.path = &self.path[1..];
                                }

                                self.looking_on_level += 1;
                            }
                        }
                    }
                }

                FDT_END_NODE => {
                    // Finish this nesting level.
                    self.nesting_level -= 1;

                    if self.nesting_level < self.looking_on_level - 1 {
                        // If the current nesting level is two levels below the level we are looking for,
                        // we have finished enumerating the parent node and can stop.
                        self.struct_slice = &[];
                    }
                }

                FDT_PROP => {
                    // EnumSubnodesIter is not interested in property information.
                    // Get the property data length.
                    let property_length = parse_prop_data_length(&mut self.struct_slice);
                    let aligned_length = align_up!(property_length, mem::size_of::<u32>());

                    // Skip over the property name offset and data.
                    self.struct_slice = &self.struct_slice[mem::size_of::<u32>()..];
                    self.struct_slice = &self.struct_slice[aligned_length..];
                }

                FDT_NOP => {
                    // Nothing to be done for NOPs.
                }

                FDT_END => {
                    // This marks the end of the device tree.
                    self.struct_slice = &[];
                }

                _ => {
                    panic!("EnumSubnodesIter encountered an invalid token {:#010X} {} bytes before the end", token, self.struct_slice.len());
                }
            }
        }

        None
    }
}

pub struct EnumPropertiesIter<'a, 'b> {
    struct_slice: &'a [u8],
    strings_slice: &'a [u8],
    path: &'b str,
    nesting_level: usize,
    looking_on_level: usize,
}

impl<'a, 'b> Iterator for EnumPropertiesIter<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        while !self.struct_slice.is_empty() {
            let token = parse_token(&mut self.struct_slice);
            match token {
                FDT_BEGIN_NODE => {
                    if self.path.is_empty() {
                        // This is a subnode of the node we have been looking for.
                        // The Flattened Device Tree Specification states that properties always precede subnodes, so we can stop.
                        self.struct_slice = &[];
                    } else {
                        // The beginning of a node starts a new nesting level.
                        self.nesting_level += 1;

                        // Get the node information and advance the cursor to the next token.
                        let node_name = parse_begin_node(&mut self.struct_slice);

                        // We're only interested in this node if it is on the nesting level we are looking for.
                        if self.looking_on_level == self.nesting_level {
                            // self.path is advanced with every path component that matches, so we can compare it against
                            // node_name using starts_with().
                            // But self.path can either contain a full node name (like "uart@fe001000") or leave out the
                            // unit address (like "uart@") to find the first UART device.
                            // Therefore, get the minimum of both lengths and only call starts_with() on that length.
                            let length_to_check = cmp::min(self.path.len(), node_name.len());
                            let name_to_check = &node_name[..length_to_check];

                            if node_name.is_empty() || self.path.starts_with(name_to_check) {
                                // The current node is either the root node (node_name.is_empty()) or a matching path
                                // component.
                                // Advance self.path and the nesting level we are looking for.
                                self.path = &self.path[length_to_check..];
                                if self.path.starts_with('/') {
                                    // Skip the slash.
                                    self.path = &self.path[1..];
                                }

                                self.looking_on_level += 1;
                            }
                        }
                    }
                }

                FDT_END_NODE => {
                    // Finish this nesting level.
                    self.nesting_level -= 1;

                    if self.path.is_empty() {
                        // If self.path is empty and we encounter the end of a nesting level, we have iterated over
                        // all properties of the node we were looking for and can stop.
                        self.struct_slice = &[];
                    }
                }

                FDT_PROP => {
                    // Get the property data length.
                    let property_length = parse_prop_data_length(&mut self.struct_slice);
                    let aligned_length = align_up!(property_length, mem::size_of::<u32>());

                    if self.path.is_empty() {
                        // We have reached the node we are looking for and this is a property to enumerate.
                        // So get the property name, skip over the data, and return the name.
                        let property_name =
                            parse_prop_name(&mut self.struct_slice, self.strings_slice);
                        self.struct_slice = &self.struct_slice[aligned_length..];
                        return Some(property_name);
                    } else {
                        // Skip over the property name offset and data.
                        self.struct_slice = &self.struct_slice[mem::size_of::<u32>()..];
                        self.struct_slice = &self.struct_slice[aligned_length..];
                    }
                }

                FDT_NOP => {
                    // Nothing to be done for NOPs.
                }

                FDT_END => {
                    // This marks the end of the device tree.
                    self.struct_slice = &[];
                }

                _ => {
                    panic!("EnumPropertiesIter encountered an invalid token {:#010X} {} bytes before the end", token, self.struct_slice.len());
                }
            }
        }

        None
    }
}

#[repr(C)]
struct DtbReserveEntry {
    address: u64,
    size: u64,
}

