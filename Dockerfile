FROM gcc:latest

WORKDIR /opt/Saba

RUN apt-get -y update && apt-get install -y cmake
RUN useradd -r  saba
RUN groupmod -o -g 1000 saba
RUN usermod -o -u 1000 saba
USER saba

EXPOSE 8585

CMD ["bash"]

LABEL Name=saba Version=0.0.1
