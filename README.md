# Illumination-Theory
Ray tracer in Rust.

Following this: https://raytracing.github.io/books/RayTracingInOneWeekend.html

## Running
Clone this repo:
```bash
git clone https://github.com/MaximusPrimeForever/Illumination-Theory.git
```

### On Linux
Run the script:
```bash
cd Illumination-Theory
chmod +x ./render.sh
./render.sh image
```
This will render the latest image.  

### On Windows
Run the script:
```powershell
cd Illumination-Theory
.\run.ps1 image
```

## Images
You can checkout a tag to get a specific image.  
e.g. this will render an early image with anti aliasing implemented:
```bash
git checkout antialiased_world
./render.sh aa_world
```