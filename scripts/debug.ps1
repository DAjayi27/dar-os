# 1. Configuration - Update these to match your project name
$projectName = "blog_os" # Change this to your crate name
$binaryPath = "target/x86_64-$projectName/debug/$projectName"
$imagePath = "target/x86_64-$projectName/debug/bootimage-$projectName.bin"

## 2. Build the project
#Write-Host "--- Building Kernel ---" -ForegroundColor Cyan
#cargo bootimage
#
#if ($LASTEXITCODE -ne 0) {
#    Write-Host "Build failed!" -ForegroundColor Red
#    exit
#}
#
## 3. Launch QEMU in the background
#Write-Host "--- Launching QEMU (Waiting for Debugger) ---" -ForegroundColor Cyan
#$qemuProcess = Start-Process qemu-system-x86_64 -ArgumentList "-drive format=raw,file=$imagePath", "-s", "-S" -PassThru

# 4. Run GDB with the fixes
Write-Host "--- Starting GDB ---" -ForegroundColor Green
# We use -ex to pass the 'osabi none' fix directly to GDB
gdb.exe $binaryPath `
    -ex "set osabi none" `
    -ex "set architecture i386:x86-64" `
    -ex "target remote :1234" `
    -ex "break _start" `
    -ex "continue"

# 5. Cleanup: Kill QEMU when GDB is closed
Write-Host "--- Closing QEMU ---" -ForegroundColor Yellow
Stop-Process -Id $qemuProcess.Id -ErrorAction SilentlyContinue