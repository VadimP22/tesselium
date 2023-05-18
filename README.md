# tesselium
Simple game engine for old-school roguelike games.


### How to build?
1. Ensure you have `raylib.h` in `/usr/include` and `libraylib.so` in `/usr/lib`
	- You can install it using your package manager. For example `sudo pacman -S raylib` (if you use pacman)
2. `cargo run` to run example app.


### Plan
- [ ] Safe raylib bindings
- [ ] Tesselium core api (to hide all raylib-related logic)
- [ ] ECS (using one of existing crates)
