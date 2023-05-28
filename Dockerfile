# 编译
FROM --platform=$TARGETPLATFORM rust:1.68-alpine3.17 as builder

WORKDIR /usr/src

RUN USER=root cargo new dm-ticket

RUN  sed -i "s/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g" /etc/apk/repositories;

RUN apk add musl-dev openssl openssl-dev pkgconfig upx git curl

COPY Cargo.toml Cargo.lock /usr/src/dm-ticket/

COPY .cargo /usr/src/dm-ticket/.cargo

WORKDIR /usr/src/dm-ticket

RUN cargo build --release --verbose

COPY src /usr/src/dm-ticket/src/

RUN RUST_BACKTRACE=1 cargo build  --release && upx /usr/src/dm-ticket/target/release/dm-ticket

FROM --platform=$TARGETPLATFORM alpine:3.17 as runtime

ENV TZ=Asia/Shanghai

RUN sed -i "s/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g" /etc/apk/repositories \
    && apk update  \
    && apk add --no-cache vim tzdata \
    && echo "${TZ}" > /etc/timezone \
    && ln -sf /usr/share/zoneinfo/${TZ} /etc/localtime \
    && rm -rf /var/cache/apk/*

WORKDIR /src/

COPY --from=builder /usr/src/dm-ticket/target/release/dm-ticket /usr/bin/dm-ticket

CMD ["/usr/sbin/crond", "-f", "-d", "0"]
