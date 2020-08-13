FROM grafana/grafana

WORKDIR /usr/src/custom-grafana

RUN grafana-cli plugins install larona-epict-panel
RUN grafana-cli plugins install redis-datasource
RUN grafana-cli plugins install cloudspout-button-panel
RUN grafana-cli plugins install grafana-clock-panel

COPY . .

EXPOSE 3000
