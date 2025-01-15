# rCli

first commit 

- [] cargo fmt 自动删除未使用的变量
- [] 尝试使用自己的想法重构代码、文件目录结构等

# 注意
- 命令行中使用 std::io::stdin().read_to_end() 的时候，会一直读取，直到结束, 而 windows 下结束的标志是 ctrl + z, 然后 enter 才可以

```makefile
cargo make base64 encode > temp.b64
```
- 在windows环境下，执行 ”> temp.b64“ 操作，生成的temp.b64 是utr-16编码的，无法直接通过 File::open().read_to_string() 打开，需要在Ubuntu下执行