# Doesn't matter what we put here - it get's overwritten by the docker \
# build command
FROM node:23 AS build_image
ARG BUILD_CONFIGURATION=production
WORKDIR /app
COPY . .
RUN npm install && \
    node_modules/.bin/ng build --configuration \
        ${BUILD_CONFIGURATION} && \
    npm prune --production && \
    echo "---------- USING APACHE ----------"


FROM httpd:2.4

# hadolint ignore=DL3008
RUN apt-get update && \
    apt-get install -y --no-install-recommends curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=BUILD_IMAGE /app/dist/frontend/browser \
    /usr/local/apache2/htdocs/
COPY --from=BUILD_IMAGE /app/httpd.conf \
    /usr/local/apache2/conf/httpd.conf

COPY docker/startup.sh /usr/local/bin/startup.sh
RUN chmod +x /usr/local/bin/startup.sh

ENTRYPOINT ["/usr/local/bin/startup.sh"]

CMD ["httpd-foreground"]
