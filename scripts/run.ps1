# $PSScriptRoot is the directory containing this script (scripts/)
# We go up one level (..) to reach the project root where 'target' lives
$projectRoot = Split-Path -Parent $PSScriptRoot
$imagePath = Join-Path $projectRoot "target/x86_64_os/debug/bootimage-DOS.bin"

## Check if file exists before running to save yourself some headache
#if (Test-Path $imagePath) {
#    qemu-system-x86_64 -drive "format=raw,file=$imagePath" -serial stdio
#} else {
#    Write-Error "Could not find bootimage at $imagePath. Did you run 'cargo bootimage' first?"
#}

# Run normally and forward COM1 to this terminal.
# Use scripts/debug.ps1 when you want the VM to start paused for GDB.
cargo run -- -serial stdio
