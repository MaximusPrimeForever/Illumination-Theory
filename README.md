# Illumination-Theory
Ray tracer in Rust.

Following this: https://raytracing.github.io/books/RayTracingInOneWeekend.html

## Running
Clone this repo:
```bash
git clone https://github.com/MaximusPrimeForever/Illumination-Theory.git
```

Run the script:
```bash
cd Illumination-Theory
chmod +x ./render.sh
./render.sh image.ppm
```
This will render the latest image.  

## Images
You can checkout a tag to get a specific image.  
e.g. this will render an early image with anti aliasing implemented:
```bash
git checkout antialiased_world
./render.sh aa_world.ppm
```