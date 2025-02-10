# Deployment & Monitoring Guide

## Deploying the Project

Simple build the project according to [BUILD.md](./BUILD.md) and then deploy the API to a server.

Remember to set the `ETHEREUM_RPC_URL` environment variable to the URL of the Ethereum node you want to use.
In addition, set the `OTEL_EXPORTER_OTLP_ENDPOINT` environment variable to the URL of the OpenTelemetry collector you want to use.

## Monitoring the Project

The project uses [Prometheus](https://prometheus.io/) to collect metrics and [Grafana](https://grafana.com/) to visualize them.

Prometheus is configured to scrape metrics from the project's API.

Grafana is configured to display the metrics collected by Prometheus.

To view the dashboards, run `docker compose up` and navigate to [`http://localhost:3000`](http://localhost:3000).

| File | Purpose |
|------|---------|
| [`infra/otel-collector-config.yaml`](./infra/otel-collector-config.yaml) | Sets up how our monitoring system collects and sends data about our API. |
| [`infra/prometheus.yml`](./infra/prometheus.yml) | Tells Prometheus (our metrics database) where to find the data it needs to store. |
| [`infra/grafana/datasources/prometheus.yml`](./infra/grafana/datasources/prometheus.yml) | Helps Grafana (our metrics dashboard) find and display the data from Prometheus. |
