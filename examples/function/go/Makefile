all: build

build: deps
	GOOS=linux go build -ldflags "-s -w" main.go
	zip lambda.zip main

deps:
	go mod tidy

clean:
	rm -f go.sum *.zip main

local:
	docker build -t tc-examples/basic-go:latest .
