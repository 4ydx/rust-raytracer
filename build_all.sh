set -e

cd raylib; cargo build; cd ..

for i in 03.3 04.2 05.2 06.1 06.2 06.7; do
  echo "=============================================";
  echo $i;
  cd $i; cargo build --release;
  rm example.ppm; ./target/debug/raytracer; cd ..
done

# to slow to actually run the raytracer as a group
for i in 07.2 08.2 08.3 08.4 08.5 08.6 09.5 09.6 10.2 10.3 10.4 10.5 11.1 11.2 12.2 13.1; do
  echo "=============================================";
  echo $i;
  cd $i; cargo build --release; cd ..
done
