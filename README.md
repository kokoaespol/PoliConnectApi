# 🔗 PoliConnect API

PoliConnect es una API hecha por y para los estudiantes politécnicos que abarca los diferentes
aspectos de la vida estudiantil politécnica.

Para más información, visita [nuestro sitio web](https://policonnect.clubkokoa.com).

## Corre la API localmente

Para correr la API localmente debes tener instalado [Docker](https://www.docker.com/)
o [Rust](https://www.rust-lang.org/learn/get-started).

### Con Docker (recomendado)

```bash
# Clona el repositorio
git clone https://github.com/kokoaespol/PoliConnectApi

# Entra al directorio de la API
cd PoliConnectApi

# Levanta los contenedores con docker-compose
docker-compose up
```

### Con Rust

Crea un `.env` con una variable `PORT` que apunte al puerto donde deseas correr la API.
Luego:

```bash
# Clona el repositorio
git clone https://github.com/kokoaespol/PoliConnectApi

# Entra al directorio de la API
cd PoliConnectApi

# Compila el proyecto
# WARNING: Esto puede tomar hasta 69 años
cargo build --release

# Corre el ejecutable
cargo run
```
