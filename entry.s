.text

.global _entry
_entry:
    la      sp, _stack_top
    jal     reset_handler

.global abort
abort:
    j       abort

.org 0x10
.global _irq_entry
_irq_entry:
    j       irq_handler
