FROM --platform=$BUILDPLATFORM python:3.13-alpine AS builder

WORKDIR /app

COPY requirements.txt /app
RUN --mount=type=cache,target=/root/.cache/pip \
    pip3 install -r requirements.txt

COPY . /app

EXPOSE 5000

ENTRYPOINT ["python3"]
CMD ["app.py"]

