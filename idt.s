.section .data

# Structure describing the address and size of the IDT
IDT_description:
# Table limit
  .short 0
# Table offset (address)
  .int 0

.section .text
  .global load_IDT
  .type load_IDT, @function

  # void load_IDT(int offset, short limit);
load_IDT:
  # Move the base of the table to IDT_description.offset
  mov 0x4(%esp), %eax
  mov %eax, (0x2 + IDT_description)
  # Move the size of the table to IDT_description.limit
  mov 0x8(%esp), %eax
  sub $0x1, %eax
  mov %ax, (IDT_description)
  # Load the IDT table
  lidt (IDT_description)
  ret
