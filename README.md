Must copy bootstrap binary to netlify functions default directory.

```
cp target/lambda/intro-to-netlify-serverless-with-rust/bootstrap netlify/functions/hello
```

warning: Netlify can not identify the binary if the binary is built in --release
mode. This is a bug in Netlify's binary detection wasm package.
