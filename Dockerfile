FROM ubuntu 

WORKDIR /opt

ADD ./ssl_certificate.pem .
ADD ./ssl_private_key.pem .
ADD ./docker-start.sh .
ADD ./static ./static
COPY target/release/scoping-tool-backend .

EXPOSE 80

CMD ["./docker-start.sh"]
