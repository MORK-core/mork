#!/bin/bash

# 检查是否提供了文件名参数
if [ $# -ne 1 ]; then
    echo "Usage: $0 <filename>"
    exit 2  # 参数错误，退出码 2
fi

filename=$1

# 检查文件是否存在
if [ ! -f "$filename" ]; then
    echo "Error: File '$filename' not found!"
    exit 3  # 文件不存在，退出码 3
fi

# 获取文件的最后一行
last_line=$(tail -n 1 "$filename")

# 检查是否包含目标字符串
if [[ "$last_line" == *"all tests passed"* ]]; then
    echo "Success: All tests passed!"
    exit 0  # 成功，退出码 0
else
    echo "Error: Tests did not pass! Last line was: '$last_line'"
    exit 1  # 失败，退出码 1
fi