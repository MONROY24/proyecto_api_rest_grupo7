# 🚀 API REST — Gestión de Proyectos de Software

> Proyecto universitario desarrollado en **Rust** con **Axum** y **PostgreSQL**  
> Grupo 7

---

## 📋 Descripción

API REST para la gestión integral de proyectos de software. Permite administrar clientes corporativos, proyectos, desarrolladores, tareas y asignaciones, además de generar un reporte general consolidado con toda la información relacionada.

---

## 🛠️ Tecnologías utilizadas

| Tecnología | Versión | Rol |
|---|---|---|
| [Rust](https://www.rust-lang.org/) | Edition 2024 | Lenguaje principal |
| [Axum](https://github.com/tokio-rs/axum) | 0.8 | Framework web |
| [SQLx](https://github.com/launchbadge/sqlx) | 0.8.6 | ORM / queries async |
| [Tokio](https://tokio.rs/) | 1.44 | Runtime asíncrono |
| [PostgreSQL](https://www.postgresql.org/) | — | Base de datos |
| [Serde](https://serde.rs/) | 1.0 | Serialización JSON |
| [Chrono](https://docs.rs/chrono) | 0.4 | Manejo de fechas |
| [Dotenvy](https://docs.rs/dotenvy) | 0.15 | Variables de entorno |

---

## 🗂️ Estructura del proyecto

```
proyecto_api_rest_grupo7/
├── sql/
│   └── grupo 7.sql          # Script para crear la base de datos
├── src/
│   ├── config/
│   │   └── config.rs        # Configuración del pool de conexión
│   ├── controller/          # Capa de controladores (rutas HTTP)
│   │   ├── clientes_corp.rs
│   │   ├── proyectos.rs
│   │   ├── desarrolladores.rs
│   │   ├── asignacion.rs
│   │   ├── tarea.rs
│   │   └── reporte.rs
│   ├── models/              # Estructuras de datos (DTOs)
│   │   ├── clientes_corp.rs
│   │   ├── proyectos.rs
│   │   ├── desarrolladores_model.rs
│   │   ├── asignacion.rs
│   │   ├── tarea.rs
│   │   └── reporte.rs
│   ├── repository/          # Capa de acceso a datos (SQLx queries)
│   │   ├── clientes_corp.rs
│   │   ├── proyectos_repository.rs
│   │   ├── desarrolladores_repository.rs
│   │   ├── asignacion_repository.rs
│   │   ├── tarea_repository.rs
│   │   └── reporte_repository.rs
│   ├── service/             # Capa de lógica de negocio
│   │   ├── clientes_corp.rs
│   │   ├── proyectos_service.rs
│   │   ├── desarrolladores_service.rs
│   │   ├── asignacion_service.rs
│   │   ├── tarea.rs
│   │   └── reporte_service.rs
│   ├── utils/
│   │   └── util.rs
│   └── main.rs              # Punto de entrada y registro de rutas
├── .env.example             # Plantilla de variables de entorno
├── .gitignore
├── Cargo.toml
└── Cargo.lock
```

---

## 🗄️ Modelo de base de datos

El sistema maneja 5 tablas relacionadas entre sí:

```
clientes_corp
    └── proyectos (id_cliente → clientes_corp)
            └── tareas (id_proyecto → proyectos)
                    └── asignaciones (id_tarea → tareas)
                                      (id_desarrollador → desarrolladores)

desarrolladores
    └── asignaciones (id_desarrollador → desarrolladores)
```

### Tablas

- **clientes_corp** — Empresas clientes con país y contacto principal
- **proyectos** — Proyectos vinculados a un cliente, con fecha de inicio
- **desarrolladores** — Equipo de desarrollo con rol y nivel de experiencia
- **tareas** — Tareas de cada proyecto con prioridad y estado
- **asignaciones** — Relación tarea–desarrollador con horas estimadas

---

## ⚙️ Configuración e instalación

### Prerrequisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [PostgreSQL](https://www.postgresql.org/download/) instalado y corriendo localmente

### 1. Clonar el repositorio

```bash
git clone https://github.com/tu-usuario/proyecto_api_rest_grupo7.git
cd proyecto_api_rest_grupo7
```

### 2. Crear la base de datos

Ejecuta el script SQL en tu cliente de PostgreSQL (psql, pgAdmin, DBeaver, etc.):

```bash
psql -U postgres -f "sql/grupo 7.sql"
```

### 3. Configurar las variables de entorno

Copia el archivo de ejemplo y edítalo con tus credenciales:

```bash
cp .env.example .env
```

Abre `.env` y ajusta los datos:

```env
DATABASE_URL=postgres://tu_usuario:tu_contraseña@localhost:5432/tu_base_de_datos
```

### 4. Compilar y ejecutar

```bash
cargo run
```

El servidor quedará escuchando en: `http://127.0.0.1:3000`

---

## 📡 Endpoints de la API

El servidor base es: `http://127.0.0.1:3000`

### Clientes Corporativos — `/api/clientes`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/clientes` | Obtener todos los clientes |
| `POST` | `/api/clientes` | Crear un nuevo cliente |
| `GET` | `/api/clientes/{id}` | Obtener cliente por ID |
| `PUT` | `/api/clientes/{id}` | Actualizar cliente |
| `DELETE` | `/api/clientes/{id}` | Eliminar cliente |

**Body para POST / PUT:**
```json
{
  "nombre_empresa": "Tech Solutions S.A.",
  "pais": "El Salvador",
  "contacto_principal": "Juan Pérez"
}
```

---

### Proyectos — `/api/proyectos`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/proyectos` | Obtener todos los proyectos |
| `POST` | `/api/proyectos` | Crear un nuevo proyecto |
| `GET` | `/api/proyectos/{id}` | Obtener proyecto por ID |
| `PUT` | `/api/proyectos/{id}` | Actualizar proyecto |
| `DELETE` | `/api/proyectos/{id}` | Eliminar proyecto |

**Body para POST / PUT:**
```json
{
  "nombre_proyecto": "Sistema de Inventario",
  "fecha_inicio": "2026-01-15",
  "id_cliente": 1
}
```

---

### Desarrolladores — `/api/desarrolladores`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/desarrolladores` | Listar todos los desarrolladores |
| `POST` | `/api/desarrolladores` | Registrar un desarrollador |
| `GET` | `/api/desarrolladores/{id}` | Obtener desarrollador por ID |
| `DELETE` | `/api/desarrolladores/{id}` | Eliminar desarrollador |

**Body para POST:**
```json
{
  "nombre": "María López",
  "rol_principal": "Backend Developer",
  "nivel": "Senior"
}
```

---

### Tareas — `/api/tareas`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/tareas` | Obtener todas las tareas |
| `POST` | `/api/tareas` | Crear una nueva tarea |
| `GET` | `/api/tareas/{id}` | Obtener tarea por ID |
| `PUT` | `/api/tareas/{id}` | Actualizar tarea |
| `DELETE` | `/api/tareas/{id}` | Eliminar tarea |

**Body para POST / PUT:**
```json
{
  "id_proyecto": 1,
  "descripcion": "Diseñar esquema de base de datos",
  "prioridad": "Alta",
  "estado": "En progreso"
}
```

---

### Asignaciones — `/api/asignaciones`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/asignaciones` | Obtener todas las asignaciones |
| `POST` | `/api/asignaciones` | Crear una asignación |
| `GET` | `/api/asignaciones/{id}` | Obtener asignación por ID |
| `PUT` | `/api/asignaciones/{id}` | Actualizar asignación |
| `DELETE` | `/api/asignaciones/{id}` | Eliminar asignación |

**Body para POST / PUT:**
```json
{
  "id_tarea": 1,
  "id_desarrollador": 2,
  "horas_estimadas": 8
}
```

---

### Reporte General — `/api/reporte-general`

| Método | Ruta | Descripción |
|---|---|---|
| `GET` | `/api/reporte-general` | Reporte consolidado de todo el sistema |

Devuelve una vista completa que combina: cliente, proyecto, tarea, desarrollador y horas asignadas.

**Respuesta de ejemplo:**
```json
[
  {
    "nombre_empresa": "Tech Solutions S.A.",
    "pais": "El Salvador",
    "nombre_proyecto": "Sistema de Inventario",
    "fecha_inicio": "2026-01-15",
    "descripcion_tarea": "Diseñar esquema de base de datos",
    "prioridad": "Alta",
    "estado": "En progreso",
    "nombre_desarrollador": "María López",
    "rol_principal": "Backend Developer",
    "horas_estimadas": 8
  }
]
```

---

## 🏗️ Arquitectura

El proyecto sigue una arquitectura en capas:

```
Request HTTP
     ↓
Controller  →  Manejo de rutas y respuestas HTTP (Axum handlers)
     ↓
Service     →  Lógica de negocio
     ↓
Repository  →  Acceso a base de datos (SQLx queries)
     ↓
PostgreSQL
```

---

## 👥 Integrantes
Monroy Rodriguez, Melvin  Rodriguez
Palma Rodriguez, Carlos Benito 
Escobar Arriaga, Josue Giovani
Polanco Vega, Bryan Moises
Jonathan Steven, Quinteros Rivas
Grupo 7

---

## 📄 Licencia

Proyecto académico — uso educativo.
