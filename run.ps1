param (
    [string]$image_name
)

if ($image_name -eq $null) {
    Write-Host "Usage: ./render.ps1 [image_name]"
    exit 1
}

$rendersDir = ".\renders"
New-Item -ItemType Directory -Path $rendersDir -Force | Out-Null

$env:RUSTFLAGS = "--allow dead_code"
cargo run --manifest-path=.\tracer\Cargo.toml
Copy-Item -Path ".\output.ppm" -Destination "$rendersDir\$image_name.ppm" -Force
