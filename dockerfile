FROM golang:1.22.5-alpine

WORKDIR /dino

COPY go.mod ./
RUN go mod download

COPY . .

RUN go build -o dino

CMD ["./dino"]
expose 8080
