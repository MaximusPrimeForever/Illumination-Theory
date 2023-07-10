param (
    [string]$image_name,
    [uint]$image_width = 400,
    [uint]$samples_per_pixel = 100,
    [uint]$trace_depth = 10
)

if ($image_name -eq $null) {
    Write-Host "Usage: ./render.ps1 [image_name]"
    exit 1
}

$rendersDir = ".\renders"
New-Item -ItemType Directory -Path $rendersDir -Force | Out-Null

$env:RUSTFLAGS = "--allow dead_code"
cargo run --release --manifest-path=.\tracer\Cargo.toml -- $image_width $samples_per_pixel $trace_depth
Copy-Item -Path ".\output.ppm" -Destination "$rendersDir\$image_name.ppm" -Force
