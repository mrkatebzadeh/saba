FROM ubuntu:latest

WORKDIR /opt/Saba

RUN apt-get -y update && DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt-get install -y cmake sudo git gcc g++ cmake make libibverbs-dev curl gcovr lcov llvm pkg-config curl zip unzip tar python3-dev clang-format clang-tidy cppcheck iwyu
RUN useradd -r  saba && echo "saba:saba" | chpasswd && adduser saba sudo
RUN groupmod -o -g 1000 saba
RUN usermod -o -u 1000 saba
USER saba

EXPOSE 8585

CMD ["bash"]

LABEL Name=saba Version=0.0.1
