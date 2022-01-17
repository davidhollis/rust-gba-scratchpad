# rust-gba-scratchpad
Playing around with targeting the gba from rust

## Milestones

My goal with this exercise is to eventually produce a barebones 2d platformer, and in doing so refresh my memory on some aspects of GBA programming and learn others for the first time. The milestones I've set for myself are:

- [x] `00`: button inputs
    - source: [`00-buttons`](00-buttons)
    - build artifact: `target/gba/buttons.gba`
- [x] `01`: horizontal running, collision with walls, in a bitmapped graphics mode
    - source: [`01-running`](01-running)
    - build artifact: `target/gba/running.gba`
- [ ] `02`: use vblank/vcount interrupts to schedule drawing and physics calculations, respectively
- [ ] `03`: add acceleration, jumping, and vertical collisions
- [ ] `04`: switch to a tiled graphics mode with a sprite for the player character
- [ ] `05`: add collisions with tiles
- [ ] `06`: add collisions with sprites
- [ ] `07`: expand the tilemap with a scolling camera
- [ ] `08`: add support for multiple rooms with loading zones
- [ ] `09`: add background music
- [ ] `10`: add sfx

### Getting binaries

I'll be creating github releases of each milestone as they're completed

### Building from source

If you'd like to build these examples from source, you'll need to install a specific version of nightly rust (this is due to a bug in the underlying gba library that'll be fixed in the next release):

```
rustup install nightly-2021-07-13
rustup +nightly-2021-07-13 component add rust-src
```

Then install the ARM binutils and `gbafix`:

```
# on OS X
brew install --cask gcc-arm-embedded
cargo install gbafix
```

Finally, to build all of the examples in this repo:

```
./build.sh
```

The build script will print out all of the roms it produced.