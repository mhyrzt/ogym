

```bash

// download cmake source code
cd mujoco-x.x.x
mkdir build
cd build
cmake ..
make

mkdir ~/.local/mujoco/
cp -r ./lib ../include/ ~/.local/mujoco/

```
