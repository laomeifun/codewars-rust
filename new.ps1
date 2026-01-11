# 快捷创建 Codewars kata 文件
# 用法: .\new.ps1 <kata_url_or_id>
# 例如: .\new.ps1 50654ddff44f800200000004
#       .\new.ps1 https://www.codewars.com/kata/50654ddff44f800200000004

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$KataId
)

Set-Location $PSScriptRoot
cargo run --bin new_kata -- $KataId
