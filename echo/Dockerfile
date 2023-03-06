FROM golang:1.19.5 AS builder
RUN go env -w GOPROXY=direct
WORKDIR /app/
COPY . .
RUN go mod download
RUN go mod tidy
RUN CGO_ENABLED=0 go build -o echo-server .

FROM gcr.io/distroless/static:nonroot
COPY --from=builder --chown=nonroot:nonroot /app/echo-server ./echo-server
EXPOSE 8888
ENTRYPOINT [ "./echo-server"]