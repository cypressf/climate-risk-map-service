-- States and counties
CREATE TABLE state (
    id TINYINT UNSIGNED NOT NULL UNIQUE,
    name VARCHAR(70) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE county (
    id SMALLINT UNSIGNED NOT NULL,
    name VARCHAR(100) NOT NULL,
    state TINYINT UNSIGNED NOT NULL,
    PRIMARY KEY (state, id),
    FOREIGN KEY(state) REFERENCES state(id)
);

-- Datasets
CREATE TABLE data_source (
    id SMALLINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(200) NOT NULL UNIQUE,
    description VARCHAR(10000) NOT NULL,
    link VARCHAR(3000) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE dataset (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    short_name VARCHAR(30) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL UNIQUE,
    description VARCHAR(2000) NOT NULL,
    units VARCHAR(30) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE county_data (
    dataset INT UNSIGNED NOT NULL,
    source SMALLINT UNSIGNED NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    state_id TINYINT UNSIGNED NOT NULL,
    county_id SMALLINT UNSIGNED NOT NULL,
    value FLOAT,
    PRIMARY KEY (
        dataset,
        source,
        start_date,
        end_date,
        state_id,
        county_id
    ),
    FOREIGN KEY (dataset) REFERENCES dataset(id),
    FOREIGN KEY (source) REFERENCES data_source(id),
    FOREIGN KEY (state_id, county_id) REFERENCES county(state, id)
);

-- Map visualizations
CREATE TABLE map_type (
    id TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE color_palette (
    id TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(30) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE scale_type (
    id TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE formatter_type (
    id TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(30) NOT NULL UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE map_visualization (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    dataset INT UNSIGNED NOT NULL,
    map_type TINYINT UNSIGNED NOT NULL,
    legend_ticks TINYINT UNSIGNED,
    should_normalize BOOLEAN NOT NULL,
    color_palette TINYINT UNSIGNED NOT NULL,
    reverse_scale BOOLEAN NOT NULL DEFAULT FALSE,
    invert_normalized BOOLEAN NOT NULL DEFAULT FALSE,
    scale_type TINYINT UNSIGNED NOT NULL,
    formatter_type TINYINT UNSIGNED NOT NULL,
    decimals TINYINT UNSIGNED NOT NULL DEFAULT 0,
    legend_formatter_type TINYINT UNSIGNED,
    legend_decimals TINYINT UNSIGNED,
    PRIMARY KEY (id),
    FOREIGN KEY (map_type) REFERENCES map_type(id),
    FOREIGN KEY (color_palette) REFERENCES color_palette(id),
    FOREIGN KEY (dataset) REFERENCES dataset(id),
    FOREIGN KEY (formatter_type) REFERENCES formatter_type(id)
);

CREATE TABLE scale_domain (
    map_visualization INT UNSIGNED NOT NULL,
    value FLOAT NOT NULL,
    FOREIGN KEY (map_visualization) REFERENCES map_visualization(id)
);

CREATE TABLE data_category (
    id TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `order` TINYINT UNSIGNED NOT NULL UNIQUE,
    name VARCHAR(20) NOT NULL,
    PRIMARY KEY (id),
    KEY (`order`)
);

CREATE TABLE map_visualization_collection (
    map_visualization INT UNSIGNED NOT NULL,
    category TINYINT UNSIGNED NOT NULL,
    `order` TINYINT UNSIGNED NOT NULL,
    PRIMARY KEY (category, map_visualization),
    KEY (`order`),
    FOREIGN KEY (map_visualization) REFERENCES map_visualization(id),
    FOREIGN KEY (category) REFERENCES data_category(id)
);