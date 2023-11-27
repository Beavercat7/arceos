import os
import struct


# 需要处理的 ELF 文件列表
elf_files = ["hello_app.elf"]  # 将文件名替换为实际的 ELF 文件

# 获取每个 ELF 文件生成的二进制文件的大小
binary_sizes = []

binary_sizes1 = []
for elf_file in elf_files:
    # 获取文件大小
    binary_size = os.path.getsize("hello_app.bin")  # 假设二进制文件与 ELF 文件同名，但扩展名不同
    binary_sizes.append(binary_size)   
    # 获取文件大小
    binary_size1 = os.path.getsize("app1.bin")  # 假设二进制文件与 ELF 文件同名，但扩展名不同
    binary_sizes1.append(binary_size1)

binary_size_data = binary_size.to_bytes(8, byteorder='little') 
binary_size_data1 = binary_size1.to_bytes(8, byteorder='little') 

def replace_bytes(source_file, dest_file, num_bytes):
    # 读取源文件和目标文件的内容
    with open(source_file, 'rb') as source:
        source_data = source.read(num_bytes)

    with open(dest_file, 'r+b') as dest:
        # 移动文件指针到需要替换的位置
        dest.seek(10)
        # 读取目标文件的内容
        dest_data = dest.read(num_bytes)
        # 确保源文件的读取字节数与目标文件读取字节数相同
        source_data = source_data.ljust(len(dest_data), b'\0')
        # 移动文件指针回到替换的位置
        dest.seek(10)
        # 替换目标文件的内容
        dest.write(source_data)

# 调用函数来替换文件内容
replace_bytes('./arceos/payload/apps1.bin', './arceos/payload/apps.bin', 10)  # 将 source.bin 文件的前 100 字节覆盖 dest.bin 文件的前 100 字节



# 写入 app.bin 文件头部
with open("./arceos/payload/apps.bin", "r+b") as app_bin_file:
    # 将二进制文件大小信息打包为二进制数据
    # binary_size_data = struct.pack("I" * len(binary_sizes), *binary_sizes)
    
    # 将二进制文件大小信息写入 app.bin 的开头
    app_bin_file.seek(40)
    app_bin_file.write(binary_size_data)
    app_bin_file.seek(42)
    app_bin_file.write(binary_size_data1)
