#!/bin/bash
wasm-pack build --target web 
cd www
npm run dev