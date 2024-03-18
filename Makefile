.PHONY: all

arch :=aarch64
tgt_sdk_version :=34

ifndef ANDROID_NDK_ROOT
		echo ANDROID_NDK_ROOT is not defined
		exit 1
endif
export ANDROID_NDK_ROOT := $(realpath ${ANDROID_NDK_ROOT})

ifeq ("$(wildcard $(ANDROID_NDK_ROOT)/bin/clang)","")	
ANDROID_NDK_ROOT := ${ANDROID_NDK_ROOT}/toolchains/llvm/prebuilt/linux-x86_64/
endif

all: Cargo.toml
	export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_NDK_ROOT}/bin/${arch}-linux-android${tgt_sdk_version}-clang"  && \
	export AR_aarch64_linux_android="${ANDROID_NDK_ROOT}/bin/llvm-ar" && \
	export CC_aarch64_linux_android="${ANDROID_NDK_ROOT}/bin/${arch}-linux-android${tgt_sdk_version}-clang" && \
	export CXX_aarch64_linux_android="${ANDROID_NDK_ROOT}/bin/${arch}-linux-android${tgt_sdk_version}-clang++" && \
	cargo build --target aarch64-linux-android
