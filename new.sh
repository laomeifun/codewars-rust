#!/bin/bash
# 快捷创建 Codewars kata 文件
# 用法: ./new.sh <kata_url_or_id>
# 例如: ./new.sh 50654ddff44f800200000004
#       ./new.sh https://www.codewars.com/kata/50654ddff44f800200000004

if [ -z "$1" ]; then
    echo "用法: ./new.sh <kata_url_or_id>"
    exit 1
fi

cd "$(dirname "$0")"
cargo run --bin new_kata -- "$1"
