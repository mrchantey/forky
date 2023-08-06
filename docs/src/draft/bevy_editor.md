# Bevy Editor

## Objectives

The first objective is to run games inside of the editor process. I'd like to try the shared memory technique first.

Steps:
1. mmap hello world - https://github.com/RazrFalcon/memmap2-rs
2. render edited image
3. render screenspace image
4. render mmaped image
5. signal to editor that image is ready