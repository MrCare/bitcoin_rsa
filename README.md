# Bitcoin-Style RSA 签名与工作量证明系统

## 项目简介

一个使用Rust实现的加密系统，模拟比特币核心加密机制，包括RSA签名和工作量证明(POW)算法。

## 技术特点

- 🔐 RSA密钥对生成
- 🔒 消息签名与验证
- ⛏️ 工作量证明(POW)算法
- 🔬 SHA-256哈希
- 🔑 Base64签名编码

## 技术栈

- 语言：Rust
- 加密库：`rsa`
- 哈希算法：SHA-256

## 快速开始

### 安装依赖
```bash
cargo build
```

### 运行项目
```bash
cargo run
```

### 输出结果
```
(base) ➜  bitcoin_rsa git:(master) ✗ cargo run
   Compiling bitcoin_rsa v0.1.0 (/Users/car/Work/2025beginAgain/bitcoin_rsa)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/bitcoin_rsa`
私钥(PEM):
Zeroizing("-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQC6khRuLSY/h1SA\nX3Lg0v9p0fh8aMNp89gWJmra2bIIFqiTIeBEFHpHVNWkWkKlRmoMFFksoZAbuM4K\nUW+sJ9tGWq/5OeD1hol84NgNeN2UfCR8Ig5acCKG7u+s/wUpx6iY0q4TbliBHoMI\nYCdVMJQgCSq8h7lZeJySP5zB5RHkOUmCBOqJrLDKJqy1k9QL8fT9+iS81Ycmx6M+\ngQMrl5UfL9i18naaNJzKf6OPfekdlHNhTpNVn0ro8IIbGFQYp9zom/5e97pa2D4f\nCCWWSj8rgEeV9NR62XdPtrvvPATn0Dt6b+m1Hr9llVKvn1JMXMHP6XPXjuW1tb2u\ncr3C32tnAgMBAAECggEBAKfnd4OAFn/t5tpxDA3RVhOB58Lu9Y6Q6NNmV5wFdvdY\njPV88zvRwgoYCv0SThA3wpCEb4x/ChYoxyT6DSo95FT4oI4GzLGqFmnQYCAfuio7\nXwpoC8rQSAJvXjuudpMXIGAImqjaEcSCFSKghkVgpq3+EfwBmYkvxpjHrufAo81j\nRQNM/ZLqGu7dnGWXpGMCOk/jlg+nGhGLOe4Q236LdEQ98TBycxK6kFXCJQgZnfKA\nGwtEsHKATCU8S9yRqNgVI17SQwrBOS1E9xOCCVGUKDc2r4r56n4PdIfOwrEFW5Fw\n7Sy5ePoVR/ADWzRUK456vY7o82LFdBp628ZlOGxNAQECgYEA9XKq4OxQXpxgcadM\n7nBX33kJUUBa56KENw3NAVewigP44l9lpW/6IVtIHtPE4IBoGAPkhX+/r5Sj5UJd\nFyQC4w7RzG2HgrplCqAdeG2WI2usFq4/aYIEtvoOskfK9BSCK6g/6V11uY0Rm+B4\nDOsYGEYycP/OB9wL8tn8TdoV8McCgYEAwpdsyNaRt8sWYEP+hjXikFzB2pue/RgT\nhg3CUuj3itI3TaPqeAHEr7EuVt75H+VJDJ1KIHxM9RVPBW4xU7oOm48TKKH9USwr\nKlyEoy14vZThaKWNxkKyd8PxFdxBj60ScoQMPvfq2Z6drGS6abTswlux91m4Zh+Z\n5X4DirEnUGECgYEA3jhmG2P409/StbOZxFr0RACqfeS9KPCLseiNzgRdcmdsQ0/J\nkIIUPXUOk9J5ciTvmaaUlIV3v/jcdmmiLJxO/NHjAqm7wZEOgBM8+1aEUydd098I\nzHaJ3DwDaEf1qQyUWgBndNpIBqmA4tbU+iUzWG7dpk8Dczw3mTTg6H9l0+sCgYAl\nrdCAqxuYLRjuFvxn3HM8ZNmcjNmiRFEzeeKQz09vb/kJbZ3vEtiU9If2hkJS7MDS\nUnqSA7+bDT852/ZtMe+2K2QYUBWO7DCHiVadiiaA/x0OeFAVfcC09Bnwk6+WNHsH\n9Y2xe6vb65y4TuDRqxjVIZA6RwMvsxrHlzj0WA02wQKBgF34Pw9tWm+jnlhR9gJr\nenD+AETbkn+LXQxCjKlSGdF6KFoOrXuy17aj2X3xTM0Jqf7V3lG4cRZj9OOM3FtL\nxETIi9wcT+00aAmCJeLyKqiZFZexULGvrMAq33Yv6MzklN07kQen6VMJxgW63i+W\nrgx6sBN+fQhCagoRtk81Xmz0\n-----END PRIVATE KEY-----\n")
公钥(PEM):
"-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAupIUbi0mP4dUgF9y4NL/\nadH4fGjDafPYFiZq2tmyCBaokyHgRBR6R1TVpFpCpUZqDBRZLKGQG7jOClFvrCfb\nRlqv+Tng9YaJfODYDXjdlHwkfCIOWnAihu7vrP8FKceomNKuE25YgR6DCGAnVTCU\nIAkqvIe5WXickj+cweUR5DlJggTqiaywyiastZPUC/H0/fokvNWHJsejPoEDK5eV\nHy/YtfJ2mjScyn+jj33pHZRzYU6TVZ9K6PCCGxhUGKfc6Jv+Xve6Wtg+Hwgllko/\nK4BHlfTUetl3T7a77zwE59A7em/ptR6/ZZVSr59STFzBz+lz147ltbW9rnK9wt9r\nZwIDAQAB\n-----END PUBLIC KEY-----\n"
找到nonce: 35330，耗时: 382.060117ms
POW结果: nonce = 35330, hash = 00007d6c920eb79043a61108f938dd4d81e036ec02ed4fbe50e1a626e194aa90
签名(base64): qJbFwQ8xXpgUhJ+5Xej2ITuRIWC/4ONAShx/67VnmlOVvKZIr7ce0Zih9PIknGGHPsnwY5Vw5n39qHJ++FmN18kiet7K7QPatEYJCrbsZQA7aGIzjiW4QPlFajYBxWE76KzoUkTm+Ad2yPgC6LWh6d/Yca2Nni8Gt2J9HnJDjhmlkO6ElKFXE4ofTV03TZg0ZMaWhqbEnO7BA+p8ib8aTHNBzjMruyLJN4j8RyOWYpAt87o3Sym7BdBT/4gh0ECKHU6jMafyk3EspULvXKGY/2P+Mmd84PSu5XA/jqoeaU5mBvtxuphA5g5ybG505xVFW6kJb6q25wp4NWLiFfpx5Q==
签名验证结果: true
```

## 核心功能

1. 密钥生成：安全生成RSA公私钥对
2. 消息签名：使用私钥签名
3. 签名验证：使用公钥验证签名
4. 工作量证明：找到满足难度的哈希值

## 代码示例

```rust
// 生成密钥对
let (public_key, private_key) = generate_rsa_keypair();

// 签名消息
let signature = sign_with_private_key(&private_key, message);

// 验证签名
let is_valid = verify_with_public_key(&public_key, message, &signature);
```

## 安全性

- RSA加密算法
- SHA-256哈希
- 安全随机数生成

## 贡献

欢迎提交Issues和Pull Requests改进项目。

## 作者

Mr.Car

## 免责声明

仅用于学习和研究目的。