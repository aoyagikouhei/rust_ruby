docker build -t rust_ruby .
docker run -v `pwd`/apps/app:/app -it --rm  rust_ruby bash
. ~/.profile
cd /app
