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

CREATE TABLE `rc_qa_question` (  
    `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',  
    `sku_code` varchar(50) DEFAULT NULL COMMENT 'sku表的code',  
    `product_code` varchar(50) NOT NULL COMMENT 'product表的code',  
    `question_content` varchar(2000) NOT NULL COMMENT '问题内容',  
    `create_user_id` varchar(100) DEFAULT NULL COMMENT '创建人',  
    `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',  
    `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',  
    `status` tinyint NOT NULL DEFAULT '1' COMMENT '状态(0/1) (0-逻辑删除，1-正常)',  
    `question_code` varchar(100) DEFAULT NULL COMMENT '问题编号',  
    `creator_name` varchar(100) DEFAULT NULL COMMENT '创建人中文名',  
    `sort` bigint NOT NULL DEFAULT '0' COMMENT '问题序号',  
    `rank` int NOT NULL DEFAULT '1' COMMENT '排序',  
    PRIMARY KEY (`id`),  
    UNIQUE KEY `rc_qa_question_un` (`question_code`)
) ENGINE=InnoDB AUTO_INCREMENT=1251 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='产品问答问题表';

CREATE TABLE `rc_qa_answer` (  
    `id` bigint NOT NULL AUTO_INCREMENT COMMENT '主键',  
    `question_id` bigint DEFAULT NULL COMMENT '问题表的id',  
    `answer_content` varchar(2000) NOT NULL COMMENT '答案内容',  
    `create_user_id` varchar(100) DEFAULT NULL COMMENT '创建人',  
    `create_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',  
    `update_time` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '更新时间',  
    `status` tinyint NOT NULL DEFAULT '1' COMMENT '状态(0/1) (0-逻辑删除，1-正常)',  
    `question_code` varchar(100) DEFAULT NULL COMMENT '问题编号',  
    `creator_name` varchar(100) DEFAULT NULL COMMENT '创建人姓名',  
    PRIMARY KEY (`id`),  KEY `rc_qa_answer_question_code_IDX` (`question_code`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1301 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='产品问答答案表';