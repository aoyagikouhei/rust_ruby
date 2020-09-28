docker build -t rust_ruby .
docker run -v `pwd`/apps/app:/app -it --rm  rust_ruby bash
. ~/.profile
export USER=root
export LD_LIBRARY_PATH=`ruby -e "puts RbConfig::CONFIG['libdir']"`
cd /app
