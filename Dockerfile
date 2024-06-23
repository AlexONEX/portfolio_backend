# Usa la imagen oficial de Rust como base
FROM rust:1.53 as builder

# Crea un nuevo proyecto ficticio para cachear las dependencias
RUN USER=root cargo new --bin app
WORKDIR /app

# Copia los archivos de configuración del proyecto y descarga las dependencias
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copia tu código fuente real
COPY ./src ./src

# Construye la aplicación para producción
RUN rm ./target/release/deps/app*
RUN cargo build --release

# Etapa final que solo toma el binario compilado
FROM debian:buster-slim
COPY --from=builder /app/target/release/app /usr/local/bin/app

# Ejecutar la aplicación
CMD ["app"]
