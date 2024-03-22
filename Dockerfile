FROM debian:bullseye

WORKDIR /app/

ADD ./target/release/simplylab /app/
ADD ./Rocket.toml /app/
ADD .env /app

CMD ["./simplylab"]