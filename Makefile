.PHONY: e2e

e2e:
	docker-compose -f docker-compose.e2e.yaml up --build --abort-on-container-exit
	docker-compose -f docker-compose.e2e.yaml down --volumes
