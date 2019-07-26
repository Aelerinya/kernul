/* Constants for the Multiboot header */
    .set ALIGN,         1<<0
    .set MEMINFO,       1<<1
    .set FLAGS,         ALIGN | MEMINFO
    .set MAGIC,         0x1BADB002
    .set CHECKSUM,      -(MAGIC + FLAGS)

/* Multiboot header */
.section .multiboot
    .align 4
    .long MAGIC
    .long FLAGS
    .long CHECKSUM

/* 16KiB Stack */
.section .bss
    .align 16
stack_end:
    .skip 16384
stack_start:

/* Start of the kernel */
.section .text
    .global _start
    .type _start, @function
_start:
    /* Set the stack pointer to the top of the stack */
    mov $stack_start, %esp
    /* Call the main */
    call kernel_main
    /* Loop if the kernel returns */
    cli
1:  hlt
    jmp 1b
