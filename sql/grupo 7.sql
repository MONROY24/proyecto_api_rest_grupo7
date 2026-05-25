--
-- PostgreSQL database dump
--

\restrict sdRtz7eQDV9l74KF1Rdza5JbAOaj7zj1YWynESslFhLgULf3E9tZi2HL6Q0INHg

-- Dumped from database version 18.4
-- Dumped by pg_dump version 18.4

-- Started on 2026-05-25 00:52:26

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- TOC entry 228 (class 1259 OID 16440)
-- Name: asignaciones; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.asignaciones (
    id_asignacion integer NOT NULL,
    id_tarea integer,
    id_desarrollador integer,
    horas_estimadas integer
);


ALTER TABLE public.asignaciones OWNER TO postgres;

--
-- TOC entry 227 (class 1259 OID 16439)
-- Name: asignaciones_id_asignacion_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.asignaciones_id_asignacion_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.asignaciones_id_asignacion_seq OWNER TO postgres;

--
-- TOC entry 5059 (class 0 OID 0)
-- Dependencies: 227
-- Name: asignaciones_id_asignacion_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.asignaciones_id_asignacion_seq OWNED BY public.asignaciones.id_asignacion;


--
-- TOC entry 220 (class 1259 OID 16390)
-- Name: clientes_corp; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.clientes_corp (
    id_cliente integer NOT NULL,
    nombre_empresa character varying(100) NOT NULL,
    pais character varying(50),
    contacto_principal character varying(100)
);


ALTER TABLE public.clientes_corp OWNER TO postgres;

--
-- TOC entry 219 (class 1259 OID 16389)
-- Name: clientes_corp_id_cliente_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.clientes_corp_id_cliente_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.clientes_corp_id_cliente_seq OWNER TO postgres;

--
-- TOC entry 5060 (class 0 OID 0)
-- Dependencies: 219
-- Name: clientes_corp_id_cliente_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.clientes_corp_id_cliente_seq OWNED BY public.clientes_corp.id_cliente;


--
-- TOC entry 224 (class 1259 OID 16413)
-- Name: desarrolladores; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.desarrolladores (
    id_desarrollador integer NOT NULL,
    nombre character varying(100) NOT NULL,
    rol_principal character varying(50),
    nivel character varying(20)
);


ALTER TABLE public.desarrolladores OWNER TO postgres;

--
-- TOC entry 223 (class 1259 OID 16412)
-- Name: desarrolladores_id_desarrollador_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.desarrolladores_id_desarrollador_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.desarrolladores_id_desarrollador_seq OWNER TO postgres;

--
-- TOC entry 5061 (class 0 OID 0)
-- Dependencies: 223
-- Name: desarrolladores_id_desarrollador_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.desarrolladores_id_desarrollador_seq OWNED BY public.desarrolladores.id_desarrollador;


--
-- TOC entry 222 (class 1259 OID 16399)
-- Name: proyectos; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.proyectos (
    id_proyecto integer NOT NULL,
    nombre_proyecto character varying(150) NOT NULL,
    fecha_inicio date,
    id_cliente integer
);


ALTER TABLE public.proyectos OWNER TO postgres;

--
-- TOC entry 221 (class 1259 OID 16398)
-- Name: proyectos_id_proyecto_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.proyectos_id_proyecto_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.proyectos_id_proyecto_seq OWNER TO postgres;

--
-- TOC entry 5062 (class 0 OID 0)
-- Dependencies: 221
-- Name: proyectos_id_proyecto_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.proyectos_id_proyecto_seq OWNED BY public.proyectos.id_proyecto;


--
-- TOC entry 226 (class 1259 OID 16422)
-- Name: tareas; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.tareas (
    id_tarea integer NOT NULL,
    id_proyecto integer,
    descripcion text NOT NULL,
    prioridad character varying(15),
    estado character varying(20) DEFAULT 'Pendiente'::character varying,
    CONSTRAINT tareas_prioridad_check CHECK (((prioridad)::text = ANY ((ARRAY['Alta'::character varying, 'Media'::character varying, 'Baja'::character varying])::text[])))
);


ALTER TABLE public.tareas OWNER TO postgres;

--
-- TOC entry 225 (class 1259 OID 16421)
-- Name: tareas_id_tarea_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.tareas_id_tarea_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.tareas_id_tarea_seq OWNER TO postgres;

--
-- TOC entry 5063 (class 0 OID 0)
-- Dependencies: 225
-- Name: tareas_id_tarea_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.tareas_id_tarea_seq OWNED BY public.tareas.id_tarea;


--
-- TOC entry 4881 (class 2604 OID 16443)
-- Name: asignaciones id_asignacion; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asignaciones ALTER COLUMN id_asignacion SET DEFAULT nextval('public.asignaciones_id_asignacion_seq'::regclass);


--
-- TOC entry 4876 (class 2604 OID 16393)
-- Name: clientes_corp id_cliente; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes_corp ALTER COLUMN id_cliente SET DEFAULT nextval('public.clientes_corp_id_cliente_seq'::regclass);


--
-- TOC entry 4878 (class 2604 OID 16416)
-- Name: desarrolladores id_desarrollador; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.desarrolladores ALTER COLUMN id_desarrollador SET DEFAULT nextval('public.desarrolladores_id_desarrollador_seq'::regclass);


--
-- TOC entry 4877 (class 2604 OID 16402)
-- Name: proyectos id_proyecto; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proyectos ALTER COLUMN id_proyecto SET DEFAULT nextval('public.proyectos_id_proyecto_seq'::regclass);


--
-- TOC entry 4879 (class 2604 OID 16425)
-- Name: tareas id_tarea; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tareas ALTER COLUMN id_tarea SET DEFAULT nextval('public.tareas_id_tarea_seq'::regclass);


--
-- TOC entry 5053 (class 0 OID 16440)
-- Dependencies: 228
-- Data for Name: asignaciones; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.asignaciones (id_asignacion, id_tarea, id_desarrollador, horas_estimadas) FROM stdin;
1	1	1	25
2	2	3	40
3	3	5	15
4	4	3	20
5	1	2	10
\.


--
-- TOC entry 5045 (class 0 OID 16390)
-- Dependencies: 220
-- Data for Name: clientes_corp; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.clientes_corp (id_cliente, nombre_empresa, pais, contacto_principal) FROM stdin;
6	Innovación financiera s.a.	Panamá	Lic. Martínez
7	Logística centroamericana	El Salvador	Ing. Ramos
\.


--
-- TOC entry 5049 (class 0 OID 16413)
-- Dependencies: 224
-- Data for Name: desarrolladores; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.desarrolladores (id_desarrollador, nombre, rol_principal, nivel) FROM stdin;
1	Josué	Frontend	Mid
2	Benito	QA	Junior
3	Moisés	Backend	Mid
4	Jonathan	Base de Datos	Junior
5	Melvin	QA	mid
\.


--
-- TOC entry 5047 (class 0 OID 16399)
-- Dependencies: 222
-- Data for Name: proyectos; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.proyectos (id_proyecto, nombre_proyecto, fecha_inicio, id_cliente) FROM stdin;
1	App de billetera virtual	2026-06-01	6
2	Sistema de control de flotas	2026-06-15	7
\.


--
-- TOC entry 5051 (class 0 OID 16422)
-- Dependencies: 226
-- Data for Name: tareas; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.tareas (id_tarea, id_proyecto, descripcion, prioridad, estado) FROM stdin;
1	1	Crear interfaz de login y dashboard de usuario	Alta	Pendiente
2	1	Desarrollar API REST para transacciones financieras	Alta	En progreso
3	2	Diseñar esquema de base de datos para los vehículos	Media	Completado
4	2	Pruebas de estrés e integración de GPS	Alta	Pendiente
\.


--
-- TOC entry 5064 (class 0 OID 0)
-- Dependencies: 227
-- Name: asignaciones_id_asignacion_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.asignaciones_id_asignacion_seq', 5, true);


--
-- TOC entry 5065 (class 0 OID 0)
-- Dependencies: 219
-- Name: clientes_corp_id_cliente_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.clientes_corp_id_cliente_seq', 7, true);


--
-- TOC entry 5066 (class 0 OID 0)
-- Dependencies: 223
-- Name: desarrolladores_id_desarrollador_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.desarrolladores_id_desarrollador_seq', 5, true);


--
-- TOC entry 5067 (class 0 OID 0)
-- Dependencies: 221
-- Name: proyectos_id_proyecto_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.proyectos_id_proyecto_seq', 2, true);


--
-- TOC entry 5068 (class 0 OID 0)
-- Dependencies: 225
-- Name: tareas_id_tarea_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.tareas_id_tarea_seq', 4, true);


--
-- TOC entry 4892 (class 2606 OID 16446)
-- Name: asignaciones asignaciones_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asignaciones
    ADD CONSTRAINT asignaciones_pkey PRIMARY KEY (id_asignacion);


--
-- TOC entry 4884 (class 2606 OID 16397)
-- Name: clientes_corp clientes_corp_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.clientes_corp
    ADD CONSTRAINT clientes_corp_pkey PRIMARY KEY (id_cliente);


--
-- TOC entry 4888 (class 2606 OID 16420)
-- Name: desarrolladores desarrolladores_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.desarrolladores
    ADD CONSTRAINT desarrolladores_pkey PRIMARY KEY (id_desarrollador);


--
-- TOC entry 4886 (class 2606 OID 16406)
-- Name: proyectos proyectos_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proyectos
    ADD CONSTRAINT proyectos_pkey PRIMARY KEY (id_proyecto);


--
-- TOC entry 4890 (class 2606 OID 16433)
-- Name: tareas tareas_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tareas
    ADD CONSTRAINT tareas_pkey PRIMARY KEY (id_tarea);


--
-- TOC entry 4895 (class 2606 OID 16452)
-- Name: asignaciones asignaciones_id_desarrollador_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asignaciones
    ADD CONSTRAINT asignaciones_id_desarrollador_fkey FOREIGN KEY (id_desarrollador) REFERENCES public.desarrolladores(id_desarrollador);


--
-- TOC entry 4896 (class 2606 OID 16447)
-- Name: asignaciones asignaciones_id_tarea_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.asignaciones
    ADD CONSTRAINT asignaciones_id_tarea_fkey FOREIGN KEY (id_tarea) REFERENCES public.tareas(id_tarea);


--
-- TOC entry 4893 (class 2606 OID 16407)
-- Name: proyectos proyectos_id_cliente_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.proyectos
    ADD CONSTRAINT proyectos_id_cliente_fkey FOREIGN KEY (id_cliente) REFERENCES public.clientes_corp(id_cliente);


--
-- TOC entry 4894 (class 2606 OID 16434)
-- Name: tareas tareas_id_proyecto_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.tareas
    ADD CONSTRAINT tareas_id_proyecto_fkey FOREIGN KEY (id_proyecto) REFERENCES public.proyectos(id_proyecto);


-- Completed on 2026-05-25 00:52:27

--
-- PostgreSQL database dump complete
--

\unrestrict sdRtz7eQDV9l74KF1Rdza5JbAOaj7zj1YWynESslFhLgULf3E9tZi2HL6Q0INHg

