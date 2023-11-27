import os
import struct

# 需要处理的 ELF 文件列表
elf_files = ["hello_app.elf"]  # 将文件名替换为实际的 ELF 文件

# 获取每个 ELF 文件生成的二进制文件的大小
binary_sizes = []
for elf_file in elf_files:
    # 获取文件大小
    binary_size = os.path.getsize("hello_app.bin")  # 假设二进制文件与 ELF 文件同名，但扩展名不同
    binary_sizes.append(binary_size)

binary_size_data = binary_size.to_bytes(8, byteorder='little') 
# 写入 app.bin 文件头部
with open("./arceos/payload/apps.bin", "r+b") as app_bin_file:
    # 将二进制文件大小信息打包为二进制数据
    # binary_size_data = struct.pack("I" * len(binary_sizes), *binary_sizes)
    
    # 将二进制文件大小信息写入 app.bin 的开头
    # 指定40为获取文件大小的地方
    app_bin_file.seek(40)
    app_bin_file.write(binary_size_data)
