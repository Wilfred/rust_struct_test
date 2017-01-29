# Rust FFI Segfault on 32-bit Linux

On 32-bit linux, this seems to pass the wrong value:

```
$ uname -a
Linux a-friendly-32-bit-box 4.4.0-57-generic #78-Ubuntu SMP Fri Dec 9 23:46:51 UTC 2016 i686 i686 i686 GNU/Linux
$ ./build.sh
$ ./example
call fcdr with 0
fcdr: -1077451660
Segmentation fault (core dumped)
```

On 64-bit linux, this passes 0 as expected:

```
$ uname -a
Linux a-friendly-64-bit-box 4.8.13-1-ARCH #1 SMP PREEMPT Fri Dec 9 07:24:34 CET 2016 x86_64 GNU/Linux
$ ./build.sh
$ ./example
call fcdr with 0
fcdr: 0
```

