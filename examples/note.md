
```shell
gcc statvfs-simple.c -o statvfs-simple
gcc statfs-simple.c -o statfs-simple

```

```text
#0: statvfs.f_bsize 在 linux 和 macos 上的值不一致
linux 上 statvfs 的 f_bsize = 4096 = 4K，f_frsize = 4096
macos 上 statvfs 的 f_bsize = 1048576 = 1M，f_frsize = 4096

这很令人困惑，因为在 Linux 手册页中，是这样说的：
无符号长 f_bsize; /* 文件系统块大小 */
unsigned long f_frsize; /* 片段大小 */

似乎 f_bsize 是我需要的逻辑块大小，而在 macOS 的手册页中，它说：
f_frsize 此文件系统上最小分配单元的大小（以字节为单位）。（这对应于 struct statfs 的 f_bsize 成员。）
f_bsize 此文件系统上文件的 I/O 请求的首选长度。（对应于 struct statfs 的 f_iosize 成员。）

好像 f_frsize 是逻辑块大小，最后我查了 POSIX 标准，这里是规范：
unsigned long f_bsize 文件系统块大小。
unsigned long f_frsize 基本文件系统块大小。

MacOS 文档说的是 statvfs.f_frsize 与 statfs.f_bsize 具有相同的含义，并且 statvfs.f_bsize 同样等效于 statfs.iosize。

结论：在 linux 上使用 f_bsize，在 macos 上使用 f_frsize。


#1 文心一言: 为什么 MacOS 系统上，statvfs 的 f_bsize 等于 1048576，但实际使用的值却是 4096。
首先，statvfs 的 f_bsize 字段表示的是文件系统的基本块大小，它并不直接对应于文件系统的实际块大小。

f_bsize 的值为 1048576 可能是因为：

你正在使用的是一个特殊或自定义的文件系统，其块大小被设置为 1048576 字节。
statvfs 的实现或返回值可能存在某种错误或异常。
你可能正在查看的是一个虚拟文件系统（例如 /proc 或其他特殊文件系统），而不是实际的磁盘文件系统。
实际上，对于大多数常见的文件系统（如 HFS+、APFS、ext4 等），块大小通常是 4096 字节，这是为了优化磁盘性能和空间利用率。

如果你想要获取特定文件或目录的实际块大小，你可以使用 stat 函数来获取 st_blksize 字段的值，这个字段表示了文件或目录的块大小。

如果你想要知道实际使用的文件系统的块大小，你可能需要检查具体的文件系统类型和配置。


#2 GPT: 为什么 MacOS 系统上，statvfs 的 f_bsize 等于 1048576，但实际使用的值却是 4096。
在MacOS系统上，`statvfs` 结构中的 `f_bsize` 字段表示文件系统块的大小（block size）。这个值通常是文件系统的基本块大小，以字节为单位。在您提到的情况下，`f_bsize` 的值为 1048576 字节，但实际使用的值却是 4096 字节。

这可能是由于文件系统使用了一种称为"分页大小"（page size）的概念。分页大小是操作系统用于管理内存和文件系统的基本单位，通常是4 KB（4096 字节）。因此，即使文件系统的块大小（`f_bsize`）较大，实际上在磁盘上的文件和目录会按照分页大小进行分配和存储。

这样的设计可以提高文件系统的性能和效率。虽然文件系统支持较大的块大小，但在实际存储和管理文件时，仍然按照较小的分页大小进行操作。

请注意，文件系统的具体实现可能会有所不同，因此这只是一种可能的解释。如果您有特定的文件系统和版本信息，可能可以提供更详细的解释。
```

[statvfs(3) — Linux manual page](https://man7.org/linux/man-pages/man3/statvfs.3.html)
[statfs(2) — Linux manual page](https://man7.org/linux/man-pages/man2/statfs.2.html)
[stat(2) — Linux manual page](https://man7.org/linux/man-pages/man2/stat.2.html)

[struct statvfs 中的 f_bsize 和 f_frsize 代表什么？](https://qa.1r1g.com/sf/ask/3837647901/)
[在statvfs中，f_bsize和f_frsize代表什么？](https://cloud.tencent.com/developer/ask/sof/115717838)

[Linux调用stat获取文件分区系统信息](http://www.freeoa.net/osuport/storagebak/linux-fsnpstat_3795.html)
[statfs系统调用](https://www.zhihu.com/column/p/32162982)
[python 查看硬盘空间](https://juejin.cn/s/python%20%E6%9F%A5%E7%9C%8B%E7%A1%AC%E7%9B%98%E7%A9%BA%E9%97%B4)
[linux获取磁盘剩余空间函数](https://juejin.cn/s/linux%E8%8E%B7%E5%8F%96%E7%A3%81%E7%9B%98%E5%89%A9%E4%BD%99%E7%A9%BA%E9%97%B4%E5%87%BD%E6%95%B0)
[Linux statvfs()获取系统磁盘信息](https://www.cnblogs.com/fortunely/p/17212612.html)

[getattrlist -- get file system attributes](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/getattrlist.2.html)
[getattrlistbulk(2) [mojave man page]](https://www.unix.com/man-page/mojave/2/getattrlistbulk/)
