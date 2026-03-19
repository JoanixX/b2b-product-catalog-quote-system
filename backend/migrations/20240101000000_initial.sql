-- habilitar extension uuid
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- tabla de categorias
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- tabla de productos (equipos medicos)
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    category_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
    brand VARCHAR(200) NOT NULL,
    model_number VARCHAR(200),
    origin_country VARCHAR(100) NOT NULL DEFAULT 'Peru',
    warranty_period INTEGER NOT NULL DEFAULT 12,
    technical_sheet_url VARCHAR(500),
    registro_sanitario VARCHAR(100) NOT NULL,
    specifications JSONB DEFAULT '{}',
    image_url VARCHAR(500),
    additional_images JSONB DEFAULT '[]',
    regulatory_info JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- tabla de cotizaciones
CREATE TABLE quotes (
    id SERIAL PRIMARY KEY,
    company_name VARCHAR(255) NOT NULL,
    company_tax_id VARCHAR(11) NOT NULL,
    contact_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    product_ids INTEGER[] NOT NULL,
    estimated_quantity TEXT,
    message TEXT,
    status VARCHAR(50) DEFAULT 'pending',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    contacted_at TIMESTAMPTZ,
    notes TEXT
);

-- tabla de administradores
CREATE TABLE admins (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_login TIMESTAMPTZ
);

-- indices de rendimiento
CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_active ON products(is_active);
CREATE INDEX idx_products_slug ON products(slug);
CREATE INDEX idx_products_brand ON products(brand);
CREATE INDEX idx_products_registro ON products(registro_sanitario);
CREATE INDEX idx_quotes_status ON quotes(status);
CREATE INDEX idx_quotes_created ON quotes(created_at DESC);
CREATE INDEX idx_quotes_ruc ON quotes(company_tax_id);

-- indice de busqueda de texto completo
CREATE INDEX idx_products_search ON products 
USING gin(to_tsvector('spanish', name || ' ' || COALESCE(description, '')));

-- usuario admin por defecto
-- contrasena: admin123 (hash argon2id)
INSERT INTO admins (email, password_hash, name) 
VALUES (
    'admin@labmedical.com', 
    '$argon2id$v=19$m=19456,t=2,p=1$c2VjdXJlc2FsdGxhYm1lZA$QsM+5bLhEfQkuWfJOBGkVoUdqz3bGJhRkGF2vNNCaQo',
    'Administrador'
);

-- categorias de ejemplo
INSERT INTO categories (name, slug, description) VALUES
('Equipos Medicos', 'equipos-medicos', 'Equipos y dispositivos medicos profesionales'),
('Instrumental Quirurgico', 'instrumental-quirurgico', 'Instrumentos para procedimientos quirurgicos'),
('Consumibles', 'consumibles', 'Productos de uso medico desechables'),
('Mobiliario Clinico', 'mobiliario-clinico', 'Mobiliario y equipamiento para clinicas y hospitales');