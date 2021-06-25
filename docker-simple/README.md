# Docker Simple

[原文地址](https://kerkour.com/blog/rust-small-docker-image/)

1. alpine

   ```sh
   docker build -t xy/docker-simple:alpine -f Dockerfile.alpine .

   docker run -it --rm xy/docker-simple:alpine
   ```

1. debian

   ```sh
   docker build -t xy/docker-simple:debian -f Dockerfile.debian .

   docker run -it --rm xy/docker-simple:debian
   ```
