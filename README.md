生产模式下启动命令：
cargo run

开发模式下启动命令（更新代码后，自动重新加载新代码）：
systemfd --no-pid -s http::3003 -- cargo watch -x run