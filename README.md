Pagemap Analyzer
===

![logo](http://lxr.linux.no/.static/gfx/lxray-large.png)

This is a tool for gathering statistics from the pagemap Linux
kernel interface. It reads the following files:

* /proc/kpagecount
* /proc/kpagecgroup
* /proc/kpageflags

And prints sorted amounts of memory occupied by pages of each
combination of values of these files.

This tool is useful for finding out which subsystem uses too
much memory.

## Example output

```
user@notebook:~/Projects/pagemap_analyzer$ cargo run --release
    Finished release [optimized] target(s) in 0.02s
     Running `target/release/pagemap_analyzer`
    1.    5.60 GiB - times_mapped =    1    ; cgroup_inode =   3203  ; flags = UPTODATE | LRU | ACTIVE | MMAP | ANON | SWAPBACKED
    2.    3.37 GiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = BUDDY
    3. 1024.00 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = NOPAGE
    4.  638.67 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = ARCH
    5.  570.40 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = (empty)
    6.  521.93 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = REFERENCED | UPTODATE | LRU | MAPPEDTODISK
    7.  412.69 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = REFERENCED | UPTODATE | LRU | ACTIVE | MAPPEDTODISK
    8.  266.67 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = UPTODATE | LRU | MAPPEDTODISK
    9.  244.03 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = RESERVED | PRIVATE
   10.  241.15 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = UPTODATE | LRU | PRIVATE
   11.  237.62 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = UPTODATE | LRU | ACTIVE | MAPPEDTODISK
   12.  230.58 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = SLAB | COMPOUND_TAIL
   13.  207.36 MiB - times_mapped =    0    ; cgroup_inode =   1475  ; flags = UPTODATE | LRU | MAPPEDTODISK
   14.  201.63 MiB - times_mapped =    0    ; cgroup_inode =    0    ; flags = RESERVED
   15.  194.32 MiB - times_mapped =    0    ; cgroup_inode =   3203  ; flags = LRU | ACTIVE | PRIVATE
Total:   15.25 GiB
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.
