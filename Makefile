# Landlock Security Testing Makefile


# 変数定義
IMAGE_NAME = landlock_test
IMAGE_TAG = latest
DOCKER_BUILD = docker build
DOCKER_RUN = docker run

DOCKER_CAP_ADD = --cap-add=CAP_SYS_ADMIN
CONTAINER_NAME = landlock_test

# デフォルトのターゲット

.PHONY: all
all: build run

# Dockerイメージのビルド
.PHONY: build
build:
	@echo "Building Docker image..."
	$(DOCKER_BUILD) -t $(IMAGE_NAME) .

# Dockerコンテナの実行
.PHONY: run
run:
	@echo "Running Docker container..."
	$(DOCKER_RUN) $(DOCKER_CAP_ADD) --name $(CONTAINER_NAME) $(IMAGE_NAME):$(IMAGE_TAG)

stop:
	@echo "Stopping Docker container..."
	docker stop $(CONTAINER_NAME)

remove:
	@echo "Removing Docker container..."
	make stop
	docker rm $(CONTAINER_NAME)

# イメージのビルドと実行を連続して行う
.PHONY: build-and-run
build-and-run: build run

# クリーンアップ（イメージの削除）
.PHONY: clean

clean:
	@echo "Removing Docker image..."
	docker rmi $(IMAGE_NAME)

# ヘルプメッセージ
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  build         - Build the Docker image"
	@echo "  run           - Run the Docker container"
	@echo "  build-and-run - Build the image and run the container"
	@echo "  clean         - Remove the Docker image"
	@echo "  help          - Display this help message"
