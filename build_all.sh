set -e

cd raylib; cargo build; cd ..

for i in 3.3 4.2 5.2 6.1 6.2 6.7; do
  echo "=============================================";
  echo $i;
  cd $i; cargo build;
  rm example.ppm; ./target/debug/raytracer; cd ..
done

# to slow to actually run the raytracer as a group
for i in 7.2 8.2 8.3; do
  echo "=============================================";
  echo $i;
  cd $i; cargo build; cd ..
done
