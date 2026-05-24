CREATE TABLE Clientes_Corp (
    id_cliente SERIAL PRIMARY KEY,
    nombre_empresa VARCHAR(100) NOT NULL,
    pais VARCHAR(50),
    contacto_principal VARCHAR(100)
);

CREATE TABLE Proyectos (
    id_proyecto SERIAL PRIMARY KEY,
    nombre_proyecto VARCHAR(150) NOT NULL,
    fecha_inicio DATE,
    id_cliente INT REFERENCES Clientes_Corp(id_cliente)
);

CREATE TABLE Desarrolladores (
    id_desarrollador SERIAL PRIMARY KEY,
    nombre VARCHAR(100) NOT NULL,
    rol_principal VARCHAR(50), -- 'Frontend', 'Backend', 'QA'
    nivel VARCHAR(20) -- 'Junior', 'Mid', 'Senior'
);

CREATE TABLE Tareas (
    id_tarea SERIAL PRIMARY KEY,
    id_proyecto INT REFERENCES Proyectos(id_proyecto),
    descripcion TEXT NOT NULL,
    prioridad VARCHAR(15) CHECK (prioridad IN ('Alta', 'Media', 'Baja')),
    estado VARCHAR(20) DEFAULT 'Pendiente'
);

CREATE TABLE Asignaciones (
    id_asignacion SERIAL PRIMARY KEY,
    id_tarea INT REFERENCES Tareas(id_tarea),
    id_desarrollador INT REFERENCES Desarrolladores(id_desarrollador),
    horas_estimadas INT
);