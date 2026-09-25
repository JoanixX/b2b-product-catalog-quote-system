-- Keep historical migrations immutable. Revoke the historical seeded credential
-- before the server can start. A deliberately replaced password is preserved.
DELETE FROM admins WHERE email = 'admin@b2bproductcatalogquotesystem.com'
AND password_hash = '$argon2id$v=19$m=19456,t=2,p=1$c2VjdXJlc2FsdGxhYm1lZA$QsM+5bLhEfQkuWfJOBGkVoUdqz3bGJhRkGF2vNNCaQo';

CREATE TABLE pilot_quotes (
    id UUID PRIMARY KEY,
    request JSONB NOT NULL,
    company TEXT NOT NULL,
    contact TEXT NOT NULL,
    product TEXT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity BETWEEN 1 AND 10000),
    unit_price_cents BIGINT NOT NULL DEFAULT 0 CHECK (unit_price_cents BETWEEN 0 AND 1000000000),
    tax_bps INTEGER NOT NULL DEFAULT 0 CHECK (tax_bps BETWEEN 0 AND 10000),
    currency TEXT NOT NULL DEFAULT 'PEN' CHECK (currency IN ('PEN','USD')),
    terms TEXT NOT NULL DEFAULT '',
    status TEXT NOT NULL DEFAULT 'received' CHECK (status IN ('received','draft','approved','proposal','followup')),
    version INTEGER NOT NULL DEFAULT 1,
    approved_by TEXT,
    approved_at TIMESTAMPTZ,
    proposal JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TABLE pilot_events (
    id BIGSERIAL PRIMARY KEY,
    quote_id UUID NOT NULL REFERENCES pilot_quotes(id),
    actor TEXT NOT NULL,
    action TEXT NOT NULL,
    version INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX pilot_events_quote ON pilot_events(quote_id, id);
