OUTPUT_DIR=./bin

all: build

build:
	@mkdir $(OUTPUT_DIR)
	rustc server.rs -o $(OUTPUT_DIR)/server
