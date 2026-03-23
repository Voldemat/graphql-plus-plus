ARG BASE_IMAGE
FROM ${BASE_IMAGE}
WORKDIR /usr/local/app/
COPY shared/graphql/ ./
RUN /gql validate
