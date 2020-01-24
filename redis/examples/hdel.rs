use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

fn del_hashmap_key(key: String, field: String) -> RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hdel(key, field)
}

fn main() {
    del_hashmap_key("michael".to_string(), "57".to_string());
}
