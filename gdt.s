.section .data

# Structure describing the address and size of the GDT
GDT_description:
# Table limit
  .short 0
# Table offset (address)
  .int 0

.section .text
  .global load_GDT
  .type load_GDT, @function

# void load_GDT(int offset, short limit);
load_GDT:
  # Move the base of the table to GDT_description.offset
  mov 0x4(%esp), %eax
  mov %eax, (0x2 + GDT_description)
  # Move the size of the table to GDT_description.limit
  mov 0x8(%esp), %eax
  sub $0x1, %eax
  mov %ax, (GDT_description)
  # Load the GDT table
  lgdt (GDT_description)
  # Update the code selector by far-jumping
  jmpl $0x8,$reload_cs
reload_cs:
# Update the data selectors
  mov $0x10, %ax
  mov %ax, %ds
  mov %ax, %es
  mov %ax, %fs
  mov %ax, %gs
  mov %ax, %ss
  ret
