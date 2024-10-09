-- Add migration script here
create table if not exists co_sku(
    sku_code varchar(50) not null,
    name varchar(255) not null,
    description text not null,
    status char(1) not null DEFAULT '1',
    `create_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `update_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`sku_code`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

create table if not exists co_sku_log(
    id int auto_increment primary key,
    sku_code varchar(50) not null,
    content JSON not null,
    `create_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB AUTO_INCREMENT=95 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

create table if not exists co_sku_price(
    id int auto_increment primary key,
    sku_code varchar(50) not null,
    `sequence` int not null,
    `start_date_time` timestamp NOT NULL,
    `end_date_time` timestamp NOT NULL,
    price decimal(10,2) not null,
    `create_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `update_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB AUTO_INCREMENT=95 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

create table if not exists co_sku_channel(
    id int auto_increment primary key,
    sku_code varchar(50) not null,
    channel varchar(50) not null,
    status char(1) not null default '1',
    `start_date_time` timestamp NOT NULL,
    `end_date_time` timestamp NOT NULL,
    `create_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `update_date_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB AUTO_INCREMENT=95 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

create table if not exists users(
    name varchar(50) not null,
    age int not null
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;