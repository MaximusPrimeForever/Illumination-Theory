param (
    [string]$image_name,
    [bool]$run_test = $false,
    [uint]$image_width = 400,
    [float]$aspect_ratio = 1.7777,
    [float]$vertical_fov = 60,
    [uint]$samples_per_pixel = 100,
    [uint]$trace_depth = 10,
    [uint]$thread_count = 0
)

if ($run_test -eq $true) {
    $env:RUSTFLAGS = "--allow dead_code"
    cargo run --release --manifest-path=.\tracer\Cargo.toml
    exit 0
}

if ($image_name -eq $null) {
    Write-Host "Usage: ./render.ps1 [image_name] [opt: image_width] [opt: aspect_ratio] [opt: vertical_fov] [opt: samples_per_pixel] [opt: trace_depth] [opt: thread_count]"
    exit 1
}

$rendersDir = ".\renders"
New-Item -ItemType Directory -Path $rendersDir -Force | Out-Null

$env:RUSTFLAGS = "--allow dead_code"
cargo run --release --manifest-path=.\tracer\Cargo.toml -- $image_width $aspect_ratio $vertical_fov $samples_per_pixel $trace_depth $thread_count
Copy-Item -Path ".\output.png" -Destination "$rendersDir\$image_name.png" -Force
