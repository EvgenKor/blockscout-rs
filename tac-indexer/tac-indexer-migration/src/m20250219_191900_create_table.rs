use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE EXTENSION IF NOT EXISTS pgcrypto;

            CREATE TABLE "tac_tasks" (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                start_timestamp timestamp without time zone NOT NULL,
                end_timestamp timestamp without time zone NOT NULL,
                status TEXT CHECK (status IN ('queueing', 'processing', 'completed', 'failed')) NOT NULL
            );

            CREATE TABLE "tac_operation_ids" (
                id bytea PRIMARY KEY,
                timestamp timestamp without time zone NOT NULL
            );



            CREATE TABLE "tac_operation_stages" (
                operation_id BYTEA NOT NULL,
                stage TEXT CHECK (stage IN (
                    'evm_merkle_msg_collected', 
                    'evm_merkle_root_set', 
                    'evm_merkle_msg_executed', 
                    'tvm_merkle_msg_collected', 
                    'tvm_merkle_msg_executed'
                )) NOT NULL,
                timestamp TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                successful BOOLEAN NOT NULL DEFAULT true,
                PRIMARY KEY (operation_id, stage),
                FOREIGN KEY (operation_id) REFERENCES "tac_operation_ids"(id) ON DELETE CASCADE
            );

            COMMENT ON TABLE "tac_tasks" IS 'TAC fetcher timeline';

            COMMENT ON TABLE "tac_operation_ids" IS 'Obtained operation identifiers';

            COMMENT ON TABLE "tac_operation_stages" IS 'Contains operation stages';
        "#;
        crate::from_sql(manager, sql).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            DROP TABLE IF EXISTS tac_tasks;
            DROP TABLE IF EXISTS tac_operation_stages;
            DROP TABLE IF EXISTS tac_operation_ids;
        "#;

        crate::from_sql(manager, sql).await
    }
}
