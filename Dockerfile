FROM debian:bullseye
ADD ./target/release/simplylab /app/
ADD ./Rocket.toml /app/

WORKDIR /app/

CMD ["./simplylab"]