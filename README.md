# Сервер приложения Termtalk

Серверная часть работы приложения Termtalk

## Запуск проекта

1. Клонировать репозиторий

```bash
git clone https://github.com/NVinox/termtalk-websocket.git
```

2. Объявить переменные окружения (**.env.development** - для dev, **.env.production** - для prod)

```bash
cp .env.example .env.development && cp .env.example .env.production
```

3. Собрать и запустить Docker

   3.1 Режим development

   ```bash
    make dev-up
   ```

   3.2 Режим production

   ```bash
    make prod-up
   ```

## Сервис готов к работе
