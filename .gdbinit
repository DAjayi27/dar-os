# 1. Force GDB to read your DWARF debug symbols
file target/riscv32imc-unknown-none-elf/debug/rusty-blinky

# 2. Flash the code to the chip (via probe-rs)
load

# 3. Reset the chip and halt it
monitor reset halt

# 4. Set a hardware breakpoint at your Rust entry point
break main

# 5. Tell the chip to run until it hits main
continue