.PHONY: e2e

e2e:
	docker-compose -f docker-compose.e2e.yaml up --build --abort-on-container-exit
	docker-compose -f docker-compose.e2e.yaml down --volumes

dev:
	docker-compose -f docker-compose.dev.yaml up --build -d

clear:
	docker-compose -f docker-compose.dev.yaml down --volumes
