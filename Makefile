TARGET = aarch64-unknown-none-softfloat

KERNEL_ELF = target/$(TARGET)/release/slingshot
KERNEL_BIN = target/$(TARGET)/release/kernel8.img

LINK_FILE = link.ld
TARGET_CPU = cortex-a72
RUSTC_ARGS = -C link-args=$(LINK_FILE) -C target-cpu=$(TARGET_CPU)
BUILD_ARGS = --release --target=$(TARGET) -- $(RUSTC_ARGS)

.PHONY: all clean copy $(KERNEL_BIN) $(KERNEL_ELF)

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	@cargo rustc $(BUILD_ARGS)

$(KERNEL_BIN): $(KERNEL_ELF)
	@rust-objcopy -O binary $(KERNEL_ELF) $(KERNEL_BIN)
	@echo "Created kernel binary"

copy: $(KERNEL_BIN)
	@./scripts/copy.sh	

clean:
	rm -rf target
