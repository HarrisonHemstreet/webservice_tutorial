FROM rust:1.67

WORKDIR /usr/src/webservice_tutorial
COPY . .

RUN cargo install --path .

CMD ["webservice_tutorial"]
