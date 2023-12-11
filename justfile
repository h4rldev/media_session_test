set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set positional-arguments

@default:
    just --list

@build:
    cross build --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/debug/media_session_test.exe .

@build-release:
    cross build --release --target x86_64-pc-windows-gnu
    cp target/x86_64-pc-windows-gnu/release/media_session_test.exe .

@build-windows:
    cargo build --target x86_64-pc-windows-gnu
    cp target\\x86_64-pc-windows-gnu\\debug\\media_session_test.exe .

@build-windows-release:
    cargo build --release --target x86_64-pc-windows-gnu
    cp target\\x86_64-pc-windows-gnu\\release\\media_session_test.exe .

@test:
    cargo test --target x86_64-pc-windows-gnu

@run-release args="":
    cross run --release --target x86_64-pc-windows-gnu -- $1

@run args="":
    echo $1
    cross run --target x86_64-pc-windows-gnu -- $1

@run-windows-release args="":
    cargo run --release --target x86_64-pc-windows-gnu -- $1

@run-windows args="":
    cargo run --target x86_64-pc-windows-gnu -- $1

@create-tag args="v0.2.3":
    git tag $1
    git push master $1

@fix-tag args="v0.2.3":
    git tag -d $1
    git tag $1
    git push master :$1
    git push master $1