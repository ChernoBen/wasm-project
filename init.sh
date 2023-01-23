#!/bin/bash
wasm-pack build --target bundler
cd pkg && npm link && cd ..
cd www && npx webpack build