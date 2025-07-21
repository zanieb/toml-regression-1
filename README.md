# toml 0.9 regression for duplicate key errors

```
--- toml 0.8.19 ---
TOML parse error at line 4, column 1
  |
4 | python-multipart = "https://example.com/bar.whl"
  | ^
duplicate key `python-multipart` in table `tool.uv.sources`


--- toml 0.9.2 ---
TOML parse error at line 4, column 1
  |
4 | python-multipart = "https://example.com/bar.whl"
  | ^^^^^^^^^^^^^^^^
duplicate key
```
