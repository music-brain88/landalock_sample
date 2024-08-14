# Rustの公式イメージをベースとして使用
FROM rust:1.80


# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y libcap2-bin


# 必要なファイルをコンテナにコピー
COPY Cargo.toml .
COPY src/main.rs ./src/
COPY test_landlock_script.sh .

# スクリプトに実行権限を付与
RUN chmod +x test_landlock_script.sh


# テスト用のディレクトリを作成し、権限を設定
RUN mkdir -p /home/user/writable && chmod 777 /home/user/writable


# Rustの依存関係をインストールしてビルド
RUN cargo build --release


# デバッグ用のシェルスクリプトを作成

RUN echo '#!/bin/bash' > /usr/local/bin/debug_and_run.sh && \
    echo 'set -x' >> /usr/local/bin/debug_and_run.sh && \
    echo 'id' >> /usr/local/bin/debug_and_run.sh && \
    echo 'ls -la /home/user/writable /tmp' >> /usr/local/bin/debug_and_run.sh && \
    echo 'capsh --print' >> /usr/local/bin/debug_and_run.sh && \
    echo './target/release/landalock_sample' >> /usr/local/bin/debug_and_run.sh && \
    chmod +x /usr/local/bin/debug_and_run.sh

# コンテナ起動時にRustアプリケーションを実行
CMD ["./target/release/landalock_sample"]
