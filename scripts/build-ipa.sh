# 构建未签名的IPA

cd "$( cd "$( dirname "$0"  )" && pwd  )/.."


touch native/src/bridge_generated.rs
flutter_rust_bridge_codegen \
    --rust-input native/src/api.rs \
    --dart-output lib/bridge_generated.dart \
    --c-output ios/Runner/bridge_generated.h \
    --rust-crate-dir native \
    --llvm-path $LLVM_HOME \
    --class-name Native

cargo build --manifest-path native/Cargo.toml --release --features= --lib  --target=aarch64-apple-ios

cp native/target/aarch64-apple-ios/release/libnative.a ios/Runner/

flutter build ios --release --no-codesign

cd build
mkdir -p Payload
mv ios/iphoneos/Runner.app Payload

sh ../scripts/thin-payload.sh
zip -9 nosign.ipa -r Payload
