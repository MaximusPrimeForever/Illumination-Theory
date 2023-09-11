param (
    [string]$image_name,
    [uint]$image_width = 400,
    [uint]$samples_per_pixel = 100,
    [uint]$trace_depth = 10,
    [uint]$thread_count = 0
)

if ($image_name -eq $null) {
    Write-Host "Usage: ./render.ps1 [image_name] [opt: image_width] [opt: samples_per_pixel] [opt: trace_depth] [opt: thread_count]"
    exit 1
}

$rendersDir = ".\renders"
New-Item -ItemType Directory -Path $rendersDir -Force | Out-Null

$env:RUSTFLAGS = "--allow dead_code"
cargo run --release --manifest-path=.\tracer\Cargo.toml -- $image_width $samples_per_pixel $trace_depth $thread_count
Copy-Item -Path ".\output.ppm" -Destination "$rendersDir\$image_name.ppm" -Force
