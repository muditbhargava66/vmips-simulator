# Sample MIPS program - input.s
.text
main:
    # Load address of data section into $t9
    la $t9, data_start
    
    # Load values from memory
    lw $t0, 0($t9)     # Load value1
    lw $t1, 4($t9)     # Load value2
    
    # Add them
    add $t2, $t0, $t1
    
    # Store result
    sw $t2, 8($t9)     # Store in result
    
    # Exit program
    li $v0, 10
    syscall

.data
data_start:
value1: .word 42
value2: .word 58
result: .word 0