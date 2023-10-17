FROM rust:slim-bookworm AS builder

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Install wasm-pack for testing
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install trunk
ADD https://github.com/thedodd/trunk/releases/download/v0.17.5/trunk-x86_64-unknown-linux-gnu.tar.gz ./tmp
RUN cd /tmp && tar xf trunk-x86_64-unknown-linux-gnu.tar.gz && chmod +x trunk && mv trunk /bin

WORKDIR /usr/src/app


COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN touch index.html
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" > src/main.rs

RUN trunk build --release

RUN rm index.html
RUN rm -r src
RUN rm ./target/wasm32-unknown-unknown/release/deps/eduverse_frontend*

COPY src src
COPY assets assets
COPY js js
COPY index.html index.html
COPY index.scss index.scss

RUN trunk build --release


FROM nginx:alpine as runner

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]


