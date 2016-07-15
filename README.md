rsay
===

A rusty fork of cowsay

Usage
---

Install via `cargo install rsay` (executables coming soon)

```shell
Usage: rsay [-OPTIONS] [ARG...]

Options:
    -h, --help          Print this help menu
    -W, --width 50      Width of output
```

:metal: Benchmarks :metal:                                                                         
---                                                                                                
                                                                                                   
Rsay turns a blind eye to the wonderful safety and concurrency features that rust offers and instead focuses on
blazing fast ascii performance:                                                                         
                                                                                                   
```                                                                                                
time cowsay foo  0.02s user 0.02s system 68% cpu 0.052 total                                       
time rsay foo  0.00s user 0.00s system 35% cpu 0.012 total                                         
 -----------------------------------------
< wow, think of how much time you'll save >
 -----------------------------------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
```                                                                                                

Todo
---

- options
  - user specified art
  - randomization
- multi os executables
