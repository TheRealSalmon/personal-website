# personal-website
This is my personal website, build with Rust+Leptos and deployed via Vercel.

I wanted to achieve a style similar to a business card. While the site is plain,
I think it functions well.

# Useful dev commands

## Local development
```bash
trunk serve
```

## Format with rustfmt and leptosfmt
```bash
cargo fmt && leptosfmt -t 2 ./**/*.rs
```
