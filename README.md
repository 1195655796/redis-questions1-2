# Simple-redis

## 支持命令

- echo
- hmget
- hset
- sadd
- smembers
- sismember

## 操作方式

Windows：
$env:RUST_LOG="info"; cargo run
Mac：
RUST_LOG=info cargo run
在第二个terminal输入：
redis-cli
127.0.0.1:6379> HSET myhash field1 "Hello"
(integer) 1
127.0.0.1:6379> HSET myhash field2 "World"
(integer) 1
127.0.0.1:6379> HMGET myhash field1 field2 nofield
1) "Hello"
2) "World"
3) (nil)
127.0.0.1:6379> SADD myset "Hello"
(integer) 1
127.0.0.1:6379> SADD myset "World"
(integer) 1
127.0.0.1:6379> SADD myset "World"
(integer) 0
127.0.0.1:6379> SMEMBERS myset
1) "Hello"
2) "World"
127.0.0.1:6379> SADD myset "one"
(integer) 1
127.0.0.1:6379> SISMEMBER myset "one"
(integer) 1
127.0.0.1:6379> SISMEMBER myset "two"
(integer) 0
127.0.0.1:6379> echo "hello world!"
"hello world!"