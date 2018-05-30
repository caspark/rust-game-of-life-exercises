## Development Setup

### Ubuntu

```
sudo apt install libsdl2-dev
```

### MacOS

Install SDL2 via HomeBrew:

    brew install sdl2

Then set up the header locations by adding `/usr/local/lib` to your path:

    echo 'export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"' >> ~/.bash_profile


## Developing

    cargo run

