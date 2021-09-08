for i in 3.3 4.2 5.2 6.1 6.2; do
  echo "============================================="
  echo $i
  cd $i; cargo build; rm example.ppm; ./target/debug/raytracer; cd ..
done
