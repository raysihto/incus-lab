FROM alpine:3.20
RUN echo "buildx test" > /hello.txt
CMD ["cat", "/hello.txt"]
