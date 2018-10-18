FROM ubuntu 

WORKDIR /opt

ADD ./docker-start.sh .
ADD ./static ./static
COPY target/release/scoping-tool-backend .

EXPOSE 80

CMD ["./docker-start.sh"]
