# wasm-leet-sample
Example of a wasm that performs text to leet conversion.

![demo](./demo/wasm-leet-sample.gif)

The implementation of the leet conversion is based on [this repository](https://github.com/shinshin86/rust-leet).

## build

```sh
wasm-pack build --target web
```

then start local server.

## License
MIT & CC-BY-SA 3.0

In the `get_leet_list` function, character combinations taken from Wikipedia are used. For this reason, only this function is CC-BY-SA 3.0 Licence.