use log::*;
use redis::{Connection, FromRedisValue, RedisResult};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn create_redis_client(connection_string: &str) -> redis::Client {
    let client = redis::Client::open(connection_string).unwrap();
    client
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    use redis::Commands;
    use testcontainers::clients::Cli;
    use testcontainers_modules::{postgres::Postgres, redis::Redis, testcontainers::RunnableImage};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_create_redis_connection() {
        env_logger::init();
        let docker = Cli::default();
        let redis_password = "21345";
        let redis_cluster = docker.run(create_redis(&redis_password));
        let connection_string: String = format!(
            "redis://:{}@{}/0",
            &redis_password,
            redis_cluster.get_host_port_ipv4(6379)
        );

        info!("Connection: {}", connection_string);

        let redis_client = create_redis_client(&connection_string);
        let mut con = redis_client.get_connection().unwrap();

        let _: () = con.set("my_key", 42).unwrap();
        // please explain _: ()
    }

    /// Create a Redis module with `6.2-alpine` tag and custom password
    fn create_redis(password: &str) -> RunnableImage<Redis> {
        RunnableImage::from(Redis::default())
            .with_tag("6.2-alpine")
            .with_env_var(("REDIS_PASSWORD", password))
    }
}
