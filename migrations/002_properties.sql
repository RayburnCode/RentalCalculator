CREATE TYPE property_type AS ENUM (
    'singlefamily',
    'multifamily',
    'apartment',
    'condominium',
    'townhouse',
    'duplex',
    'triplex',
    'fourplex',
    'commercial',
    'mixeduse',
    'vacationrental',
    'mobilehome',
    'studenthousing',
    'seniorhousing',
    'colivingspace',
    'storageunit',
    'parkingspace'
);

CREATE TYPE furnishing_type AS ENUM (
    'unfurnished',
    'partiallyfurnished',
    'fullyfurnished'
);

CREATE TYPE rental_status AS ENUM (
    'vacant',
    'occupied',
    'undermaintenance',
    'underrenovation',
    'listed'
);

CREATE TYPE lease_term AS ENUM (
    'monthly',
    'quarterly',
    'biannual',
    'annual',
    'fixedterm',
    'monthtomonth'
);

CREATE TYPE payment_method AS ENUM (
    'cash',
    'check',
    'banktransfer',
    'creditcard',
    'debitcard',
    'onlinepayment',
    'moneyorder'
);

CREATE TYPE expense_category AS ENUM (
    'mortgage',
    'propertytax',
    'insurance',
    'hoafees',
    'utilities',
    'maintenance',
    'repairs',
    'propertymanagement',
    'advertising',
    'legalfees',
    'accounting',
    'capitalimprovements',
    'vacancyreserve',
    'other'
);

CREATE TYPE income_category AS ENUM (
    'rent',
    'latefees',
    'parkingfees',
    'storagefees',
    'laundry',
    'petfees',
    'applicationfees',
    'leasebreakfees',
    'other'
);

CREATE TYPE tax_deduction_category AS ENUM (
    'depreciation',
    'interest',
    'repairs',
    'homeoffice',
    'travel',
    'education',
    'professionalservices',
    'other'
);

CREATE TYPE roi_metric AS ENUM (
    'cashoncash',
    'caprate',
    'grossrentmultiplier',
    'netoperatingincome',
    'internalrateofreturn',
    'equitybuildrate',
    'appreciationrate'
);

CREATE TYPE vacancy_calculation_method AS ENUM (
    'fixedrate',
    'historicalaverage',
    'marketaverage',
    'customdays'
);

CREATE TYPE appreciation_method AS ENUM (
    'none',
    'fixedannualrate',
    'historicalmarketaverage',
    'cpiadjusted',
    'custom'
);

CREATE TYPE financing_type AS ENUM (
    'cash',
    'conventionalloan',
    'fha',
    'va',
    'hardmoney',
    'privatemoney',
    'sellerfinancing',
    'heloc'
);

CREATE TABLE properties (
    id SERIAL PRIMARY KEY,
    property_type PROPERTY_TYPE NOT NULL,
    furnishing_type FURNISHING_TYPE NOT NULL,
    status RENTAL_STATUS NOT NULL DEFAULT 'vacant',
    -- other columns
);

-- Add these additional ENUM types if needed
CREATE TYPE expense_frequency AS ENUM (
    'monthly',
    'quarterly',
    'annual',
    'onetime',
    'pertenant',
    'perunit'
);

CREATE TYPE utilities_responsibility AS ENUM (
    'ownerpaysall',
    'tenantpaysall',
    'split'
);

-- Enhanced properties table with ROI-relevant fields
CREATE TABLE properties (
    id SERIAL PRIMARY KEY,
    property_type PROPERTY_TYPE NOT NULL,
    furnishing_type FURNISHING_TYPE NOT NULL,
    status RENTAL_STATUS NOT NULL DEFAULT 'vacant',
    purchase_price DECIMAL(12, 2) NOT NULL,
    purchase_date DATE NOT NULL,
    square_footage INT,
    bedroom_count INT,
    bathroom_count DECIMAL(3, 1),
    year_built INT,
    -- ROI-specific fields
    current_market_value DECIMAL(12, 2),
    after_repair_value DECIMAL(12, 2),
    appreciation_method APPRECIATION_METHOD DEFAULT 'none',
    annual_appreciation_rate DECIMAL(5, 2) DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Supporting tables for financial calculations
CREATE TABLE property_financing (
    id SERIAL PRIMARY KEY,
    property_id INT REFERENCES properties(id) ON DELETE CASCADE,
    financing_type FINANCING_TYPE NOT NULL,
    down_payment_percent DECIMAL(5, 2),
    interest_rate DECIMAL(5, 2),
    loan_term_years INT,
    points DECIMAL(5, 2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE income_records (
    id SERIAL PRIMARY KEY,
    property_id INT REFERENCES properties(id) ON DELETE CASCADE,
    category INCOME_CATEGORY NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    frequency EXPENSE_FREQUENCY NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE expense_records (
    id SERIAL PRIMARY KEY,
    property_id INT REFERENCES properties(id) ON DELETE CASCADE,
    category EXPENSE_CATEGORY NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    frequency EXPENSE_FREQUENCY NOT NULL,
    tax_deductible BOOLEAN DEFAULT FALSE,
    tax_category TAX_DEDUCTION_CATEGORY,
    start_date DATE NOT NULL,
    end_date DATE,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE roi_calculations (
    id SERIAL PRIMARY KEY,
    property_id INT REFERENCES properties(id) ON DELETE CASCADE,
    metric ROI_METRIC NOT NULL,
    value DECIMAL(10, 4) NOT NULL,
    calculation_date DATE NOT NULL DEFAULT CURRENT_DATE,
    time_period_months INT NOT NULL, -- e.g., 12 for annual
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);