.PHONY: dev-up dev-stop dev-down prod-up prod-stop prod-down prod-rebuild logs clean


dev-up:
	docker compose --env-file .env.development up dev -d --build

dev-stop:
	docker compose stop dev

dev-down:
	docker compose rm -s -f dev

prod-up:
	docker compose --env-file .env.production up prod -d --build

prod-stop:
	docker compose stop prod

prod-down:
	docker compose rm -s -f prod

prod-rebuild:
	docker compose build --no-cache prod
	docker compose up -d prod

logs:
	docker compose logs -f

clean:
	docker compose down --remove-orphans -v