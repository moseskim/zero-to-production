use sqlx::postgres::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    // 보안 상의 이유로 명시적으로 localhost, 127.0.0.1로 바인딩한다. 이 바인딩 설정은
    // 일부 환경에서는 문제를 일으킬 수 있다. 예를 들어, WLS2를 실행하는 곳에서는 연결성
    // 이슈가 있을 수 있다. 이 환경에서는 WSL2의 localhost 인터페이스에 바인딩을 해서는
    // 서버에 접근할 수 없다. 이에 관한 회피 방법으로 대신 모든 인터페이스(0,0,0,0)을
    // 바인딩 할 수 있지만, 서버를 모든 인터페이스에 노출시키는 것으로 인한 보안 이슈는
    // 주지해야 한다.
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
