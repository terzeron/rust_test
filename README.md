# Cargo를 이용한 프로젝트 관리
* 새로운 프로젝트 생성
    * `cargo new hello_cargo --bin`
    * `cd hello_cargo`
* 빌드
    * `cargo build`
    * `./target/debug/hello_cargo`
    * 빌드와 실행을 한꺼번에
        * `cargo run`
    * 릴리즈 빌드
        * `cargo build --release`
* 실행이 잘 안 될 때
    * IntelliJ IDEA에서 Cargo.toml 파일을 열어서 attach하면 fn main에 대해 run 가능해짐
    
