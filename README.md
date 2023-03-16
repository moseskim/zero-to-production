# Zero To Production In Rust

<div align="center"><a href="https://zero2prod.com" target="_blank"><img src="https://www.zero2prod.com/assets/img/zero2prod_banner.webp" /></a></div>

[Zero To Production In Rust](https://zero2prod.com)는 Rust를 사용한 백엔드 개발에 관한 주관적인 소개이다.

이 저장소는 [이 책](https://zero2prod.com/)의 보충 자료로 제공된다. 책 전체에서 설명하는 이메일 뉴스레터 프로젝트의 코드 베이스 스냅샷을 담고 있다.

## 챕터별 스냅샷

[`main`](https://github.com/moseskim/zero-to-production) 브랜치는 책 마지막의 프로젝트를 나타낸다.

각 장 마지막의 프로젝트는 개별 브랜치를 전환해서 확인할 수 있다:

- [3장, Part 0](https://github.com/moseskim/zero-to-production/tree/root-chapter-03-part0)
- [3장, Part 1](https://github.com/moseskim/zero-to-production/tree/root-chapter-03-part1)
- [4장](https://github.com/moseskim/zero-to-production/tree/root-chapter-04)
- [5장](https://github.com/moseskim/zero-to-production/tree/root-chapter-05)
- [6장, Part 0](https://github.com/moseskim/zero-to-production/tree/root-chapter-06-part0)
- [6장, Part 1](https://github.com/moseskim/zero-to-production/tree/root-chapter-06-part1)
- [7장, Part 0](https://github.com/moseskim/zero-to-production/tree/root-chapter-07-part0)
- [7장, Part 1](https://github.com/moseskim/zero-to-production/tree/root-chapter-07-part1)
- [7장, Part 2](https://github.com/moseskim/zero-to-production/tree/root-chapter-07-part2)
- [8장](https://github.com/moseskim/zero-to-production/tree/root-chapter-08)
- [9장](https://github.com/moseskim/zero-to-production/tree/root-chapter-09)
- [10장, Part 0](https://github.com/moseskim/zero-to-production/tree/root-chapter-10-part0)
- [10장, Part 1](https://github.com/moseskim/zero-to-production/tree/root-chapter-10-part1)
- [10장, Part 2](https://github.com/moseskim/zero-to-production/tree/root-chapter-10-part2)
- [10장, Part 3](https://github.com/moseskim/zero-to-production/tree/root-chapter-10-part3)
- [11장](https://github.com/moseskim/zero-to-production/tree/root-chapter-11)

## 사전 준비

다음을 설치해야 한다:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

다음은 OS별 요구 사항이다.

### Windows
  
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### Linux

```bash
# Ubuntu 
sudo apt-get install lld clang
# Arch 
sudo pacman -S lld clang
```

### MacOS

```bash
brew install michaeleisel/zld/zld
```

## 빌드 방법

`cargo`를 기동한다:

```bash
cargo build
```

## 테스트 방법

`cargo`를 기동한다:

```bash
cargo test 
```
