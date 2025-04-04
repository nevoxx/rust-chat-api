CREATE TABLE IF NOT EXISTS `servers`
(
    `id`          char(36)     NOT NULL,
    `name`        varchar(128) NOT NULL,
    `description` text,
    `created_at`  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `servers_name_unique` (`name`)
);

CREATE TABLE IF NOT EXISTS `channels`
(
    `id`         char(36)     NOT NULL,
    `name`       varchar(255) NOT NULL,
    `sort_order` int          NOT NULL,
    `is_default` boolean      NOT NULL DEFAULT FALSE,
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `deleted_at` timestamp    NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `channels_name_unique` (`name`)
);

CREATE TABLE IF NOT EXISTS `users`
(
    `id`             char(36)     NOT NULL,
    `username`       varchar(255) NOT NULL,
    `display_name`   varchar(255) NOT NULL,
    `password`       varchar(255) NOT NULL,
    `is_system_user` boolean      NOT NULL DEFAULT FALSE,
    `created_at`     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_username_unique` (`username`),
    UNIQUE KEY `users_display_name_unique` (`display_name`)
);

CREATE TABLE IF NOT EXISTS `messages`
(
    `id`                 char(36)  NOT NULL,
    `user_id`            char(36)  NOT NULL,
    `channel_id`         char(36)  NOT NULL,
    `content`            text,
    `created_at`         timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`         timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    `deleted_at`         timestamp NULL     DEFAULT NULL,
    `deleted_by_user_id` char(36)           DEFAULT NULL,
    PRIMARY KEY (`id`),
    KEY `messages_user_id_foreign` (`user_id`),
    KEY `messages_channel_id_foreign` (`channel_id`),
    KEY `messages_deleted_by_user_id_foreign` (`deleted_by_user_id`),
    CONSTRAINT `messages_channel_id_foreign` FOREIGN KEY (`channel_id`) REFERENCES `channels` (`id`),
    CONSTRAINT `messages_deleted_by_user_id_foreign` FOREIGN KEY (`deleted_by_user_id`) REFERENCES `users` (`id`),
    CONSTRAINT `messages_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE IF NOT EXISTS `attachments`
(
    `id`         char(36)  NOT NULL,
    `type`       varchar(255)       DEFAULT NULL,
    `model_id`   varchar(255)       DEFAULT NULL,
    `model_type` varchar(255)       DEFAULT NULL,
    `mime_type`  varchar(255)       DEFAULT NULL,
    `filename`   varchar(255)       DEFAULT NULL,
    `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `attachments_message_id_foreign` (`model_id`)
);

CREATE TABLE IF NOT EXISTS `invite_codes`
(
    `id`         char(36)     NOT NULL,
    `code`       varchar(255) NOT NULL,
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `invite_codes_code_unique` (`code`)
);

CREATE TABLE IF NOT EXISTS `roles`
(
    `id`         char(36)     NOT NULL,
    `name`       varchar(255) NOT NULL,
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `roles_name_unique` (`name`)
);

CREATE TABLE IF NOT EXISTS `permissions`
(
    `id`         char(36)     NOT NULL,
    `name`       varchar(255) NOT NULL,
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY `permissions_name_unique` (`name`)
);

CREATE TABLE IF NOT EXISTS `role_permissions`
(
    `id`            int unsigned NOT NULL AUTO_INCREMENT,
    `role_id`       char(36)     NOT NULL,
    `permission_id` char(36)     NOT NULL,
    `created_at`    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `role_permissions_role_id_foreign` (`role_id`),
    KEY `role_permissions_permission_id_foreign` (`permission_id`),
    CONSTRAINT `role_permissions_permission_id_foreign` FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`),
    CONSTRAINT `role_permissions_role_id_foreign` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`)
);

CREATE TABLE IF NOT EXISTS `user_permissions`
(
    `id`            int unsigned NOT NULL AUTO_INCREMENT,
    `user_id`       char(36)     NOT NULL,
    `permission_id` char(36)     NOT NULL,
    `created_at`    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at`    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `user_permissions_user_id_foreign` (`user_id`),
    KEY `user_permissions_permission_id_foreign` (`permission_id`),
    CONSTRAINT `user_permissions_permission_id_foreign` FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`),
    CONSTRAINT `user_permissions_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);

CREATE TABLE IF NOT EXISTS `user_roles`
(
    `id`         int unsigned NOT NULL AUTO_INCREMENT,
    `user_id`    char(36)     NOT NULL,
    `role_id`    char(36)     NOT NULL,
    `created_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    KEY `user_roles_user_id_foreign` (`user_id`),
    KEY `user_roles_role_id_foreign` (`role_id`),
    CONSTRAINT `user_roles_role_id_foreign` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`),
    CONSTRAINT `user_roles_user_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
);
