# Bevy Examples Program

## Usage

```
cargo run --example ["Project name"]
```

・使用環境によっては以下のコードはコメントアウトしてください

```rs
    .insert_resource(WgpuSettings {
            backends: Some(Backends::VULKAN),
            ..Default::default()
        })
```